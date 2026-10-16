# 0005. Seeking missiles steer with angular velocity toward the strongest Bloon

## Context

Missiles must home on bloons, and BTD6 exposes a specific feel: they launch slow, accelerate, and curve toward a target rather than snapping to it. The ace's darts are the opposite — straight and radial — so the two projectile kinds need different motion.

## Decision

A projectile is straight unless its stats carry `seeking`. A seeking projectile keeps its own `speed` and `angular_velocity`; each tick it accelerates its speed toward a cap, nudges its angular velocity by a fixed angular acceleration in the direction that reduces the angle to the target, and turns by that angular velocity. The target is the **strongest** enemy — highest `danger`, then furthest along its path, then lowest id — re-evaluated every tick. Angles are handled in radians per tick, and `wrap_pi` normalises the error. Distances, speeds, and angles from the BTD6 spec are converted by `TICK_DT`.

## Considered Options

- **Pure pursuit.** Point the heading straight at the target each tick. Simple, but no turn-rate limit, so missiles never overshoot or circle.
- **Proportional navigation.** Steer by the target's line-of-sight rate. More realistic, but the parameters given are angular velocity/acceleration, which this model matches directly.
- **Locking the target at launch.** Cheaper, but missiles would chase a corpse or ignore a stronger Bloon.

## Consequences

A missile can orbit its target when its turn radius exceeds the distance; the gradual angular-velocity reversal as the target crosses sides is deliberate. Targeting depends on `EnemyStats.danger` rather than raw health, so a MOAB is preferred over a Ceramic even when damaged.
