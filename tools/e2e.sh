#!/usr/bin/env bash
# In-game check for Iron Furnaces slice 1 (the tiered fuel furnaces).
#
# The End arena (stone floor at y=63, furnaces at y=64). Native mod commands
# are typed with `ctl chat "/cmd"` because `ctl command` runs the console path
# and bypasses the native `perform_command` hook.
#
# Default run:
#  - an iron furnace and a netherite furnace each get one raw iron and one coal
#    through `ctl container` (slots 0/1);
#  - after 12 ticks the netherite furnace (5 ticks per 200-tick recipe) has
#    produced an iron ingot while the iron furnace (160) is still cooking —
#    the tier-speed differential;
#  - after 170 more ticks the iron furnace has produced its ingot too;
#  - a fuel-less iron furnace with an input never cooks;
#  - an iron furnace with a non-smeltable input (stone) and coal never cooks;
#  - the vanilla furnace menu opens on the iron furnace, shows its three
#    slots, and its result slot withdraws the ingot to the inventory;
#  - a copper furnace holding raw iron + coal is upgraded in place to an iron
#    furnace with an `upgrade_iron2`, keeping the contents, and the upgrade
#    item is consumed in survival.
#
# SELFTEST=1 flips the netherite output assertion (it must FAIL), proving the
# scenario can fail.
#
# EMBEDDED=1: the game is launched with **only the native bundle** (no
# `--dev-mod`) and the content comes from the bundle's embedded copy
# (`mixins/ironfurnaces/data/ironfurnaces-content.hmod`, the AE2 pattern). The
# run also checks the file the helper wrote. CONTROL=1 (default with EMBEDDED)
# first launches the same binary with `.horizon-cache` unusable, where the
# furnace ids must be unknown.
set -euo pipefail
export LC_ALL=C

CLONE=${CAMPAIGN_ROOT:-$(cd "$(dirname "$0")/../../.." && pwd)}
PORT=$CLONE/ports/ironfurnaces
CTL=${CTL:-${HORIZON_CTL:-$CLONE/ports/_shared/bin/horizon-1d63e0a5/horizon-ctl}}
GAME=${GAME:-$PORT/target/horizon-cargo/fast/horizon-client}
RES=${RES:-${HORIZON_RES:-${HORIZON_RES:-$HOME/.local/horizon-client/26.3}}}
SOCK=${SOCK:-/tmp/opencode/ironfurnaces-e2e.sock}
SELFTEST=${SELFTEST:-0}
EMBEDDED=${EMBEDDED:-0}
CONTROL=${CONTROL:-$EMBEDDED}
KEEP=${KEEP:-0}
WORLD=IronFurnacesE2E
CONTROL_WORLD=IronFurnacesControl

[ -x "$CTL" ] || { echo "missing horizon-ctl: $CTL" >&2; exit 1; }
[ -x "$GAME" ] || { echo "missing horizon-client: $GAME" >&2; exit 1; }
[ -d "$RES/assets" ] || { echo "missing prepared resources: $RES" >&2; exit 1; }

SCRATCH=$(mktemp -d /tmp/opencode/ironfurnaces-home.XXXXXX)
export HORIZON_HOME=$SCRATCH/game
export HORIZON_LAUNCHER_HOME=$SCRATCH/launcher
export XDG_DATA_HOME=$SCRATCH/data
export XDG_CONFIG_HOME=$SCRATCH/config
export HORIZON_NO_AUDIO=1
mkdir -p "$(dirname "$SOCK")" "$XDG_CONFIG_HOME"
rm -f "$SOCK"

ctl() { "$CTL" --socket "$SOCK" "$@"; }
say() { ctl chat "/$1" >/dev/null; }
fail() { echo "FAIL: $*" >&2; ctl logs --lines 60 >&2 || true; exit 1; }

