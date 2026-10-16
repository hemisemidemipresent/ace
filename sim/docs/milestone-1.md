# Milestone 1 — Data model and collision checker

The concrete contract for the first slice of `ace_simulator`. Vocabulary lives in [GLOSSARY.md](../GLOSSARY.md); the reasoning behind each decision lives in [`adr/`](./adr/).

## Scope

**In scope**

- Enemy and projectile kinds, their base stats, and their instances.
- A pure collision checker between the two groups.
- A per-tick `World::simulate` entry point.
- A demo binary that prints collisions, plus unit and integration tests.

**Out of scope (deferred)**

Movement and heading updates, targeting, damage/HP, pierce consumption, despawning, continuous collision (tunneling), multi-enemy priority rules, and the ordering of collision versus movement within a tick.

## Data model

- `EnemyType { Red, MOAB }` and `ProjectileType { Dart, Missile }` are closed enums; per-kind base stats live in a stats table keyed by kind (ADR 0001).
- Instances are `Enemy { id, kind, position, heading, hp }` and `Projectile { id, kind, position, pierce_remaining, hit_enemies }`, identified by `EnemyId` / `ProjectileId`. The last two projectile fields are runtime damage state added in [milestone 2](./milestone-2.md).
- Initial stats (speeds are world units per tick; unused until movement lands):

| Kind | HP | Speed | Hitbox | Pierce |
| --- | --- | --- | --- | --- |
| Red | 1 | 1.0 | circle, r 8 | — |
| MOAB | 200 | 0.25 | compound: 3 circles, r 12, local offsets `(-16,0)`, `(0,0)`, `(16,0)` | — |
| Dart | — | 10.0 | circle, r 3 | 2 |
| Missile | — | 20.0 | circle, r 1.5 | 1 |

- `pierce` is the number of distinct Bloons a projectile may damage before it is spent. The collision checker does not consume it; see [milestone 2](./milestone-2.md).

## Coordinate system and geometry

- Positions are `f32`; the origin is the play-area centre.
- Play area: `x ∈ [-200, 200]`, `y ∈ [-150, 150]`, edges inclusive.
- A projectile's hitbox is a single circle centred on its position.
- An enemy's hitbox is either a single circle or a compound of circles. Compound circles are local offsets rotated by the enemy's `heading` about the enemy's position.
- `heading` is in radians; `0` is `+x` and counter-clockwise is positive.
- Two circles overlap when the distance between their centres is ≤ the sum of their radii (touching counts).

## Broadphase

- A uniform grid covers the play area. Cell size is configurable via `SimConfig` (default `32.0`).
- The grid is derived state: it is rebuilt on every collision check and never stored on `World`.
- A brute-force reference implementation is kept behind the same interface.

## Collision contract (ADR 0002, ADR 0003)

- Signature: `check_collisions(enemies: &[Enemy], projectiles: &[Projectile], config: &SimConfig) -> Vec<Hit>`, where `Hit { enemy: usize, projectile: usize }` indexes the input slices.
- Pure: it mutates nothing and applies no damage or pierce.
- Reports each overlapping pair at most once, even when several compound circles overlap one projectile.
- Deterministic order: by projectile index, then the distance from the projectile's centre to the nearest overlapping hitbox circle, then enemy index. This is the order in which pierce resolves.
- Out of bounds: an object is out of bounds if and only if its **centre** lies outside the play area. Any pair involving an out-of-bounds object is skipped. A hitbox that spills over the edge is not clipped, but the grid never populates out-of-range cells.

## Simulation

- `World { enemies: Vec<Enemy>, projectiles: Vec<Projectile>, config: SimConfig }`.
- `World::simulate(&mut self) -> Vec<Collision>` runs exactly one tick: it builds the grid, runs the checker, applies the hits (see [milestone 2](./milestone-2.md)), resolves deaths, and returns the applied hits as `Collision { enemy: EnemyId, projectile: ProjectileId }` pairs.
- The demo binary prints one line per applied hit: `enemy {id} <-> projectile {id}`.

## Layout

`lib.rs` + `main.rs`, with modules:

| Module | Responsibility |
| --- | --- |
| `types.rs` | kind enums, stats tables, `Enemy`, `Projectile`, ids |
| `hitbox.rs` | `Vec2`, `Circle`, `EnemyHitbox`, `circles_overlap` |
| `broadphase.rs` | bounds, `in_bounds`, `SimConfig`, `Grid` |
| `collision.rs` | `Hit`, `Collision`, `check_collisions`, and the brute-force reference |
| `world.rs` | `World`, `simulate`, `apply_collisions`, `resolve_deaths` |

## Verification

- Unit tests: circle overlap (overlap / touch / separate), rotation, bounds inclusivity, grid insertion and query.
- Integration tests: single-circle and compound-hitbox collisions, compound dedupe, rotation changing hits, out-of-bounds skipping, deterministic ordering, grid-versus-brute-force equivalence, cell-size invariance, and the `simulate` return value.
- The demo binary runs and prints its collisions.
