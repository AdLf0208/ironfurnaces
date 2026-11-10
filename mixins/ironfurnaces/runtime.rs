// Iron Furnaces: the pure smelting/fuel/upgrade rules of the tiered furnaces.
//
// This file is both the standalone Cargo library and the module injected as
// `crate::game::ironfurnaces`; it must stay free of game types so the tests
// run without staging. The support module does the recipe/item/NBT glue.

/// The three slots the furnace menu shows: input, fuel, output.
pub const INPUT: usize = 0;
pub const FUEL: usize = 1;
pub const OUTPUT: usize = 2;
/// Slots in a furnace.
pub const SLOTS: usize = 3;

/// The twelve tiers ported here. Silver and the three modded tiers
/// (allthemodium, vibranium, unobtainium) are crafted from other mods'
/// metals; the rainbow furnace is out of scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    Copper,
    Iron,
    Gold,
    Diamond,
    Emerald,
    Obsidian,
    Crystal,
    Netherite,
    Silver,
    Allthemodium,
    Vibranium,
    Unobtainium,
}

impl Tier {
    pub const ALL: [Tier; 12] = [
        Tier::Copper,
        Tier::Iron,
        Tier::Gold,
        Tier::Diamond,
        Tier::Emerald,
        Tier::Obsidian,
        Tier::Crystal,
        Tier::Netherite,
        Tier::Silver,
        Tier::Allthemodium,
        Tier::Vibranium,
        Tier::Unobtainium,
    ];

    /// The tier of a block name (`ironfurnaces:iron_furnace`).
    pub fn from_block(block: &str) -> Option<Tier> {
        Tier::ALL.into_iter().find(|tier| tier.block_id() == block)
    }

    /// The tier's block id.
    pub fn block_id(self) -> &'static str {
        match self {
            Tier::Copper => "ironfurnaces:copper_furnace",
            Tier::Iron => "ironfurnaces:iron_furnace",
            Tier::Gold => "ironfurnaces:gold_furnace",
            Tier::Diamond => "ironfurnaces:diamond_furnace",
            Tier::Emerald => "ironfurnaces:emerald_furnace",
            Tier::Obsidian => "ironfurnaces:obsidian_furnace",
            Tier::Crystal => "ironfurnaces:crystal_furnace",
            Tier::Netherite => "ironfurnaces:netherite_furnace",
            Tier::Silver => "ironfurnaces:silver_furnace",
            Tier::Allthemodium => "ironfurnaces:allthemodium_furnace",
            Tier::Vibranium => "ironfurnaces:vibranium_furnace",
            Tier::Unobtainium => "ironfurnaces:unobtainium_furnace",
        }
    }

    /// Ticks to smelt a 200-tick recipe, as `Config.<tier>FurnaceSpeed` ships.
    pub fn base_ticks(self) -> u32 {
        match self {
            Tier::Copper => 180,
            Tier::Iron => 160,
            Tier::Gold => 120,
            Tier::Diamond => 80,
            Tier::Emerald => 40,
            Tier::Obsidian => 20,
            Tier::Crystal => 40,
            Tier::Netherite => 5,
            Tier::Silver => 140,
            Tier::Allthemodium => 5,
            Tier::Vibranium => 3,
            Tier::Unobtainium => 1,
        }
    }

    /// The lowercase name used in commands and the menu title.
    pub fn label(self) -> &'static str {
        match self {
            Tier::Copper => "copper",
            Tier::Iron => "iron",
            Tier::Gold => "gold",
            Tier::Diamond => "diamond",
            Tier::Emerald => "emerald",
            Tier::Obsidian => "obsidian",
            Tier::Crystal => "crystal",
            Tier::Netherite => "netherite",
            Tier::Silver => "silver",
            Tier::Allthemodium => "allthemodium",
            Tier::Vibranium => "vibranium",
            Tier::Unobtainium => "unobtainium",
        }
    }
}

/// The upgrade item that turns one furnace into another.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Upgrade {
    pub item: &'static str,
    pub from: &'static str,
    pub to: &'static str,
}

