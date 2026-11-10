// Game-side glue for Iron Furnaces: the block entity, its 3-slot furnace
// container, the tick that runs the pure `runtime` rules, the tier-upgrade
// items and the `/ironfurnaces info` harness command.
//
// Injected into `crate::game` as `ironfurnaces_state`; it needs the game, so
// it is not part of the standalone library (only `runtime.rs` is).

use std::sync::Mutex;

use horizon_client::containers::{BlockEntity, ModBlockEntity, ModContainer};
use horizon_client::items::ItemStack;
use horizon_client::text::Text;
use horizon_world::nbt::{Compound, JavaString, Tag};

use crate::game::ironfurnaces::{FUEL, INPUT, OUTPUT, SLOTS, State, Tick, Tier, upgrade_target};
use crate::game::native_server::{NativeCommand, NativeServerPlayer};

/// The vanilla `furnace` class all tier blocks use as their template.
const VANILLA_FURNACE: &str = "minecraft:furnace";

/// What a queued upgrade must restore into the new tier's block entity after
/// the block swap: the old contents and the smelting state.
struct Pending {
    target: &'static str,
    items: Vec<Option<ItemStack>>,
    state: State,
}

/// Upgrades waiting for their new block entity's first tick, by dimension and
/// position. The block entity write from a player hook is not authoritative,
/// so it is applied from the block entity's own tick (the `enderstorage`
/// precedent).
static PENDING: Mutex<Vec<(String, [i32; 3], Pending)>> = Mutex::new(Vec::new());

fn pending() -> std::sync::MutexGuard<'static, Vec<(String, [i32; 3], Pending)>> {
    PENDING.lock().unwrap_or_else(|poison| poison.into_inner())
}

fn int(nbt: &Compound, key: &str) -> i32 {
    match nbt.get(&JavaString::from(key)) {
        Some(Tag::Int(value)) => *value,
        _ => 0,
    }
}

fn set_int(nbt: &mut Compound, key: &str, value: i32) {
    nbt.insert(JavaString::from(key), Tag::Int(value));
}

fn read_state(entity: &ModBlockEntity) -> State {
    State {
        burn: int(&entity.nbt, "BurnTime"),
        burn_total: int(&entity.nbt, "BurnTimeTotal"),
        progress: int(&entity.nbt, "CookTime"),
        total: int(&entity.nbt, "CookTimeTotal"),
    }
}

fn write_state(entity: &mut ModBlockEntity, state: State) {
    set_int(&mut entity.nbt, "BurnTime", state.burn);
    set_int(&mut entity.nbt, "BurnTimeTotal", state.burn_total);
    set_int(&mut entity.nbt, "CookTime", state.progress);
    set_int(&mut entity.nbt, "CookTimeTotal", state.total);
}

/// The block name at `pos` (`BlockState.getBlock`).
fn block_at(game: &crate::game::Game<'_>, pos: [i32; 3]) -> String {
    game.res
        .facts()
        .get(game.world.block(pos[0], pos[1], pos[2]))
        .name
        .to_string()
}

fn mark_modified(game: &mut crate::game::Game<'_>, pos: [i32; 3]) {
    game.world.modified.insert((pos[0] >> 4, pos[2] >> 4));
}

/// `EntityBlock.newBlockEntity` for a tier block (API 15).
pub fn on_block_entity(block: &str, entity: &mut Option<(String, Compound)>) {
    if Tier::from_block(block).is_some() {
        *entity = Some((block.to_owned(), Compound::new()));
    }
}

/// `EntityBlock.getTicker` for a tier block (API 15).
pub fn on_block_entity_ticks(block: &str, ticks: &mut bool) {
    if Tier::from_block(block).is_some() {
        *ticks = true;
    }
}