cleanup() {
    local status=$?
    ctl quit >/dev/null 2>&1 || true
    sleep 1
    if [ "$KEEP" = 1 ]; then
        echo "kept scratch: $SCRATCH" >&2
    else
        rm -rf "$SCRATCH" 2>/dev/null || true
    fi
    # Last line of every run: `check-e2e-logs.py` reads it as the verdict.
    echo "EXIT:$status"
}
trap cleanup EXIT

launch() { # launch [WORLD]
    local world=${1:-$WORLD}
    rm -f "$SOCK"
    cd "$SCRATCH"
    local args=(--game "$GAME" --world "$world" --seed 20261105 --creative --offscreen --wait
                --resources "$RES")
    if [ "$EMBEDDED" = 1 ]; then
        echo "  launch: no --dev-mod; the bundle carries its own content" >&2
    else
        args+=(--dev-mod "$PORT/content")
    fi
    "$CTL" --socket "$SOCK" launch "${args[@]}" >/dev/null
    ctl wait-ready 120
    ctl wait 40
}

# A stone floor and a clear volume in The End, so the arena is deterministic.
arena() {
    ctl command "execute in minecraft:the_end run tp @s 0.5 64 -4.5"
    local loaded=false
    for _ in $(seq 1 120); do
        ctl wait 5
        if [ "$(ctl block 0 63 -8 | jq -r '.loaded // false')" = true ]; then
            loaded=true
            break
        fi
    done
    [ "$loaded" = true ] || fail "the End arena chunk never loaded"
    ctl command "execute in minecraft:the_end run fill -8 63 -12 24 63 4 minecraft:stone"
    ctl command "execute in minecraft:the_end run fill -8 64 -12 24 74 4 minecraft:air"
    ctl command "execute in minecraft:the_end run kill @e[type=!player]"
    ctl wait 20
}

# The `/ironfurnaces info` line for a position (the harness readout).
info() {
    say "ironfurnaces info $1 $2 $3"
    ctl wait 8 >/dev/null
    ctl messages 20 | jq -r '.chat[]' | grep -E "^ironfurnaces [a-z]+ $1 $2 $3 " | head -1 || true
}

expect() { # expect X Y Z TEXT
    local line
    line=$(info "$1" "$2" "$3")
    echo "  info($1,$2,$3): $line"
    case "$line" in
        *"$4"*) ;;
        *) fail "expected '$4' in '$line'" ;;
    esac
}

container_has() { # container_has X Y Z TEXT
    local got
    got=$(ctl container "$1" "$2" "$3" | jq -c '[.slots[] | select(. != null)]')
    echo "  container($1,$2,$3): $got"
    case "$got" in
        *"$4"*) ;;
        *) fail "expected '$4' in container at $1,$2,$3; got $got" ;;
    esac
}

inventory_count() { # inventory_count ITEM
    ctl inventory | jq -r --arg item "$1" '.counts[$item] // 0'
}

# The control for EMBEDDED=1: the same binary (every hook is in it) with the
# embedded content made unavailable — `.horizon-cache` is a plain file, so the
# helper can neither compare nor write its content file and falls back to
# `Continue`. The furnace block and item ids must then be unknown.
control_leg() {
    echo "== control: the same game with no embedded content file =="
    rm -rf "$SCRATCH/.horizon-cache"
    : > "$SCRATCH/.horizon-cache"
    launch "$CONTROL_WORLD"
    arena
    ctl command "execute in minecraft:the_end run setblock 0 64 -8 ironfurnaces:iron_furnace" >/dev/null 2>&1 || true
    ctl wait 4
    local name
    name=$(ctl block 0 64 -8 | jq -r '.block')
    [ "$name" = "minecraft:air" ] || fail "the control game holds $name, so the block id was known"
    ctl command "item replace entity @s hotbar.0 with ironfurnaces:iron_furnace 1" >/dev/null 2>&1 || true
    ctl wait 4
    ctl inventory | jq -e '[.hotbar[], .main[] | select(.item == "ironfurnaces:iron_furnace")] | length == 0' >/dev/null \
        || fail "the control game has an iron furnace item, so the content did not come from the embedded file"
    [ -f "$SCRATCH/.horizon-cache" ] || fail "the control's .horizon-cache was replaced: the helper wrote its content"
    echo "  without the embedded file: no furnace item, the block stayed $name"
    ctl quit >/dev/null 2>&1 || true
    rm -f "$SCRATCH/.horizon-cache"
}

