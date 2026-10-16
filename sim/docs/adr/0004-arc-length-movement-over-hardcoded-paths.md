# 0004. Arc-length movement over hardcoded paths

## Context

Bloons travel a fixed track. `bloon_paths.csv` provides 37 waypoints and a cumulative `distance` column, but that column is the game track's true arc length: on two curved stretches it exceeds the straight polyline between the listed waypoints (by ~103 and ~146 units). Movement follows the waypoints, so the polyline is the geometry that matters.

## Decision

The waypoints are hardcoded in `src/paths.rs` (rounded to three decimals), and cumulative distances are recomputed from that geometry; the CSV `distance` column is ignored. An enemy carries a `path_id` and a scalar `path_distance`; position and heading are derived by sampling the path at that distance, so heading falls out of movement and feeds the rotating compound hitboxes. `PathId` has an `Anchored` variant for enemies that do not move, and resolving a path for it is a programming error that panics. No CSV parser or dependency is introduced.

## Considered Options

- **Parse the CSV at runtime.** Keeps the file authoritative, but adds IO, a dependency or a hand-rolled parser, and can still disagree with polyline movement.
- **Trust the `distance` column.** Would place a Bloon at distance 212 beyond the end of the 108.7-unit first segment; geometry and distance would disagree.
- **Store a waypoint index instead of arc length.** Simpler, but makes speed, leaks, and interpolation clumsier.

## Consequences

Changing the track means editing Rust source, not the CSV. Coordinates are rounded to three decimals — a sub-millimetre change that also removes float32 noise. If the track data becomes dynamic, a parser can be added behind a `Paths::from_*` constructor without touching callers.
