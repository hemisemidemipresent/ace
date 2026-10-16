# ace_simulator

> Headless, frame-by-frame simulation of BTD6 Monkey Ace combat, in Rust.

`ace_simulator` models the collision layer of a 2D tower-defense simulation: Bloons moving through a bounded arena and the projectiles an ace fires, checked for overlap one tick at a time.

**Status: milestone 6.** Bloons spawn, travel a hardcoded path, and pop into children, while a Monkey Ace fires radial darts and seeking missiles that collide and deal damage. The ace's own flight and its targeting modes are still shells (see [Roadmap](#roadmap)).

## Quick start

```bash
cargo run                 # watch scheduled bloons travel the bundled path
cargo test                # unit + integration tests
cargo clippy --all-targets
cargo fmt --check
```

The demo places an ace, schedules a Red and a MOAB on the path, and fires:

```
tick 0: 0 hit(s), 3 enemy/enemies, 0 projectile(s)
...
tick 18: 0 hit(s), 3 enemy/enemies, 32 projectile(s)
tick 19: 1 hit(s), 2 enemy/enemies, 32 projectile(s)
```

## Prototype

An interactive prototype lives behind the optional `prototype` feature:

```bash
cargo run --bin prototype --features prototype
```

It draws the track, bloons as their hitbox circles, the ace as an arrow, darts as short lines, and missiles as green rectangles, with on-screen buttons for targeting and reverse. All rendering lives in `src/bin/prototype.rs`; the core crate stays dependency-free, and `cargo test` does not build the prototype.

> **Linking note.** On this machine the `x86_64-pc-windows-gnu` toolchain is missing `libimm32.a` (macroquad/miniquad links it directly). Copy `libwinapi_imm32.a` to `libimm32.a` in the same `winapi-x86_64-pc-windows-gnu` cache `lib/` directory, then build.

## Python training environment

[`sim/py`](./py) builds a PyO3 extension that runs the simulation as a
reinforcement-learning environment from Python: dial the four `SimConfig`
multipliers (clamped), pick a Round from `rounds.json`, set the ace's starting
state, and read the whole `World` back each step as a dict of numpy arrays. The
simulation crate stays dependency-free; the bridge depends on it by path. See
[`sim/py/README.md`](./py/README.md) and
[ADR 0007](./docs/adr/0007-pyo3-bridge-for-python-rl.md).

## How it fits together

```
World::simulate()
  ├─ move_enemies()            advance path_distance, then sample position/heading
  ├─ move_projectiles()        straight darts, seeking missiles
  ├─ spawn_due()               emit bloons scheduled for this tick
  ├─ spawn_projectiles()       the ace fires its dart and missile volleys
  └─ check_collisions()        pure: no mutation
       ├─ Grid (broadphase)    rebuilt each tick
       └─ circle tests         circle vs circle, incl. rotating compound hitboxes
  └─ apply_collisions()        damage + pierce, in hit order
  └─ resolve_deaths()          sweep popped/leaked/spent, then emit children
```

- A **tick** is one discrete step; the game advances 60 per second.
- **Bloons** spawn on a tick schedule and travel a hardcoded path; each carries a scalar `path_distance`, from which position and heading are sampled (ADR 0004). Reaching the path's end is a leak, and the Bloon is removed.
- The **collision checker** is a pure function over enemy and projectile slices. It reports index-based hits — ordered by projectile, then nearest enemy — and never spends pierce, deals damage, or despawns anything.
- **Mutation** belongs to `World::simulate`: `apply_collisions` deals damage and spends pierce, then `resolve_deaths` removes popped, leaked, and spent entities and spawns children behind each popped Bloon.
- A projectile damages each distinct enemy **once**, and `pierce` counts distinct enemies. `simulate` returns only the hits that connected.

## Project layout

