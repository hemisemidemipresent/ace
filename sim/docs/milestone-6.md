# Milestone 6 — The Monkey Ace and its attacks

Builds on [milestone 4](./milestone-4.md)'s projectile seams. Vocabulary lives in [GLOSSARY.md](../GLOSSARY.md); the reasoning is in the [ADRs](./adr/), especially ADR 0005.

## Scope

- A Monkey Ace that fires projectiles.
- Two attacks: a radial dart volley and two seeking missiles.
- Projectile movement, lifespan, and out-of-bounds removal.

`targeting` and `reverse` are stored for future use; movement is a shell.

## Ace

```rust
pub struct Ace {
    pub center: Vec2,          // where the ace is placed on the map
    pub position: Vec2,        // movement offset relative to center
    pub heading: f32,          // rotates the attack offsets and dart ring
    pub phase: f32,            // internal movement clock
    pub targeting: Targeting,  // Circle | FigureInfinite | FigureEight
    pub reverse: bool,         // default false
    pub attacks: AceAttacks,   // dart + missile timers
}
```

- `center` is fixed placement; `position` is the ace's movement offset from it. `Ace::world_position()` is `center + position`, and every attack launches from there.
- `World::ace: Option<Ace>` — absent by default, so a world only fires when an ace is placed.
- `Ace::process_movement` is a **shell**; it will mutate `position` and `phase`. `heading` is needed now because attack offsets rotate with it.
- Timers live in `AceAttacks` as `f32` ticks. Each tick both tick up by 1; a timer fires and subtracts its interval when it reaches it, so the dart's fractional 18.9-tick cadence carries its remainder.

## Attacks

| Attack | Count | Offset (rotated by heading) | Interval | Lifespan |
| --- | --- | --- | --- | --- |
| Dart | 32, radial | `(0, 6.5)` | 18.9 ticks (0.315 s) | 84 ticks (1.4 s) |
| Missile | 2 | `(12, −0.24)` and `(−12, −0.24)` | 90 ticks (1.5 s) | 1008 ticks (16.8 s) |

- Dart directions are a full ring, rotated by the ace's heading (dart `i` points at `heading + i·2π/32`).
- Dart stats: speed 6 units/tick, pierce 8, damage 1.
- Missile stats: pierce 1, damage 0. Missiles launch with the ace's heading, at 25 units/s, and seek. A missile only contacts MOAB-class Bloons (MOAB, BFB, ZOMG, DDT, BAD, and their fortified variants); overlaps with weaker Bloons are ignored (no damage, no pierce, no detonation). On contact it detonates into a one-tick explosion (radius 30, pierce 5, damage 150) that damages every Bloon kind in range. See ADR 0006.

## Missile steering (ADR 0005)

All values are radians and radians per tick, converted from the spec's per-second degrees with `TICK_DT = 1/60`:

- launch speed 25 u/s → `25·TICK_DT`,
- acceleration 160 u/s² → `160·TICK_DT²`, capped at 400 u/s → `400·TICK_DT`,
- initial angular velocity 25°/s → `25°·TICK_DT` toward the target,
- angular acceleration 150°/s² → `150°·TICK_DT²`, capped at 500°/s → `500°·TICK_DT`.

Each tick a seeking projectile accelerates its speed, nudges its angular velocity in the direction that reduces the angle error (via `wrap_pi`), turns by its angular velocity, then moves along its heading. The target is re-chosen every tick, among MOAB-class Bloons only: highest `danger`, then furthest along its path, then lowest id.

## Projectile lifecycle

- `Projectile` carries `speed`, `angular_velocity`, and `ticks_remaining`.
- A projectile dies when its pierce is spent or its lifespan runs out; the sweep removes it. Leaving the play area does not destroy it — collision checks simply skip out-of-bounds projectiles.
- A projectile fired this tick is not moved until the next tick, but it can connect on the tick it appears.

## Verification

- A volley of 32 darts leaves from the rotated offset with headings spaced `2π/32` apart.
- The dart ring and offsets rotate with the ace's heading.
- A missile volley launches two missiles from the rotated offsets.
- Missiles accelerate and turn toward their target.
- Missiles deal no direct damage; each MOAB-class contact spawns a one-tick explosion that damages every Bloon within 30 units.
- Targeting ranks by danger, then path distance, then id.
- Darts fly straight and expire after their lifespan; leaving the play area does not remove them.
