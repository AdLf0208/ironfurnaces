# ironfurnaces (Horizon port)

A Horizon port of [ironfurnaces](Qelifern/IronFurnaces) — Apache-2.0.

**Status: first slices.** What is below is what the port actually does today.

## Implemented so far

- The eight furnace tiers with vanilla-only recipes in the jar: **copper, iron,
- gold, diamond, emerald, obsidian, crystal, netherite**. Smelting with fuel,
- the tier speed, the coal-scaled burn time, the vanilla furnace menu (input /
- fuel / output + the flame and arrow data slots), the `lit` state and the
- tier-upgrade items. The jar also ships silver, allthemodium, vibranium,
- unobtainium and the rainbow furnace; they need other mods' materials and are
- deferred (their assets stay in the package, inert).
- | Area | Upstream | This port |

## Silver and the modded tiers

Four more furnaces come in 0.2.0, each with the upstream speed and upgrades:

| Furnace | Ticks per smelt | Crafted from | Upgrades |
| --- | --- | --- | --- |
| Silver | 140 | `c:ingots/silver` on an iron or copper furnace | copper → silver, iron → silver, silver → gold |
| Allthemodium | 5 | allthemodium or stellarite ingots and blocks on a netherite furnace | netherite → allthemodium |
| Vibranium | 3 | vibranium or infernium on an allthemodium furnace | allthemodium → vibranium |
| Unobtainium | 1 | unobtainium or voidium on a vibranium furnace | vibranium → unobtainium |

They need another mod's metals. Silver comes from any mod that fills
`c:ingots/silver`, such as [Vast Ores](https://github.com/voltzz900/vast-ores).
The modded tiers take Allthemodium's metals through `c:` tags, or
[Vast Depths](https://github.com/voltzz900/vast-ores)' stellarite, infernium and
voidium, which fill the same three steps; the `ironfurnaces:<tier>_ingots` and
`ironfurnaces:<tier>_corners` item tags list both. With neither mod, the recipes
are skipped and the furnaces are only reachable by command.

The jar gates these recipes with `neoforge:tag_empty`; Horizon leaves a file
with an unknown condition out, so the port drops the gate and relies on a
recipe with an empty tag being skipped when it loads.

## Not yet ported

- Eight tiers only.** Silver, allthemodium, vibranium, unobtainium and the
- rainbow furnace need other mods' materials; their assets are inert.
- Augments, factory, generator, wireless heater, settings GUI, auto IO,
- redstone modes and XP** are deferred. The furnace branch only.
- Light 13 when lit** (the `minecraft:furnace` template) vs upstream 14.
- Vanilla-furnace → copper/iron upgrades** are implemented (the source is
- read through the vanilla furnace block entity and its 3 slots restored) but
- the e2e exercises only copper→iron.

## Install

Build the bundle with a native-mod capable launcher, or add the file from the
registry once it is listed. Pinned to Horizon `1d63e0a` and horizon-mixins
`5a64af93` for game 26.3.

## Build from source

```sh
cargo test
python3 tools/horizon.py check --horizon <horizon checkout>
python3 tools/horizon.py build --horizon <horizon checkout>
python3 tools/package.py
```

## Credits and licence

Original mod: Qelifern/IronFurnaces. Port: AdLf0208. Licence: Apache-2.0 — see [LICENSE](LICENSE);
the original notices are preserved.
