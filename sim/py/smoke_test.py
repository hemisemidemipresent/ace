"""Ad-hoc smoke test for the ace_sim Python environment (not shipped)."""

import numpy as np

from ace_sim import Env, HOLD, KIND_NAMES, ROUNDS

assert ROUNDS == ["98"], ROUNDS

env = Env(round="98", ticks_per_step=6)
obs = env.reset(ace={"phase": 1.0})
assert obs["ace_present"] is True
assert abs(obs["ace_phase"] - 1.0) < 1e-6

# Round 98 has 30 (0, 1800) BFBs and 8 (0, 300) ZOMGs.
assert obs["n_pending_spawns"] == 38, obs["n_pending_spawns"]
spawn_ticks = sorted(int(t) for t in obs["spawn_tick"])
assert spawn_ticks[0] == 0
# BFB spacing = floor(i*60); ZOMG spacing = floor(i*37.5)
assert 60 in spawn_ticks and 120 in spawn_ticks, spawn_ticks[:8]
assert 37 in spawn_ticks and 75 in spawn_ticks, spawn_ticks[:8]

# Shapes match counts.
assert obs["enemy_x"].shape == (obs["n_enemies"],)
assert obs["projectile_x"].shape == (obs["n_projectiles"],)
assert obs["hit_projectile_id"].shape == obs["hit_enemy_id"].shape

# One step spawns the tick-0 cohort.
obs, terminated, truncated, info = env.step(HOLD)
assert not terminated and not truncated
assert obs["tick"] == 6
assert obs["n_enemies"] == 38 or obs["n_enemies"] > 0

# Action decoding.
env.step(5)          # targeting=FigureEight(2), reverse=True
assert env._targeting == 2 and env._reverse is True
env.step(("circle", False))
assert env._targeting == 0 and env._reverse is False
env.step(HOLD)
assert env._targeting == 0 and env._reverse is False

# Cadence guard.
for bad in (0, 1, 5):
    try:
        env.step(HOLD, ticks=bad)
    except ValueError:
        pass
    else:
        raise AssertionError("expected ValueError for ticks < 6")

# Clamping.
obs = env.reset(
    bloon_speed_multiplier=999.0,
    moab_speed_multiplier=-5.0,
    ceramic_health_multiplier=999.0,
    moab_health_multiplier=0.0,
)
assert obs["bloon_speed_multiplier"] == 5.0
assert np.isclose(obs["moab_speed_multiplier"], 0.05)
assert obs["ceramic_health_multiplier"] == 20.0
assert np.isclose(obs["moab_health_multiplier"], 0.05)

# Flat view.
flat = env.observe_flat(max_bloons=64, max_projectiles=128)
assert flat["global"].dtype == np.float32
assert flat["enemies"].shape == (64, 8)
assert flat["projectiles"].shape == (128, 9)

# Paths.
paths = env.paths()["paths"]
assert len(paths) == 1
assert paths[0]["waypoint_x"].shape == paths[0]["waypoint_y"].shape
assert paths[0]["length"] > 0

# Constructor parameters are sticky, and can be overridden per reset.
sticky = Env(round="98", moab_speed_multiplier=2.0)
assert np.isclose(sticky.reset()["moab_speed_multiplier"], 2.0)
assert np.isclose(sticky.reset(moab_speed_multiplier=1.5)["moab_speed_multiplier"], 1.5)
assert np.isclose(sticky.reset()["moab_speed_multiplier"], 1.5)
try:
    Env(nonsense=1.0)
except TypeError:
    pass
else:
    raise AssertionError("expected TypeError for an unknown parameter")

# At default speed the MOAB-class Bloons cannot cross the track inside the
# two-minute cap, so a full episode truncates at exactly 7200 ticks.
obs = env.reset(
    ace={"phase": 0.0},
    ceramic_health_multiplier=1.0,
    moab_health_multiplier=1.0,
    bloon_speed_multiplier=1.0,
    moab_speed_multiplier=1.0,
)
steps = 0
while True:
    obs, terminated, truncated, info = env.step(HOLD)
    steps += 1
    if terminated or truncated:
        break
    assert steps < 2000, "episode did not end"
assert truncated and not terminated, (terminated, truncated)
assert obs["tick"] == 2 * 60 * 60
assert obs["leaked_penalty_total"] > 0
assert info["leaked_penalty_total"] == env.leaked_penalty_total

# At maximum speed every Bloon clears the track, so the episode terminates
# naturally once the schedule is exhausted and the World is empty.
obs = env.reset(
    ace={"phase": 0.0},
    ceramic_health_multiplier=1.0,
    moab_health_multiplier=1.0,
    bloon_speed_multiplier=5.0,
    moab_speed_multiplier=5.0,
)
steps = 0
while True:
    obs, terminated, truncated, info = env.step(HOLD)
    steps += 1
    if terminated or truncated:
        break
    assert steps < 2000, "episode did not end"
assert terminated and not truncated, (terminated, truncated, obs["tick"])
assert obs["n_enemies"] == 0
assert obs["n_pending_spawns"] == 0

print(
    "OK: truncated at",
    2 * 60 * 60,
    "| terminated at tick",
    obs["tick"],
    "steps",
    steps,
    "leaked_penalty_total",
    obs["leaked_penalty_total"],
)
