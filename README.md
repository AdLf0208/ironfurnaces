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
