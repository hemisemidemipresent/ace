# 0007. A separate PyO3 bridge crate exposes the simulator to Python

## Context

We want to drive the headless simulation as a reinforcement-learning training
environment from Python: dial the four `SimConfig` multipliers, pick a Round,
set the ace's starting state, and read the whole `World` back every step. The
simulation crate is deliberately small and dependency-free, so the binding
should not drag a Python runtime into its default build or reshape its API.

## Decision

A new crate, `sim/py`, depends on `ace_simulator` by path and builds a PyO3
extension module (`ace_sim._ace_sim`) plus a thin Python `Env` wrapper. Wheels
are produced per platform with `maturin`; the simulation crate's build is
unchanged.

The bridge owns no simulation logic. It parses `rounds.json`, clamps the
parameters, accumulates `leaked_penalty`, and serialises the `World` into numpy
arrays. It reaches the simulation through two small, additive, non-breaking
seams: `World::leaks`, populated in `resolve_deaths` so a leak's kind and
remaining Health are visible before the Enemy is removed, and
`Path::waypoints`, so the track geometry can be exposed. `World::simulate`'s
signature and every existing test are untouched.

## Considered Options

- **C ABI `cdylib` + `ctypes`.** In-process and toolchain-light, but "expose
  everything" becomes a large hand-rolled marshalling surface and offers no
  numpy integration.
- **JSON over stdio subprocess.** Simplest to build, but pays per-step IPC and
  process-management costs on the hot path of training.
- **Editing `simulate` to return leaks.** Would change a public contract and
  break callers and tests; the additive `World::leaks` field avoids that.

## Consequences

- Users need a Rust toolchain (or a prebuilt wheel) and `maturin`; the base
  `cargo test`/`cargo clippy` loop is unaffected.
- Every observation allocates fresh numpy arrays. Throughput is adequate for
  training and can be improved later (fixed buffers, batch worlds) without
  moving the decision recorded here.
- The reward function stays out of Rust: Python shapes it from the exposed
  `leaked_penalty` and world state.
