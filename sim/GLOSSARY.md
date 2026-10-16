# ace_simulator

Headless simulation of BTD6 Monkey Ace combat. It tracks Bloons and the projectiles the ace fires, one tick at a time.

## Language

### Enemies

**Bloon**:
The umbrella class of all enemy creatures in BTD6.
_Avoid_: creep, mob

**Red**:
The basic single-hit Bloon class.
_Avoid_: basic bloon, normal bloon, grunt

**MOAB**:
The large, high-health Bloon class. Pops into four Ceramics.
_Avoid_: blimp, boss

**BFB**:
A very large, very high-health Bloon class. Pops into four MOABs.
_Avoid_: big bloon, bfb bloon

**ZOMG**:
The largest, highest-health Bloon class. Pops into four BFBs.
_Avoid_: zomg bloon, mega bloon

**DDT**:
A fast, high-health Bloon class. Pops into four Ceramics.
_Avoid_: ddt bloon

**BAD**:
The strongest Bloon class. Pops into two ZOMGs and three DDTs.
_Avoid_: bad bloon

**Fortified**:
A Ceramic-or-stronger Bloon variant with double Health; its Children are Fortified too.
_Avoid_: armored, reinforced

**Ceramic**:
A fast, high-health Bloon class. It pops into nothing yet.
_Avoid_: ceramic bloon, cera

**Enemy**:
A Bloon instance present in the simulation.
_Avoid_: unit, target

**Health**:
The damage a Bloon can still absorb before it pops.
_Avoid_: HP, life

**Child Bloon**:
A Bloon spawned when its parent is popped. It appears behind the parent on the same Path.
_Avoid_: offspring, spawn

**Danger**:
A Bloon kind's targeting priority; higher means stronger. Used to pick a seeking projectile's target.
_Avoid_: strength, threat

### Projectiles

**Projectile**:
A travelling object fired by a tower that can collide with Bloons.
_Avoid_: bullet, shot

**Dart**:
The basic projectile fired by the Monkey Ace.
_Avoid_: bullet

**Missile**:
A heavier projectile fired by the Monkey Ace. It seeks MOAB-class Bloons and deals no damage; on contact it detonates.
_Avoid_: bomb, rocket, laser

**Explosion**:
The one-tick blast a Missile leaves where it detonates. It damages every Bloon within its blast radius.
_Avoid_: bomb, frag, AoE

**Blast radius**:
The distance from an Explosion's centre at which it can damage a Bloon.
_Avoid_: splash, area

**Pierce**:
The number of distinct Bloons a projectile can damage before it is spent.
_Avoid_: penetration

### Ace

**Ace**:
The Monkey Ace: the moving point that fires the simulation's projectiles. It has a fixed center on the map and a movement position relative to it.
_Avoid_: plane, tower, monkey

**Targeting**:
The flight pattern the Ace follows. One of Circle, Figure Infinite, or Figure Eight.
_Avoid_: path, mode, pattern

**Phase**:
The Ace's internal movement clock.
_Avoid_: angle, timer

### Combat

**Damage**:
The health removed from an Enemy when a Projectile hits it.
_Avoid_: attack, DPS

**Collision**:
A geometric overlap between one Enemy and one Projectile during a tick.
_Avoid_: contact

**Hit**:
A collision that applied damage. A projectile hits each distinct Enemy at most once.
_Avoid_: contact, impact

**Detonation**:
The moment a Missile contacts a MOAB-class Bloon and becomes an Explosion. It deals no damage itself.
_Avoid_: impact, hit

**Pop**:
The destruction of a Bloon when its health reaches zero.
_Avoid_: kill, death

### Paths

**Path**:
The fixed polyline a Bloon travels from its spawn point to its exit.
_Avoid_: track, route, lane

**Waypoint**:
A vertex of a Path.
_Avoid_: node, point, control point

**Path distance**:
How far along its Path an Enemy has travelled.
_Avoid_: progress, offset

**Leak**:
A Bloon that reaches the end of its Path without being popped; it is removed from the simulation.
_Avoid_: escape, exit

### Geometry

**Hitbox**:
The shape used to decide whether two objects collide. A projectile has one circle; a Bloon has one circle or a compound of circles.
_Avoid_: collider, bounds

**Compound hitbox**:
A Bloon hitbox built from several circles.
_Avoid_: multi-hitbox, group hitbox

**Heading**:
The direction in which a Bloon is currently travelling.
_Avoid_: rotation, angle, bearing

### Rounds

**Round**:
A predetermined set of Bloon Groups that spawn together as one episode.
_Avoid_: wave, level

**Bloon Group**:
A run of a single Bloon kind that spawns `count` times spread across a start–end window.
_Avoid_: wave, batch

### Simulation

**Tick**:
One discrete step of the simulation. The game advances sixty ticks per second.
_Avoid_: frame, step

**World**:
The complete mutable state of a running simulation.
_Avoid_: state, sim

**Spawn**:
A Bloon entering the simulation at a scheduled tick.
_Avoid_: wave, round

**Anchored**:
An Enemy that is not on any Path and never moves.
_Avoid_: static, frozen

**Leaked penalty**:
The total Health of a leaked Bloon plus the recursive Health of its Children; the cost a leak imposes on a run.
_Avoid_: leak cost
