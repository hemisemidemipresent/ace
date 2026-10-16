# 0003. Hit-once projectile damage with deferred death resolution

## Context

Damage had to be applied before movement exists, so a stationary projectile overlaps its target on every tick. Applying damage per tick would make a projectile grind down the same enemy frame after frame — an artifact of the missing movement, not a rule of the game.

## Decision

A projectile remembers which enemies it has hit. Each distinct enemy is damaged at most once per projectile, and `pierce` counts distinct enemies. Hits are applied in the checker's order: by projectile, then nearest enemy. Deaths are resolved only after the whole damage pass, so nothing spawned or removed mid-pass can invalidate another hit's index. `simulate` returns the overlaps that resolved into an effect: damage, or (since ADR 0006) a missile detonation.

## Considered Options

- **Per-tick damage.** Simpler, but double-damages stationary enemies each frame.
- **Resolve deaths — and spawn children — inline during the pass.** Would let a child be hit later in the same tick and would invalidate indices mid-pass.

## Consequences

`Projectile` carries runtime state (`pierce_remaining`, `hit_enemies`) and is no longer a plain value. Until movement lands, a projectile that still has pierce after hitting everything it overlaps lingers harmlessly. Splash damage, when it arrives, is a separate broadphase query rather than a change to `check_collisions`.