/// The 27-slot shape is replaced here by the furnace's input/fuel/output
/// slots, reachable from every face, and the vanilla furnace menu (API 16/17).
pub fn on_block_entity_container(block: &str, container: &mut Option<ModContainer>) {
    if Tier::from_block(block).is_none() {
        return;
    }
    let all = vec![INPUT, FUEL, OUTPUT];
    *container = Some(ModContainer {
        slots: SLOTS,
        faces: [
            all.clone(),
            all.clone(),
            all.clone(),
            all.clone(),
            all.clone(),
            all,
        ],
        inputs: vec![INPUT, FUEL],
        outputs: vec![OUTPUT],
        menu: Some(VANILLA_FURNACE.to_owned()),
    });
}

/// One server tick of a tier furnace: apply a queued upgrade, then run the
/// pure smelting rules against the game's recipe and fuel tables (API 17).
#[allow(private_interfaces)]
pub fn on_block_entity_tick(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    entity: &mut ModBlockEntity,
) {
    if entity.items.len() < SLOTS {
        entity.items.resize(SLOTS, None);
    }
    if entity.data.len() < 4 {
        entity.data.resize(4, 0);
    }
    apply_pending(game, pos, entity);
    let Some(tier) = Tier::from_block(&entity.id) else {
        return;
    };
    let before = read_state(entity);

    let (recipe_time, result) = {
        let recipes = match &game.world_tables {
            Some(tables) => &tables.recipes,
            None => game.res.recipes(),
        };
        match entity
            .items
            .get(INPUT)
            .and_then(|slot| slot.as_ref())
            .and_then(|stack| recipes.cook("minecraft:smelting", stack))
        {
            Some(recipe) => (Some(recipe.time), Some(recipe.result.clone())),
            None => (None, None),
        }
    };

    let output_ok = match (&result, entity.items.get(OUTPUT).and_then(|s| s.as_ref())) {
        (Some(_), None) => true,
        (Some(result), Some(output)) => {
            let max = game
                .res
                .items()
                .get(&result.item)
                .map_or(64, |item| item.max_stack)
                .min(64);
            output.same_item_same_components(result) && output.count + result.count <= max
        }
        (None, _) => false,
    };
    let fuel_time = entity
        .items
        .get(FUEL)
        .and_then(|slot| slot.as_ref())
        .map_or(0, |fuel| {
            game.res
                .providers()
                .burn_time(game.res.items(), fuel, false)
                .max(0) as u32
        });
    let input_present = entity
        .items
        .get(INPUT)
        .is_some_and(|slot| slot.as_ref().is_some_and(|s| s.count > 0));

    let mut state = before;
    let step = crate::game::ironfurnaces::tick(
        &mut state,
        Tick {
            base_ticks: tier.base_ticks(),
            has_input: input_present,
            recipe_time,
            fuel_time,
            output_ok,
        },
    );

    if step.consume_fuel {
        consume_fuel(game, pos, entity);
    }
    if step.smelt {
        smelt_one(entity, result.as_ref());
    }

    write_state(entity, state);
    entity.data[0] = state.burn;
    entity.data[1] = state.burn_total;
    entity.data[2] = state.progress;
    entity.data[3] = state.total;
    if state != before || step.consume_fuel || step.smelt {
        mark_modified(game, pos);
    }
    sync_lit(game, pos, state.is_burning());
}

/// `consumeFuel`: take one item, keeping a crafting remainder (a lava bucket
/// becomes an empty bucket) and dropping any remainder the slot cannot hold.
fn consume_fuel(game: &mut crate::game::Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let Some(fuel) = entity.items.get(FUEL).and_then(|slot| slot.as_ref()) else {
        return;
    };
    let remainder = game
        .res
        .items()
        .get(&fuel.item)
        .and_then(|item| item.crafting_remainder.clone());
    let Some(slot) = entity.items.get_mut(FUEL) else {
        return;
    };
    let Some(stack) = slot else {
        return;
    };
    stack.count = stack.count.saturating_sub(1);
    if stack.count == 0 {
        *slot = remainder.map(|id| ItemStack::new(&id, 1));
    } else if let Some(id) = remainder {
        game.spawn_item(
            ItemStack::new(&id, 1),
            [
                f64::from(pos[0]) + 0.5,
                f64::from(pos[1]) + 1.0,
                f64::from(pos[2]) + 0.5,
            ],
            [0.0; 3],
            10,
        );
    }
}

