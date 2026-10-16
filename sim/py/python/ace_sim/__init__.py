"""Python training environment for the ``ace_simulator`` headless simulation.

The heavy lifting lives in the compiled ``ace_sim._ace_sim`` extension; this
module adds an RL-friendly ``Env`` wrapper on top of the low-level ``Session``.

Quick start
-----------

    from ace_sim import Env, HOLD

    env = Env(round="98", ticks_per_step=6,
              ceramic_health_multiplier=1.0, moab_health_multiplier=1.0,
              bloon_speed_multiplier=1.0, moab_speed_multiplier=1.0)
    obs = env.reset(ace={"phase": 1.0})
    terminated = truncated = False
    while not (terminated or truncated):
        obs, terminated, truncated, info = env.step(HOLD)

The agent controls the ace's flight pattern and direction: an action is
``targeting`` (0 = Circle, 1 = Figure Infinite, 2 = Figure Eight) crossed with
``reverse`` (bool), encoded as an integer ``0..5``, a ``(targeting, reverse)``
pair, or ``HOLD`` to keep the previous input. Inputs may be given at most once
every six ticks, but the agent may hold for longer.

Every step returns the whole World: scalars plus numpy arrays for Enemies,
Projectiles, registered Hits, and pending Spawns. See ``Env.state`` and
``Env.paths``. Reward is intentionally *not* returned -- the caller shapes it --
but ``leaked_penalty_step`` / ``leaked_penalty_total`` are exposed, where a
leak costs the leaked Bloon's remaining Health plus the recursive Health of its
Children.
"""

from __future__ import annotations

from typing import Any, Dict, List, Optional, Sequence, Tuple, Union

from ._ace_sim import (
    Session,
    kind_names,
    list_rounds,
    projectile_names,
    targeting_names,
)

__all__ = [
    "Env",
    "Session",
    "HOLD",
    "ROUNDS",
    "KIND_NAMES",
    "PROJECTILE_NAMES",
    "TARGETING_NAMES",
    "list_rounds",
    "kind_names",
    "projectile_names",
    "targeting_names",
]

__version__ = "0.1.0"

#: Sentinel meaning "keep the current targeting/reverse input".
HOLD: int = -1

#: The bundled Round names, e.g. ``["98"]``.
ROUNDS: List[str] = list_rounds()

#: Bloon kind names indexed by the integer code used in observations.
KIND_NAMES: List[str] = kind_names()

#: Projectile kind names indexed by the integer code used in observations.
PROJECTILE_NAMES: List[str] = projectile_names()

#: Targeting names indexed by the integer code used in observations.
TARGETING_NAMES: List[str] = targeting_names()

#: The minimum number of ticks between two agent inputs.
MIN_INPUT_INTERVAL: int = 6

#: The four tunable simulation parameters.
_PARAM_KEYS = (
    "ceramic_health_multiplier",
    "moab_health_multiplier",
    "bloon_speed_multiplier",
    "moab_speed_multiplier",
)

Action = Union[int, Tuple[Union[int, str], bool], None]