/// Every upgrade whose two ends are ported here, in the jar's registration
/// order. `minecraft:furnace` is the vanilla furnace.
pub const UPGRADES: [Upgrade; 16] = [
    Upgrade {
        item: "ironfurnaces:upgrade_iron",
        from: "minecraft:furnace",
        to: "ironfurnaces:iron_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_gold",
        from: "ironfurnaces:iron_furnace",
        to: "ironfurnaces:gold_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_diamond",
        from: "ironfurnaces:gold_furnace",
        to: "ironfurnaces:diamond_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_emerald",
        from: "ironfurnaces:diamond_furnace",
        to: "ironfurnaces:emerald_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_obsidian",
        from: "ironfurnaces:emerald_furnace",
        to: "ironfurnaces:obsidian_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_crystal",
        from: "ironfurnaces:diamond_furnace",
        to: "ironfurnaces:crystal_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_netherite",
        from: "ironfurnaces:obsidian_furnace",
        to: "ironfurnaces:netherite_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_copper",
        from: "minecraft:furnace",
        to: "ironfurnaces:copper_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_obsidian2",
        from: "ironfurnaces:crystal_furnace",
        to: "ironfurnaces:obsidian_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_iron2",
        from: "ironfurnaces:copper_furnace",
        to: "ironfurnaces:iron_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_silver",
        from: "ironfurnaces:copper_furnace",
        to: "ironfurnaces:silver_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_silver2",
        from: "ironfurnaces:iron_furnace",
        to: "ironfurnaces:silver_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_gold2",
        from: "ironfurnaces:silver_furnace",
        to: "ironfurnaces:gold_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_allthemodium",
        from: "ironfurnaces:netherite_furnace",
        to: "ironfurnaces:allthemodium_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_vibranium",
        from: "ironfurnaces:allthemodium_furnace",
        to: "ironfurnaces:vibranium_furnace",
    },
    Upgrade {
        item: "ironfurnaces:upgrade_unobtainium",
        from: "ironfurnaces:vibranium_furnace",
        to: "ironfurnaces:unobtainium_furnace",
    },
];

/// The block an upgrade item turns `source` into, or none when the item does
/// not fit that block.
pub fn upgrade_target(source: &str, held_item: &str) -> Option<&'static str> {
    UPGRADES
        .iter()
        .find(|upgrade| upgrade.item == held_item && upgrade.from == source)
        .map(|upgrade| upgrade.to)
}

/// `getCookTime`'s speed for a recipe: the tier's 200-tick base scaled to the
/// recipe's own time, at least one tick (`regular / (200.0 / recipe_time)`).
pub fn cook_ticks(base_ticks: u32, recipe_time: u32) -> u32 {
    // Upstream computes this in `f64`, then casts (truncating); keep that so a
    // value like 2.9999999999999996 stays 2, as Java's `(int)` does.
    let ticks = f64::from(base_ticks) / (200.0 / f64::from(recipe_time));
    ticks.max(1.0) as u32
}

/// `furnaceBurnTime = fuel_time * getCookTime() / 200`.
pub fn fuel_burn_ticks(fuel_time: u32, cook_ticks: u32) -> u32 {
    ((u64::from(fuel_time) * u64::from(cook_ticks)) / 200) as u32
}

/// The saved furnace state (`BurnTime`, `BurnTimeTotal`, `CookTime`,
/// `CookTimeTotal`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct State {
    pub burn: i32,
    pub burn_total: i32,
    pub progress: i32,
    pub total: i32,
}

impl State {
    pub fn is_burning(self) -> bool {
        self.burn > 0
    }
}

/// What one tick asks the glue to do, besides the state change.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Step {
    /// A fuel item is consumed this tick (the slot must be decremented).
    pub consume_fuel: bool,
    /// The input smelts this tick (one item consumed into the output).
    pub smelt: bool,
}

