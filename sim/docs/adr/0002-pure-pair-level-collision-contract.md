# 0002. The collision checker is pure and reports every overlapping pair

## Context

`simulate` must both find collisions and mutate the world in response to them (damage, pierce, despawning). Where that mutation lives — inside the collision pass or outside it — shapes the checker's interface.

## Decision

`check_collisions` is a pure function: it takes the enemy and projectile slices plus the config and returns the complete list of overlapping pairs. It spends no pierce, applies no damage, and removes nothing. `World::simulate` consumes that list and owns all mutation through a single `apply_collisions` seam.

## Considered Options

- **The checker owns pierce and removal.** Would let it early-exit per projectile once pierce is exhausted, but bakes damage rules into geometry and makes the checker stateful and hard to test.
- **The checker returns per-circle hits.** More granular, but forces every caller to deduplicate compound-hitbox overlaps.

## Consequences

`pierce` is inert until the damage system lands. The caller must collapse multiple hits on one projectile before applying pierce; the checker's sorted, deduplicated pair list makes that straightforward.
