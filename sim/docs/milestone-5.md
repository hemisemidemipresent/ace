# Milestone 5 — Child Bloon spawning

Builds on [milestone 3](./milestone-3.md) (movement) and the death seam from [ADR 0003](./adr/0003-hit-once-projectile-damage.md). Vocabulary lives in [GLOSSARY.md](../GLOSSARY.md).

## Scope

When a Bloon is popped, it leaves children on the same path.

## Data

| Kind | HP | Speed (units/tick) | Hitbox | Children |
| --- | --- | --- | --- | --- |
| Red | 1 | 1.0 | circle r 8 | — |
| Ceramic | 60 | 2.5 | circle r 8 | — |
| MOAB | 200 | 0.25 | 2 circles: r 14.7 @ +6.3, r 13.18 @ −11 | 4 Ceramic |
| BFB | 700 | 0.0625 | 2 circles: r 18.01 @ +12.3, r 20.04 @ −11.6 | 4 MOAB |
| ZOMG | 4000 | 0.045 | 3 circles: r 15.23 @ +17.2, r 19.64 @ +5.1, r 24.02 @ −17.2 | 4 BFB |

- `EnemyStats.children: &'static [EnemyType]` lists a kind's children **in spawn order**; it is empty for a kind that pops into nothing.
- Children can be mixed kinds — the list is just a sequence of `EnemyType`s, so any ordering of any kinds is expressible.

## Placement

For a popped parent at `parent_distance` with `count` children:

- `spacing = min(26.7 / count, 8.6)`,
- child `i` (0-indexed) sits at `parent_distance − i · spacing`, clamped to `>= 0`.

The first child sits at the parent's location; each later child trails the previous one, earlier on the track. Distance is clamped to zero so a brood popped near the start stacks at the path start instead of carrying a negative distance.

`World::spawn_enemy_at(kind, path_id, path_distance)` places a Bloon at an arbitrary distance; the existing `spawn_enemy` is `spawn_enemy_at(.., 0.0)`.

## Timing

- Children are emitted in `resolve_deaths`, **after** the whole damage pass.
- They therefore cannot be moved or hit until the next tick, and popping never cascades within a tick.
- An **anchored** parent has no path and emits no children.
- A **leaked** parent was not popped and emits no children.
- Children are appended after the popped and leaked enemies are removed, and take ids from `next_enemy_id` in child order.

## Verification

- A MOAB pops into four Ceramics at full health.
- Children trail the parent at the capped spacing, first child at the parent's location.
- Spacing is capped at 8.6 for broods of three or fewer and is `26.7 / count` beyond that.
- Distances below zero clamp to zero.
- Anchored and leaked parents emit no children.
- A pop does not cascade within its tick.