/// One tick's inputs, as the glue read them from the world.
#[derive(Clone, Copy, Debug)]
pub struct Tick {
    /// The tier's 200-tick base.
    pub base_ticks: u32,
    /// Whether the input slot holds an item.
    pub has_input: bool,
    /// The matched recipe's own cooking time, `None` without a recipe.
    pub recipe_time: Option<u32>,
    /// Vanilla burn time of the fuel slot's item, 0 without fuel.
    pub fuel_time: u32,
    /// The output can take one more result (`canSmelt`'s output test).
    pub output_ok: bool,
}

/// `AbstractFurnaceBlockEntity.serverTick`'s furnace branch.
pub fn tick(state: &mut State, input: Tick) -> Step {
    let mut step = Step::default();
    if state.burn > 0 {
        state.burn -= 1;
    }
    // `if (totalCookTime != getCookTime()) totalCookTime = getCookTime();`
    if let Some(recipe_time) = input.recipe_time {
        state.total = cook_ticks(input.base_ticks, recipe_time) as i32;
    }
    let burning = state.burn > 0;
    let valid = input.has_input && input.recipe_time.is_some() && input.output_ok;
    if burning || (input.fuel_time > 0 && input.has_input) {
        if !burning && valid {
            let cook_time = state.total.max(1) as u32;
            let burn = fuel_burn_ticks(input.fuel_time, cook_time);
            if burn > 0 {
                state.burn = burn as i32;
                state.burn_total = burn as i32;
                step.consume_fuel = true;
            }
        }
        if state.burn > 0 && valid {
            state.progress += 1;
            if state.progress >= state.total.max(1) {
                state.progress = 0;
                if let Some(recipe_time) = input.recipe_time {
                    state.total = cook_ticks(input.base_ticks, recipe_time) as i32;
                }
                step.smelt = true;
            }
        } else {
            state.progress = 0;
        }
    } else if state.burn <= 0 && state.progress > 0 {
        state.progress = (state.progress - 2).clamp(0, state.total.max(0));
    }
    step
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tick_for(state: &mut State, base: u32, recipe: Option<u32>, fuel: u32) -> Step {
        tick(
            state,
            Tick {
                base_ticks: base,
                has_input: true,
                recipe_time: recipe,
                fuel_time: fuel,
                output_ok: true,
            },
        )
    }

    #[test]
    fn tier_table_matches_the_upstream_config() {
        assert_eq!(Tier::Copper.base_ticks(), 180);
        assert_eq!(Tier::Iron.base_ticks(), 160);
        assert_eq!(Tier::Gold.base_ticks(), 120);
        assert_eq!(Tier::Diamond.base_ticks(), 80);
        assert_eq!(Tier::Emerald.base_ticks(), 40);
        assert_eq!(Tier::Obsidian.base_ticks(), 20);
        assert_eq!(Tier::Crystal.base_ticks(), 40);
        assert_eq!(Tier::Netherite.base_ticks(), 5);
        assert_eq!(Tier::Silver.base_ticks(), 140);
        assert_eq!(Tier::Allthemodium.base_ticks(), 5);
        assert_eq!(Tier::Vibranium.base_ticks(), 3);
        assert_eq!(Tier::Unobtainium.base_ticks(), 1);
    }

    #[test]
    fn block_names_map_to_tiers() {
        assert_eq!(
            Tier::from_block("ironfurnaces:iron_furnace"),
            Some(Tier::Iron)
        );
        assert_eq!(
            Tier::from_block("ironfurnaces:netherite_furnace"),
            Some(Tier::Netherite)
        );
        assert_eq!(Tier::from_block("minecraft:furnace"), None);
        assert_eq!(
            Tier::from_block("ironfurnaces:silver_furnace"),
            Some(Tier::Silver)
        );
        assert_eq!(
            Tier::from_block("ironfurnaces:unobtainium_furnace"),
            Some(Tier::Unobtainium)
        );
        assert_eq!(Tier::from_block("ironfurnaces:million_furnace"), None);
    }

    #[test]
    fn cook_time_scales_with_the_recipe_time() {
        // A normal 200-tick smelt takes the tier's base.
        assert_eq!(cook_ticks(160, 200), 160);
        // A 100-tick recipe is half.
        assert_eq!(cook_ticks(160, 100), 80);
        // The result never drops below one tick.
        assert_eq!(cook_ticks(5, 1), 1);
        assert_eq!(cook_ticks(180, 200), 180);
        assert_eq!(cook_ticks(5, 200), 5);
    }

    #[test]
    fn fuel_burn_scales_with_the_cook_time() {
        // Coal (1600) in an iron furnace (160 ticks): 1600*160/200 = 1280.
        assert_eq!(fuel_burn_ticks(1600, 160), 1280);
        // Netherite (5): 1600*5/200 = 40.
        assert_eq!(fuel_burn_ticks(1600, 5), 40);
        // Integer truncation, as Java's int division.
        assert_eq!(fuel_burn_ticks(1600, 3), 24);
    }

    #[test]
    fn lights_with_fuel_and_a_valid_recipe() {
        let mut state = State::default();
        let step = tick_for(&mut state, 160, Some(200), 1600);
        assert!(step.consume_fuel, "the fuel item is consumed to light");
        assert!(!step.smelt);
        assert_eq!(state.burn, 1280);
        assert_eq!(state.total, 160);
        // The lighting tick also advances the cooking counter, as upstream.
        assert_eq!(state.progress, 1);
    }

    #[test]
    fn smelts_when_the_cooking_timer_reaches_the_total() {
        let mut state = State {
            burn: 2,
            burn_total: 1280,
            progress: 159,
            total: 160,
        };
        let step = tick_for(&mut state, 160, Some(200), 1600);
        assert!(step.smelt);
        assert!(!step.consume_fuel);
        assert_eq!(state.burn, 1);
        assert_eq!(state.progress, 0);
        assert_eq!(state.total, 160);
    }

    #[test]
    fn no_fuel_does_not_light() {
        let mut state = State::default();
        let step = tick_for(&mut state, 160, Some(200), 0);
        assert!(!step.consume_fuel);
        assert!(!step.smelt);
        assert!(!state.is_burning());
        assert_eq!(state.progress, 0);
    }

    #[test]
    fn no_recipe_resets_the_progress() {
        let mut state = State {
            burn: 100,
            burn_total: 1280,
            progress: 40,
            total: 160,
        };
        let step = tick_for(&mut state, 160, None, 1600);
        assert!(!step.smelt);
        assert_eq!(state.progress, 0);
        assert_eq!(state.burn, 99);
    }

    #[test]
    fn a_blocked_output_resets_the_progress() {
        let mut state = State {
            burn: 100,
            burn_total: 1280,
            progress: 40,
            total: 160,
        };
        let step = tick(
            &mut state,
            Tick {
                base_ticks: 160,
                has_input: true,
                recipe_time: Some(200),
                fuel_time: 1600,
                output_ok: false,
            },
        );
        assert!(!step.smelt);
        assert_eq!(state.progress, 0);
    }

    #[test]
    fn a_stopped_fire_cools_the_progress_by_two() {
        let mut state = State {
            burn: 0,
            burn_total: 0,
            progress: 9,
            total: 160,
        };
        let step = tick(
            &mut state,
            Tick {
                base_ticks: 160,
                has_input: false,
                recipe_time: None,
                fuel_time: 0,
                output_ok: true,
            },
        );
        assert!(!step.smelt);
        assert_eq!(state.progress, 7);
    }

    /// Run one input item (with the recipe and a coal) until it smelts;
    /// returns the tick it smelted on.
    fn ticks_to_first_smelt(tier: Tier) -> u32 {
        let mut state = State::default();
        for tick_no in 1..=2000 {
            let step = tick(
                &mut state,
                Tick {
                    base_ticks: tier.base_ticks(),
                    has_input: true,
                    recipe_time: Some(200),
                    fuel_time: 1600,
                    output_ok: true,
                },
            );
            if step.smelt {
                return tick_no;
            }
        }
        panic!("{tier:?} never smelted");
    }

    #[test]
    fn a_faster_tier_smelts_sooner() {
        assert_eq!(ticks_to_first_smelt(Tier::Iron), 160);
        assert_eq!(ticks_to_first_smelt(Tier::Gold), 120);
        assert_eq!(ticks_to_first_smelt(Tier::Diamond), 80);
        assert_eq!(ticks_to_first_smelt(Tier::Netherite), 5);
    }

    /// A furnace with `inputs` items and one coal: how many ticks until they
    /// are all smelted (1 item per smelt).
    fn ticks_to_smelt_all(tier: Tier, inputs: u32) -> u32 {
        let mut state = State::default();
        let mut left = inputs;
        for tick_no in 1..=100_000 {
            let step = tick(
                &mut state,
                Tick {
                    base_ticks: tier.base_ticks(),
                    has_input: left > 0,
                    recipe_time: Some(200),
                    fuel_time: 1600,
                    output_ok: true,
                },
            );
            if step.smelt {
                left -= 1;
                if left == 0 {
                    return tick_no;
                }
            }
        }
        panic!("{tier:?} never finished {inputs} items");
    }

    #[test]
    fn one_coal_smelts_eight_items_on_every_tier() {
        // The burn time scales with the cook time, so fuel efficiency is the
        // same across tiers: 1600/200 = 8 items.
        assert_eq!(ticks_to_smelt_all(Tier::Iron, 8), 160 * 8);
        assert_eq!(ticks_to_smelt_all(Tier::Netherite, 8), 5 * 8);
    }

    #[test]
    fn upgrades_apply_only_to_their_source_block() {
        assert_eq!(
            upgrade_target("ironfurnaces:iron_furnace", "ironfurnaces:upgrade_gold"),
            Some("ironfurnaces:gold_furnace")
        );
        assert_eq!(
            upgrade_target("minecraft:furnace", "ironfurnaces:upgrade_iron"),
            Some("ironfurnaces:iron_furnace")
        );
        assert_eq!(
            upgrade_target("ironfurnaces:copper_furnace", "ironfurnaces:upgrade_iron2"),
            Some("ironfurnaces:iron_furnace")
        );
        assert_eq!(
            upgrade_target(
                "ironfurnaces:crystal_furnace",
                "ironfurnaces:upgrade_obsidian2"
            ),
            Some("ironfurnaces:obsidian_furnace")
        );
        // A wrong source does nothing.
        assert_eq!(
            upgrade_target("ironfurnaces:gold_furnace", "ironfurnaces:upgrade_gold"),
            None
        );
        // A non-upgrade item does nothing.
        assert_eq!(
            upgrade_target("ironfurnaces:iron_furnace", "minecraft:diamond"),
            None
        );
        // Silver sits between copper/iron and gold.
        assert_eq!(
            upgrade_target("ironfurnaces:copper_furnace", "ironfurnaces:upgrade_silver"),
            Some("ironfurnaces:silver_furnace")
        );
        assert_eq!(
            upgrade_target("ironfurnaces:iron_furnace", "ironfurnaces:upgrade_silver"),
            None
        );
        assert_eq!(
            upgrade_target("ironfurnaces:silver_furnace", "ironfurnaces:upgrade_gold2"),
            Some("ironfurnaces:gold_furnace")
        );
        // The modded tiers chain on from netherite.
        assert_eq!(
            upgrade_target(
                "ironfurnaces:netherite_furnace",
                "ironfurnaces:upgrade_allthemodium"
            ),
            Some("ironfurnaces:allthemodium_furnace")
        );
        assert_eq!(
            upgrade_target(
                "ironfurnaces:vibranium_furnace",
                "ironfurnaces:upgrade_unobtainium"
            ),
            Some("ironfurnaces:unobtainium_furnace")
        );
    }

    #[test]
    fn every_tier_has_a_block_and_item_id() {
        for tier in Tier::ALL {
            assert!(tier.block_id().starts_with("ironfurnaces:"));
            assert!(tier.block_id().ends_with("_furnace"));
            assert!(tier.base_ticks() >= 1);
        }
    }
}