| Path | Purpose |
| --- | --- |
| `src/types.rs` | `EnemyType` / `ProjectileType`, stats tables, `Enemy` / `Projectile` |
| `src/hitbox.rs` | `Vec2`, `Circle`, `EnemyHitbox`, `circles_overlap` |
| `src/broadphase.rs` | play-area bounds, `SimConfig`, uniform grid |
| `src/paths.rs` | hardcoded track waypoints, arc-length sampling, `PathId` |
| `src/ace.rs` | `Ace`, `Targeting`, attack offsets, `process_movement` (shell) |
| `src/movement.rs` | `move_enemies`, `move_projectiles`, `strongest_target` |
| `src/collision.rs` | `Hit` / `Collision`, `check_collisions` + brute-force reference |
| `src/world.rs` | `World`, `simulate`, `apply_collisions`, `resolve_deaths` |
| `src/main.rs` | demo binary |
| `py/` | Python bindings and RL environment (separate crate) |
| `tests/` | integration tests |

## Documentation

| Document | Contents |
| --- | --- |
| [GLOSSARY.md](./GLOSSARY.md) | Domain vocabulary: Bloon, Enemy, Projectile, Hitbox, Tick, … |
| [docs/milestone-1.md](./docs/milestone-1.md) | The milestone-1 contract: data model, geometry, broadphase, collision, simulation |
| [docs/milestone-2.md](./docs/milestone-2.md) | The milestone-2 contract: damage, pierce, death |
| [docs/milestone-3.md](./docs/milestone-3.md) | The milestone-3 contract: movement, spawning, leaks |
| [docs/milestone-4.md](./docs/milestone-4.md) | The milestone-4 shells: projectile spawning and movement |
| [docs/milestone-5.md](./docs/milestone-5.md) | The milestone-5 contract: child Bloon spawning |
| [docs/milestone-6.md](./docs/milestone-6.md) | The milestone-6 contract: the Monkey Ace and its attacks |
| [docs/adr/0001](./docs/adr/0001-closed-kind-enums-over-traits.md) | Closed kind-enums with a stats table, over trait objects |
| [docs/adr/0002](./docs/adr/0002-pure-pair-level-collision-contract.md) | A pure, pair-level collision contract |
| [docs/adr/0003](./docs/adr/0003-hit-once-projectile-damage.md) | Hit-once damage with deferred death resolution |
| [docs/adr/0004](./docs/adr/0004-arc-length-movement-over-hardcoded-paths.md) | Arc-length movement over hardcoded paths |
| [docs/adr/0005](./docs/adr/0005-seeking-missile-steering.md) | Seeking missile steering |
| [docs/adr/0006](./docs/adr/0006-missiles-detonate-into-explosions.md) | Missiles detonate into short-lived explosions |
| [docs/adr/0007](./docs/adr/0007-pyo3-bridge-for-python-rl.md) | A PyO3 bridge crate for the Python RL environment |

## Key decisions

- **Closed kind-enums + stats table** (ADR 0001). `EnemyType` and `ProjectileType` name each kind; per-kind data lives in one table. Adding a kind is one variant plus one stats row.
- **Pure collision checker** (ADR 0002). Geometry never mutates the world; `simulate` owns all mutation.
- **Hit-once damage, deferred deaths** (ADR 0003). A projectile damages each distinct enemy once, and deaths resolve after the whole pass so indices stay valid and child Bloons can only be hit on a later tick.
- **Arc-length movement over hardcoded paths** (ADR 0004). A Bloon's progress is one scalar; position and heading are sampled from the path, and cumulative distances are recomputed from the waypoints rather than trusted from the CSV.
- **Children trail the parent** (milestone 5). A popped Bloon leaves its children on the same path, spaced `min(26.7 / count, 8.6)` behind it, emitted only after the pass so they cannot be hit until the next tick.
- **Seeking missiles steer by angular velocity** (ADR 0005). They accelerate and turn toward the strongest Bloon — ranked by `danger`, then distance along the path, then id — re-chosen every tick.
- **Uniform grid broadphase.** The play area is fixed at `x ∈ [-200, 200]`, `y ∈ [-150, 150]`, so a grid is predictable; a brute-force path is kept for reference and testing.

## Roadmap

Deferred until later milestones:

- The ace's flight (`Ace::process_movement`) and its `targeting` / `reverse` modes.
- Splash damage (a separate broadphase query).
- Continuous collision (tunneling) and the ordering of collision versus movement within a tick.