/// `smelt`: one input into the result, plus the wet-sponge bucket case.
fn smelt_one(entity: &mut ModBlockEntity, result: Option<&ItemStack>) {
    let Some(result) = result else {
        return;
    };
    let wet_sponge = entity
        .items
        .get(INPUT)
        .and_then(|slot| slot.as_ref())
        .is_some_and(|stack| &*stack.item == "minecraft:wet_sponge");
    if wet_sponge
        && entity
            .items
            .get(FUEL)
            .and_then(|slot| slot.as_ref())
            .is_some_and(|stack| &*stack.item == "minecraft:bucket")
    {
        entity.items[FUEL] = Some(ItemStack::new("minecraft:water_bucket", 1));
    }
    match entity.items.get_mut(OUTPUT).and_then(|slot| slot.as_mut()) {
        Some(output) => output.count += result.count,
        None => entity.items[OUTPUT] = Some(result.clone()),
    }
    if let Some(input) = entity.items.get_mut(INPUT).and_then(|slot| slot.as_mut()) {
        input.count = input.count.saturating_sub(1);
        if input.count == 0 {
            entity.items[INPUT] = None;
        }
    }
}

/// `lit` follows `isBurning`, as `AbstractFurnaceBlockEntity` sets its block
/// state (API 18).
fn sync_lit(game: &mut crate::game::Game<'_>, pos: [i32; 3], lit: bool) {
    let current = game
        .pack
        .assets
        .registry
        .state(game.world.block(pos[0], pos[1], pos[2]))
        .and_then(|state| state.properties.get("lit"))
        .is_some_and(|value| value == "true");
    if current != lit {
        let _ = game.native_set_block_property(pos, "lit", if lit { "true" } else { "false" });
    }
}

/// Restore a queued upgrade into the new tier's block entity.
fn apply_pending(game: &mut crate::game::Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let dimension = game.dim.id().to_owned();
    let mut queue = pending();
    let Some(index) = queue
        .iter()
        .position(|(dim, at, _)| dim == &dimension && *at == pos)
    else {
        return;
    };
    let (_, _, upgrade) = queue.remove(index);
    if entity.id != upgrade.target {
        return;
    }
    entity.items = upgrade.items;
    if entity.items.len() < SLOTS {
        entity.items.resize(SLOTS, None);
    }
    write_state(entity, upgrade.state);
    mark_modified(game, pos);
}

/// `ItemUpgrade.useOn`: right-click a compatible furnace with the upgrade item
/// to replace it with the next tier, keeping contents and smelting state, and
/// consuming one item unless creative. `right_click_block` runs before the
/// furnace's own use, which would otherwise open its menu (API 21).
#[allow(private_interfaces)]
pub fn on_right_click_block(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    _face: [i32; 3],
    canceled: &mut bool,
) {
    let Some(stack) = game.held().cloned() else {
        return;
    };
    let source = block_at(game, pos);
    let Some(target) = upgrade_target(&source, &stack.item) else {
        return;
    };
    let (items, state) = capture(game, pos, &source);
    let Some(target_state) = target_state(game, pos, target) else {
        return;
    };
    let dimension = game.dim.id().to_owned();
    if !game.native_set_block(&dimension, pos, target_state, 3) {
        return;
    }
    pending().push((
        dimension,
        pos,
        Pending {
            target,
            items,
            state,
        },
    ));
    let creative = game
        .native_acting_player()
        .is_some_and(|player| player.game_mode == 1);
    if !creative {
        let _ = game.native_take_items(&stack.item, 1);
    }
    *canceled = true;
}