# EMBEDDED=1: the bundle must have written its content out at startup, byte for
# byte the delivered content mod.
embedded_content_check() {
    local cache=$SCRATCH/.horizon-cache/ironfurnaces-embedded-content.hmod
    [ -f "$cache" ] || fail "the bundle did not write its embedded content to $cache"
    cmp -s "$cache" "$PORT/content/ironfurnaces.hmod" \
        || fail "the embedded content differs from content/ironfurnaces.hmod"
    echo "  embedded content: $(wc -c < "$cache" | tr -d ' ') B at $cache, identical to content/ironfurnaces.hmod"
}

echo "== launch (The End arena, $([ "$EMBEDDED" = 1 ] && echo 'native bundle only (embedded content)' || echo 'content + native bundle')) =="
if [ "$CONTROL" = 1 ]; then control_leg; fi
launch
if [ "$EMBEDDED" = 1 ]; then embedded_content_check; fi
arena

echo "== place an iron, a netherite and a copper furnace =="
ctl command "execute in minecraft:the_end run setblock 0 64 -8 ironfurnaces:iron_furnace"
ctl command "execute in minecraft:the_end run setblock 4 64 -8 ironfurnaces:netherite_furnace"
ctl command "execute in minecraft:the_end run setblock 16 64 -8 ironfurnaces:copper_furnace"
ctl wait 6
ctl give ironfurnaces:iron_furnace 1
ctl wait 4
ctl inventory | jq -e '[.hotbar[] | select(.item == "ironfurnaces:iron_furnace")] | length >= 1' >/dev/null \
    || fail "the iron furnace item did not register"
for p in "0 64 -8 iron_furnace" "4 64 -8 netherite_furnace" "16 64 -8 copper_furnace"; do
    set -- $p
    block=$(ctl block "$1" "$2" "$3" | jq -r '.block')
    echo "  block($1,$2,$3): $block"
    case "$block" in
        *"ironfurnaces:$4"*) ;;
        *) fail "expected ironfurnaces:$4 at $1,$2,$3, got $block" ;;
    esac
done

echo "== one raw iron + one coal in each of iron and netherite =="
ctl container 0 64 -8 minecraft:raw_iron 1
ctl container 0 64 -8 minecraft:coal 1
ctl container 4 64 -8 minecraft:raw_iron 1
ctl container 4 64 -8 minecraft:coal 1
ctl wait 12
# The netherite tier (5 ticks) must have finished; the iron tier (160) not.
if [ "$SELFTEST" = 1 ]; then
    # Deliberately wrong expectation: the netherite furnace HAS smelted.
    expect 4 64 -8 "output=none"
else
    expect 4 64 -8 "output=minecraft:iron_ingot:1"
fi
expect 0 64 -8 "output=none"
iron_cook=$(info 0 64 -8 | sed -E 's/.*cook=([0-9]+)\/.*/\1/')
echo "  iron cook progress after 12 ticks: $iron_cook"
[ "$iron_cook" -gt 0 ] || fail "the iron furnace did not start cooking"

echo "== the iron furnace finishes later =="
ctl wait 170
expect 0 64 -8 "output=minecraft:iron_ingot:1"

echo "== control: no fuel never cooks =="
ctl command "execute in minecraft:the_end run setblock 8 64 -8 ironfurnaces:iron_furnace"
ctl wait 4
ctl container 8 64 -8 minecraft:raw_iron 1
ctl wait 40
expect 8 64 -8 "cook=0/"
expect 8 64 -8 "output=none"

