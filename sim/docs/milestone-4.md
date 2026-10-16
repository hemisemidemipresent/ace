# Milestone 4 — Projectile spawn and movement shells

Scaffolding for projectile behaviour. The seams exist and are wired into the tick, but their bodies are empty: nothing about projectile motion is decided or implemented yet.

## Seams

- `movement::move_projectiles(&mut [Projectile])` — **shell.** Will travel each projectile along its `heading` at its kind's speed, steer the heading of seeking projectiles, and remove projectiles that leave the play area.
- `World::spawn_projectile(kind, position, heading) -> ProjectileId` — **implemented.** Allocates an id from `World::next_projectile_id` and pushes the projectile. This is the immediate spawner tests and future towers use.
- `World::spawn_projectiles(&mut self)` — **shell.** The ace's per-tick firing logic will live here.

## Projectile state

- `Projectile` gains a mutable `heading: f32`, in the same convention as `Enemy` (`0 = +x`, counter-clockwise positive). Seeking projectiles will steer it.

## Tick order

`World::simulate` now runs:

1. move enemies,
2. **move projectiles** (shell),
3. spawn bloons,
4. **spawn projectiles** (shell),
5. collide,
6. apply,
7. resolve.

Projectiles spawn after Bloons' movement and before the collision check, so a projectile spawned this tick can connect on the tick it appears.

## Verification

- `spawn_projectile` allocates sequential ids and records position and heading.
- Movement is inert, so a spawned projectile does not move on its tick. This test will be rewritten when the shell is filled in.