/// The source block's contents and smelting state. A vanilla furnace keeps
/// only its three slots (upstream drops them and starts the new tier fresh).
fn capture(
    game: &crate::game::Game<'_>,
    pos: [i32; 3],
    source: &str,
) -> (Vec<Option<ItemStack>>, State) {
    let key = (pos[0], pos[1], pos[2]);
    if Tier::from_block(source).is_some() {
        if let Some(BlockEntity::Mod(entity)) = game.world.block_entities.get(&key) {
            let mut items = entity.items.clone();
            items.resize(SLOTS, None);
            return (items, read_state(entity));
        }
    } else if source == VANILLA_FURNACE {
        if let Some(BlockEntity::Furnace(furnace)) = game.world.block_entities.get(&key) {
            return (furnace.items.to_vec(), State::default());
        }
    }
    (vec![None; SLOTS], State::default())
}

/// The target block's state with the source's `facing` and an unlit face.
fn target_state(
    game: &crate::game::Game<'_>,
    pos: [i32; 3],
    target: &str,
) -> Option<horizon_world::registry::BlockStateId> {
    let source = game.world.block(pos[0], pos[1], pos[2]);
    let mut descriptor = game.pack.assets.registry.state(source)?.clone();
    descriptor.name = target.to_owned();
    descriptor.properties.insert("lit".into(), "false".into());
    descriptor.properties.insert("jovial".into(), "0".into());
    descriptor.properties.insert("type".into(), "0".into());
    game.pack.assets.registry.resolve(&descriptor).ok()
}

/// `/ironfurnaces info <x> <y> <z>`: the harness readout (API 8).
pub fn on_register_commands(commands: &mut Vec<NativeCommand>) {
    commands.push(NativeCommand::new("ironfurnaces info <pos:block_pos>"));
}

fn slot_text(entity: &ModBlockEntity, index: usize) -> String {
    entity
        .items
        .get(index)
        .and_then(|slot| slot.as_ref())
        .map_or_else(
            || "none".to_owned(),
            |stack| format!("{}:{}", stack.item, stack.count),
        )
}

fn parse_pos(words: &[&str]) -> Option<[i32; 3]> {
    Some([
        words.first()?.parse().ok()?,
        words.get(1)?.parse().ok()?,
        words.get(2)?.parse().ok()?,
    ])
}

/// The `/ironfurnaces` command bodies (API 8).
#[allow(private_interfaces)]
pub fn on_perform_command(
    game: &mut crate::game::Game<'_>,
    _player: &NativeServerPlayer,
    command: &str,
    ran: &mut bool,
) {
    let words: Vec<&str> = command.split_whitespace().collect();
    let ["ironfurnaces", "info", x, y, z] = words.as_slice() else {
        return;
    };
    *ran = true;
    let Some(pos) = parse_pos(&[*x, *y, *z]) else {
        game.native_feedback(Text::literal("Usage: /ironfurnaces info <x> <y> <z>"));
        return;
    };
    let block = block_at(game, pos);
    let Some(tier) = Tier::from_block(&block) else {
        game.native_feedback(Text::literal("No Iron Furnace there."));
        return;
    };
    let Some(BlockEntity::Mod(entity)) = game.world.block_entities.get(&(pos[0], pos[1], pos[2]))
    else {
        game.native_feedback(Text::literal("No Iron Furnace there."));
        return;
    };
    let state = read_state(entity);
    game.native_feedback(Text::literal(format!(
        "ironfurnaces {} {} {} {} burn={}/{} cook={}/{} input={} fuel={} output={}",
        tier.label(),
        pos[0],
        pos[1],
        pos[2],
        state.burn,
        state.burn_total,
        state.progress,
        state.total,
        slot_text(entity, INPUT),
        slot_text(entity, FUEL),
        slot_text(entity, OUTPUT),
    )));
}
