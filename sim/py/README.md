# ace_sim (Python)

Python bindings for [`ace_simulator`](../README.md): a headless training
environment for reinforcement-learning approaches to Monkey Ace micro.

The bindings are additive. The simulation crate keeps its own dependency-free
build; this crate depends on it by path and exposes one `World` at a time to
Python through [PyO3](https://pyo3.rs) and [rust-numpy](https://github.com/PyO3/rust-numpy).

## Build

Requires a Rust toolchain and Python 3.9+. The build is cross-platform; wheels
are produced per platform with [maturin](https://maturin.rs).

```bash
pip install maturin
cd sim/py
maturin build --release          # writes target/wheels/*.whl
pip install target/wheels/*.whl
```

For an editable, in-place build inside an active virtualenv, use
`maturin develop --release` instead.

```python
python smoke_test.py             # runnable end-to-end check
```

## Usage

```python
from ace_sim import Env, HOLD

env = Env(round="98", ticks_per_step=6)
obs = env.reset(
    ceramic_health_multiplier=1.0,   # clamped to [0.10, 20.0]
    moab_health_multiplier=1.0,      # clamped to [0.05, 20.0]
    bloon_speed_multiplier=1.0,      # clamped to [0.05, 5.0]
    moab_speed_multiplier=1.0,       # clamped to [0.05, 5.0]
    ace={"phase": 0.5},              # any Ace field; see below
)

terminated = truncated = False
while not (terminated or truncated):
    obs, terminated, truncated, info = env.step(HOLD)
```

### Actions

The agent steers the ace's chosen flight pattern and direction. An action is:

- an int `0..5`: `targeting * 1 + reverse * 3`, where targeting is
  `0 = Circle`, `1 = Figure Infinite`, `2 = Figure Eight`;
- a `(targeting, reverse)` pair, with `targeting` an int or a name such as
  `"figure_eight"`;
- `HOLD` (`-1`) or `None` to keep the previous input.

Inputs may be given **at most once every six ticks**. `step(action, ticks=n)`
holds `action` for `n` ticks with `n >= 6`; the agent may hold for longer than
six.

### Observation

Every step returns the whole World as a dict:

| Group | Keys |
| --- | --- |
| scalars | `tick`, `n_enemies`, `n_projectiles`, `n_pending_spawns`, `leaked_penalty_step`, `leaked_penalty_total`, `cell_size`, the four multipliers |
| ace | `ace_present`, `ace_center_x/_y`, `ace_position_x/_y`, `ace_heading`, `ace_phase`, `ace_base_velocity`, `ace_on_path`, `ace_targeting`, `ace_reverse`, `ace_dart_timer`, `ace_missile_timer`, `ace_target_pos_x/_y` |
| enemies | `enemy_id`, `enemy_kind`, `enemy_path_id`, `enemy_path_distance`, `enemy_x`, `enemy_y`, `enemy_heading`, `enemy_hp` |
| projectiles | `projectile_id`, `projectile_kind`, `projectile_x`, `projectile_y`, `projectile_heading`, `projectile_speed`, `projectile_angular_velocity`, `projectile_pierce_remaining`, `projectile_ticks_remaining` |
| hits | `hit_projectile_id`, `hit_enemy_id` (registered hit pairs) |
| spawns | `spawn_tick`, `spawn_kind`, `spawn_path_id` (Bloons still queued) |

Integer kind codes are named by `KIND_NAMES`, `PROJECTILE_NAMES`, and
`TARGETING_NAMES`. `env.paths()` returns the track geometry, and
`env.observe_flat(max_bloons, max_projectiles)` returns a padded, fixed-shape
float32 view for a neural net.

### Reward

Reward is deliberately left to the caller. The environment exposes
`leaked_penalty_step` / `leaked_penalty_total`, the sane default being the
negative of the step value:

> A leak costs the leaked Bloon's remaining Health plus the recursive Health of
> all its Children, scaled by the health multipliers in force.

### Episode ends

`terminated` is true once the Round's spawn schedule is exhausted **and** the
World has no Enemies left (all popped or leaked). `truncated` is true once the
two-minute safety cap (7200 ticks) is reached.

## Ace reset fields

`reset(ace={...})` accepts any of `center`, `position`, `target_pos` (each an
`(x, y)` pair), `heading`, `phase` (radians), `base_velocity`, `on_path`,
`targeting`, `reverse`, `dart_timer`, `missile_timer`.

## Rounds

Rounds live in [`rounds.json`](./rounds.json) and can be added without touching
Rust. Each group spawns a single Bloon kind `count` times spread evenly over
`[start, end)` in **ticks**, with Bloon `i` at
`floor(start + i * (end - start) / count)`. Only `"98"` ships today.

```json
{ "98": { "groups": [
    { "bloon": "BfbFortified", "start": 0.0, "end": 1800.0, "count": 30 },
    { "bloon": "Zomg", "start": 0.0, "end": 300.0, "count": 8 }
] } }
```