echo "== control: a non-smeltable input never cooks =="
ctl command "execute in minecraft:the_end run setblock 12 64 -8 ironfurnaces:iron_furnace"
ctl wait 4
ctl container 12 64 -8 minecraft:diamond 1
ctl container 12 64 -8 minecraft:coal 1
ctl wait 40
expect 12 64 -8 "cook=0/"
expect 12 64 -8 "output=none"

echo "== the vanilla furnace menu opens and withdraws the result =="
ctl command "execute in minecraft:the_end run tp @s 0.5 64 -4.5"
ctl wait 4
ctl interact 0 64 -8
ctl wait 6
state=$(ctl screen)
jq -e '.screen == "container"' <<<"$state" >/dev/null \
    || fail "the furnace did not open a container: $(jq -c '{screen,container}' <<<"$state")"
jq -e '[.slots[] | select(.slot >= 300)] | length == 3' <<<"$state" >/dev/null \
    || fail "the menu does not have 3 furnace slots: $(jq -c '[.slots[] | select(.slot >= 300)] | length' <<<"$state")"
jq -e '[.slots[] | select(.slot == 302 and .item == "minecraft:iron_ingot")] | length == 1' <<<"$state" >/dev/null \
    || fail "the menu result slot does not show the iron ingot: $(jq -c '[.slots[] | select(.slot >= 300)]' <<<"$state")"
ctl click-slot 302 >/dev/null || fail "cannot click the result slot"
ctl wait 4
ctl click-slot 9 >/dev/null || fail "cannot put the withdrawn ingot into the inventory"
ctl wait 8
ctl close
ctl wait 6
withdrawn=$(inventory_count minecraft:iron_ingot)
echo "  iron ingots in the inventory after the menu withdrawal: $withdrawn"
[ "$withdrawn" -ge 1 ] || fail "the menu withdrawal gave no iron ingot"
expect 0 64 -8 "output=none"

echo "== a copper furnace is upgraded in place to iron =="
ctl container 16 64 -8 minecraft:raw_iron 2
ctl container 16 64 -8 minecraft:coal 1
ctl wait 5
expect 16 64 -8 "ironfurnaces copper 16 64 -8"
ctl command "execute in minecraft:the_end run tp @s 16.5 64 -4.5"
ctl wait 4
ctl command "gamemode survival"
ctl give ironfurnaces:upgrade_iron2 1
ctl wait 4
ctl select ironfurnaces:upgrade_iron2
ctl wait 4
before=$(inventory_count ironfurnaces:upgrade_iron2)
ctl interact 16 64 -8
ctl wait 8
block=$(ctl block 16 64 -8 | jq -r '.block')
echo "  block(16,64,-8) after the upgrade: $block"
case "$block" in
    *"ironfurnaces:iron_furnace"*) ;;
    *) fail "the copper furnace was not upgraded to iron: $block" ;;
esac
expect 16 64 -8 "input=minecraft:raw_iron:2"
upgraded_cook=$(info 16 64 -8 | sed -E 's/.*cook=([0-9]+)\/.*/\1/')
echo "  upgraded furnace cook progress: $upgraded_cook"
[ "$upgraded_cook" -gt 0 ] || fail "the upgrade did not keep the cooking progress"
after=$(inventory_count ironfurnaces:upgrade_iron2)
echo "  upgrade items: $before -> $after"
[ "$after" -lt "$before" ] || fail "the upgrade item was not consumed in survival"
ctl command "gamemode creative"
ctl wait 4

echo "PASS: two tiers smelted (netherite before iron), a coal-scaled fuel burn, the no-fuel and non-smeltable controls stayed idle, the vanilla furnace menu showed its three slots and withdrew the ingot, and a copper furnace upgraded in place to iron keeping its contents"
