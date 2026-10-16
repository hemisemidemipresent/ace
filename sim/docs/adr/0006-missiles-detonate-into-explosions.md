# 0006. Missiles detonate into a short-lived explosion projectile

## Context

Missiles home on MOAB-class Bloons. They should deal no damage themselves: on contact they explode, damaging everything nearby. We had to decide how the blast is represented, when it lands, and where it is created, without letting the pure collision checker construct projectiles.

## Decision

A missile's `damage` is `0`. When it contacts a MOAB-class Bloon (its only legal target) the collision is still surfaced by the damage pass. A `detonate_missiles` step in `World::simulate`, running after damage application and before death resolution, spawns a `ProjectileType::Explosion` at the missile's position. The explosion is an ordinary stationary projectile — `speed 0`, `radius 30`, `pierce 5`, `damage 150`, `lifespan_ticks 1` — whose radius doubles as the blast radius. The pure checker and the damage pass never construct projectiles; the blast is caught by the normal checker on the following tick and swept after that pass. Explosions damage every Bloon kind, including the Bloon that triggered them, and never chain.

## Considered Options

- **A single-tick area query inside the collision or damage pass.** Immediate, but it would put projectile creation inside the checker and fork the damage path, contrary to ADR 0002 and ADR 0003's note that splash is a separate query.
- **A longer-lived blast volume.** With hit-once damage, later ticks only matter for Bloons that drift in, which is not how an explosion reads.
- **A small direct hit plus splash from the missile.** The spec says the missile itself does no damage.

## Consequences

- `simulate` returns a `Collision` for a missile contact that applied no damage; ADR 0003's return contract is amended to "the overlaps that resolved into damage or a detonation".
- The explosion damages one tick after the detonation, so the visible blast and its damage are one tick apart.
- The prototype draws a renderer-only 0.25 s flash, because the explosion projectile itself lives a single tick.