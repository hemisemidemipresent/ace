# Milestone 2 — Damage, pierce, and death

Builds on the [milestone 1 contract](./milestone-1.md). Vocabulary lives in [GLOSSARY.md](../GLOSSARY.md); the reasoning is in the [ADRs](./adr/), especially ADR 0003.

## Scope

**In scope**

- A `damage` stat and hit-once damage.
- Runtime pierce consumption.
- Death resolution, including the seam where child Bloons will be emitted.

**Out of scope (deferred)**

Movement, targeting, child-Bloon spawning, splash damage, and continuous collision.

## Damage

- `ProjectileStats` gains `damage`. Dart deals 1, Missile deals 2.
- A hit reduces `Enemy::hp` with `saturating_sub`.
- A projectile damages each distinct enemy at most once. It records the enemies it has hit in `hit_enemies`, so a stationary projectile cannot damage the same target on consecutive ticks.

## Pierce

- `Projectile` gains runtime state `pierce_remaining`, seeded from the kind's `pierce`.
- Every hit spends one pierce; a projectile with none left is spent and cannot hit again.
- Pierce counts **distinct** enemies, not damage events.

## Ordering

Hits are applied in the collision checker's canonical order:

1. by projectile index,
2. then by the distance from the projectile's centre to the nearest overlapping hitbox circle,
3. then by enemy index.

This order is what decides which enemies a limited-pierce projectile reaches.

## Death resolution

- Deaths are resolved **after** the whole damage pass, in `resolve_deaths`.
- Popped enemies (`hp == 0`) and spent projectiles are removed by `retain`.
- Child Bloons are emitted in this step (see [milestone 5](./milestone-5.md)). Because it runs last, a child can only be hit on a later tick.

## Reporting

- `World::simulate` returns `Vec<Collision>`, where `Collision { enemy: EnemyId, projectile: ProjectileId }` names a **hit that applied damage**.
- Hits skipped because the enemy was already dead, the projectile was spent, or the projectile had already hit that enemy are not reported.
- Damage is otherwise observed through world state: `Enemy::hp`, `Projectile::pierce_remaining`, and the lengths of `World::enemies` / `World::projectiles`.

## Verification

- Damage reduces health; a popped enemy is removed.
- A projectile damages a given enemy only once, even across ticks.
- Pierce caps the number of distinct enemies hit, nearest first.
- A dead enemy does not absorb a later hit or spend its projectile's pierce.
- `simulate` reports applied hits only.
