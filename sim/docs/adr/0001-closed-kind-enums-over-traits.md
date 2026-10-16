# 0001. Closed kind-enums with a stats table over trait objects

## Context

Enemy and projectile kinds must be extensible: new kinds are expected as the simulator grows. Rust enums are closed, so "extensible enum" needed a concrete meaning.

## Decision

`EnemyType` and `ProjectileType` are closed enums naming each kind. Per-kind base stats (health, speed, hitbox, pierce) live in separate stats tables keyed by kind. Instances (`Enemy`, `Projectile`) carry identity, position, and current health alongside their kind. Adding a kind means adding one variant and one stats row.

## Considered Options

- **Data-carrying enum variants.** Couples each instance to base stats and duplicates that data per instance.
- **A `dyn Enemy` trait plus a registry.** Supports open extension and per-kind behavior, but is more machinery than the current kinds require.

## Consequences

Kinds cannot be added from outside the crate without editing the enum. If per-kind behavior (not just per-kind data) becomes necessary, this decision should be revisited.
