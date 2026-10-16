# Milestone 3 — Movement and spawning

Builds on milestones [1](./milestone-1.md) and [2](./milestone-2.md). Vocabulary lives in [GLOSSARY.md](../GLOSSARY.md); the reasoning is in the [ADRs](./adr/), especially ADR 0004.

## Scope

**In scope**

- Bloon movement along a path.
- Tick-scheduled spawning.
- Leaks.

**Out of scope (deferred)**

Projectile movement, targeting, child-Bloon spawning, splash damage, and continuous collision.

## Paths

- `src/paths.rs` hardcodes the track's waypoints (transcribed from `bloon_paths.csv`, rounded to three decimals).
- `Path::new` recomputes cumulative distances from the waypoint geometry. The CSV `distance` column is **not** used: it is the game track's true arc length, which is longer than the straight polyline on two curved stretches.
- `PathId` is either `Anchored` (not on a path) or `Indexed(u32)`.
- `Paths::get(PathId::Anchored)` is a programming error and panics; `Paths::try_get` returns `Option` for callers that want it.
- `Path::sample(distance)` returns the position and heading at a distance, clamped to the ends. `Path::speed_multiplier_at(distance)` returns the current segment's multiplier (always `1.0` for the bundled path).

## Enemy movement

- `Enemy` carries `path_id` and `path_distance`; `position` and `heading` are cached and re-derived each tick.
- `Enemy::new` produces an **anchored** enemy (never moves); `World::spawn_enemy` puts one on a path.
- Per tick: `path_distance += enemy_stats.speed * segment_speed_multiplier`, clamped to the path length. Position and heading are then sampled from the segment containing the distance.
- Anchored enemies are skipped by movement.

## Spawning

- `Spawn { tick, kind, path_id }` is queued with `World::schedule_spawn`.
- `World` owns enemy ids (`next_enemy_id`); `World::spawn_enemy(kind, path_id)` allocates one and places the Bloon at distance 0.
- Spawns scheduled for the current tick are emitted during `simulate`. Spawning happens **after** movement, so a newly spawned Bloon is not moved until the next tick. Multiple bloons on one tick stack at distance 0.

## Tick order

`World::simulate` runs, in order:

1. **move** enemies,
2. **spawn** the bloons due this tick,
3. **collide** (pure check),
4. **apply** damage and pierce,
5. **resolve** — remove popped, leaked, and spent entities,
6. `tick += 1`.

`World::tick` starts at `0`, so the first `simulate` is tick 0.

## Leaks

- A Bloon whose `path_distance` reaches the path length has leaked. It is removed in the resolve step.
- Lives and leak events are not modelled; `simulate` still returns applied `Collision`s only.

## Verification

- `Path::sample` walks straight segments, including heading changes at corners.
- The bundled path's length is recomputed (first segment 108.7 units, not the CSV's 212.0).
- Enemies advance by their kind's speed and follow the path's position and heading.
- A Bloon is not moved on the tick it spawns; it moves from the next tick.
- Anchored enemies never move.
- Reaching the path end removes the Bloon.
- Resolving a path for `PathId::Anchored`, or spawning one, panics.