def _decode_action(action: Action) -> Optional[Tuple[int, bool]]:
    """Normalise an action into ``(targeting, reverse)`` or ``None`` (hold)."""
    if action is HOLD or action is None:
        return None
    if isinstance(action, (tuple, list)):
        targeting, reverse = action
        if isinstance(targeting, str):
            try:
                targeting = TARGETING_NAMES.index(targeting)
            except ValueError as error:
                raise ValueError(
                    f"unknown targeting {targeting!r}; expected one of {TARGETING_NAMES}"
                ) from error
        targeting = int(targeting)
        if not 0 <= targeting <= 2:
            raise ValueError("targeting must be 0, 1, or 2")
        return targeting, bool(reverse)
    code = int(action)
    if not 0 <= code < 6:
        raise ValueError(
            "action must be an int 0..5, a (targeting, reverse) pair, or HOLD"
        )
    return code % 3, bool(code // 3)


class Env:
    """A single-world RL environment over the headless simulation.

    Each ``step`` advances at least :data:`MIN_INPUT_INTERVAL` ticks with the
    chosen ``targeting``/``reverse`` held; call :meth:`reset` to begin a Round.
    """

    def __init__(
        self,
        round: str = "98",
        ticks_per_step: int = MIN_INPUT_INTERVAL,
        ace: Optional[Dict[str, Any]] = None,
        **params: float,
    ) -> None:
        self._session = Session()
        self.round = round
        self.ticks_per_step = int(ticks_per_step)
        self.ace: Optional[Dict[str, Any]] = dict(ace) if ace is not None else None
        self.params: Dict[str, float] = {key: 1.0 for key in _PARAM_KEYS}
        self._set_params(params)
        self._targeting = 0
        self._reverse = False

    def _set_params(self, params: Dict[str, float]) -> None:
        for key, value in params.items():
            if key not in _PARAM_KEYS:
                raise TypeError(f"unexpected parameter {key!r}")
            self.params[key] = float(value)

    # -- lifecycle ---------------------------------------------------------

    def reset(
        self,
        round: Optional[str] = None,
        *,
        ticks_per_step: Optional[int] = None,
        ace: Optional[Dict[str, Any]] = None,
        **params: float,
    ) -> Dict[str, Any]:
        """Begin a new episode and return the first observation.

        Parameters and ``ace`` are sticky: values set on the constructor (or a
        previous reset) are reused unless overridden here. ``ace`` may override
        any :class:`Session` ace field: ``center``, ``position``, ``target_pos``
        (each an ``(x, y)`` pair), ``heading``, ``phase`` (radians),
        ``base_velocity``, ``on_path``, ``targeting``, ``reverse``,
        ``dart_timer``, and ``missile_timer``.
        """
        if round is not None:
            self.round = round
        if ticks_per_step is not None:
            self.ticks_per_step = int(ticks_per_step)
        if ace is not None:
            self.ace = dict(ace)
        self._set_params(params)

        observation = self._session.reset(
            self.round,
            self.params["ceramic_health_multiplier"],
            self.params["moab_health_multiplier"],
            self.params["bloon_speed_multiplier"],
            self.params["moab_speed_multiplier"],
            self.ace,
        )
        self._targeting = int(observation["ace_targeting"])
        self._reverse = bool(observation["ace_reverse"])
        return observation

    def step(
        self,
        action: Action = HOLD,
        ticks: Optional[int] = None,
    ) -> Tuple[Dict[str, Any], bool, bool, Dict[str, Any]]:
        """Advance the World, holding ``action`` for ``ticks`` ticks.

        Returns ``(observation, terminated, truncated, info)``. ``terminated``
        means every scheduled Bloon is gone (popped or leaked); ``truncated``
        means the two-minute safety cap was reached. The caller computes reward
        from ``observation["leaked_penalty_step"]`` (or ``info``).
        """
        ticks = self.ticks_per_step if ticks is None else int(ticks)
        if ticks < MIN_INPUT_INTERVAL:
            raise ValueError(
                f"the agent may input at most once every {MIN_INPUT_INTERVAL} ticks"
            )

        decoded = _decode_action(action)
        if decoded is not None:
            self._targeting, self._reverse = decoded

        observation, terminated, truncated = self._session.step(
            self._targeting, self._reverse, ticks
        )
        info = {
            "round": self.round,
            "leaked_penalty_step": observation["leaked_penalty_step"],
            "leaked_penalty_total": observation["leaked_penalty_total"],
            "ticks": ticks,
        }
        return observation, terminated, truncated, info

    # -- introspection -----------------------------------------------------

    def state(self) -> Dict[str, Any]:
        """The current observation, without advancing the World."""
        return self._session.state()

    def paths(self) -> Dict[str, Any]:
        """The track geometry (waypoints, distances, speed multipliers)."""
        return self._session.paths()

    def observe_flat(
        self, max_bloons: int = 512, max_projectiles: int = 4096
    ) -> Dict[str, Any]:
        """A fixed-capacity, padded float32 view suitable for a neural net.

        Returns ``global`` (1-D) and ``enemies``/``projectiles`` reshaped to
        ``(max, stride)``. The trailing column of each row is a 1.0/0.0
        presence mask.
        """
        import numpy as np

        flat = self._session.observe_flat(max_bloons, max_projectiles)
        return {
            "global": flat["global"],
            "enemies": np.asarray(flat["enemies"]).reshape(
                max_bloons, flat["enemy_stride"]
            ),
            "projectiles": np.asarray(flat["projectiles"]).reshape(
                max_projectiles, flat["projectile_stride"]
            ),
        }

    @property
    def leaked_penalty_total(self) -> int:
        """The episode's accumulated leaked penalty."""
        return self._session.leaked_penalty_total

    def close(self) -> None:
        """Present for API symmetry; the extension holds no OS resources."""
        return None
