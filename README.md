# ace

> Personal research repository exploring **ace micro** in *Bloons TD 6* (BTD6).

This repo collects everything I've built while studying how ace (the Monkey Ace tower) moves, targets, and reverses: a browser-based flight simulator, animations for a video explaining the technique, reverse-engineered wingmonkey targeting code, and data analysis of sky shredder missiles.

---

## Table of Contents

- [Repository Layout](#repository-layout)
- [Sub-Projects](#projects)
  - [Website: Ace Micro Simulator](#website-ace-micro-simulator)
  - [Manim Animations](#manim-animations)
  - [Sky Shredder Missile Research](#sky-shredder-missile-research)
  - [Wingmonkey Targeting Research](#wingmonkey-targeting-research)

---

## Repository Layout

| Path | Type | Purpose |
| --- | --- | --- |
| [`website/`](./website) | Web app (SolidJS + Vite) | Interactive ace micro simulator, live at GitHub Pages |
| [`manim_stuff/`](./manim_stuff) | Python / [Manim](https://www.manim.community/) | Animation scripts and simulation data behind an ace micro explainer video |
| [`media/`](./media) | Rendered output | Videos, SVG text assets, and image output produced by the Manim scripts |
| [`missiles/`](./missiles) | Python notebook + CSV | Analysis of sky shredder missile trajectories |
| [`wingmonkey/`](./wingmonkey) | Reverse-engineering notes | Decompiled and rewritten code for the wingmonkey targeting option |
| [`.github/workflows/`](./.github/workflows) | CI | Builds and deploys the website to GitHub Pages |

---

## Projects

### Website: ACE Micro Simulator

**Live site:** <https://hemisemidemipresent.github.io/ace/>

**Location:** [`./website`](./website)

An interactive, canvas-based simulator that lets you fly an ace around a BTD6 track and watch its path, targeting, and reversals play out in real time.

**Tech stack**

- [SolidJS](https://www.solidjs.com/) for reactive UI and state (`createStore`, signals, and context)
- [Vite](https://vitejs.dev/) for dev server and production builds (`base: '/ace/'`)
- Plain CSS modules and a bundled Cascadia Code font

**Features**

- **Three targeting modes** plus a **Reverse** mode, cycled with *Tab* / *untab* hotkeys
  - `Circle`, `Figure Infinite`, and `Figure Eight` paths
  - `Reverse` for flipping direction
- **Adjustable speed**, with presets for Bloon Ace (`BACE_SPEED = 1`) and Sky Shredder (`SHREDDER_SPEED = 1.25`)
- **Playback controls:** Play / Pause, speed multiplier, and Reset
- **Event timeline** showing every tab, untab, and reverse, with timestamps
- **Savestates** to bookmark a moment and return to it later
- **BTD6 perspective** toggle that squishes the vertical axis by `cos(π/6)` to mimic the game's isometric view
- **Hotkey customization** for left, right, and reverse inputs
- **Save / load recordings** as `.csv` files, so you can replay or share runs

**Source layout**

| File | Responsibility |
| --- | --- |
| `src/App.jsx` | Main canvas render loop, UI panels, hotkeys, and CSV import/export |
| `src/Ace.jsx` | `AceProvider` context: ace state (position, heading, phase, targeting), tab/untab/reverse logic, and event store |
| `src/SimulationContext.jsx` | Shared simulation context |
| `src/EventsList.jsx` | Timeline of recorded events |
| `src/Background.jsx` | Background / track drawing |
| `src/Menu.jsx` | Play / Pause control |
| `src/utils/Constants.jsx` | Display size, ace radius `R`, turn limit, and speed constants |
| `src/utils/AngleUtils.js` | Angle normalisation helpers (`constrain_to_pi`, `constrain_to_tau`) |
| `src/utils/ColorUtils.js` | Colour interpolation helpers |

**Run it locally**

```bash
cd website
npm install
npm run dev        # http://localhost:3000
```

Other scripts: `npm run build` (output to `dist/`) and `npm run serve` (preview the build).

**Github Deployment**

Pushing to `main` triggers [`.github/workflows/main.yml`](./.github/workflows/main.yml), which:

1. Installs dependencies in `website/` with `npm ci`.
2. Runs `npm run build` to create `website/dist`.
3. Uploads `dist` as a GitHub Pages artifact and deploys it.

The site is served from `/ace/`, which matches the `base` path set in `website/vite.config.js`.


---

### Manim Animations

**Location:** [`./manim_stuff`](./manim_stuff)

The animations behind my ace micro explainer video, written with [Manim Community](https://www.manim.community/). Each `.py` file is a self-contained scene that draws an ace path, simulates its motion frame by frame, and animates the result at 60 FPS.

**Shared conventions**

- Distances are scaled from BTD6 units into Manim units with a single factor `x` (for example `R = 80 / x`, `r = 40 / x`), so a scene can be zoomed in or out by changing one number.
- Speed is controlled by `v_p` (base ace speed) and `v_c` (chasing ace speed, `1.2 * v_p`).
- The helper functions `constrain_to_pi` and `constrain_to_tau` are duplicated across scripts to keep each file standalone.
- Comments marked `# unchanged` flag constants carried over from the game.
- Matching `.csv` files hold the path samples (`x`, `y`, `phase`, `pathName`) for each scene.

**Scenes**

| Script | Notes |
| --- | --- |
| `scene.py` | General-purpose scene; the example command in the repo |
| `e.py` | Variant scene |
| `aqoa.py` | aqoa manoeuvre |
| `cp.py` / `cp2.py` | Two versions of the same Centered Path scene |
| `G.py` | Scene of intial takeoff |
| `infinite.py` | Figure-infinite targeting |
| `infiniteloop.py` / `infiniteloop2.py` | Infinite-loop variants (second file is a revised copy) |
| `plisp.py` | Uses timings from plisp to re-create a flight path |
| `reverse.py` | Shows the reversal manoeuvre |
| `snap.py` | Uses a `MovingCameraScene` to show when the ace snaps onto the path |
| `woor.py` | Wiggle out of range manoeuvre |

**Data files:** the `.csv` files store sampled simulation output (`x`, `y`, `phase`, `pathName`). `data.ipynb` is the exploratory notebook for them.

**Rendering a scene**

```bash
cd manim_stuff
manim -pqm scene.py CreateCircle --disable_caching
```

Flags: `-p` preview when done, `-q m` medium quality, `--disable_caching` forces a fresh render. Use `-qh` for high quality or `-ql` for a quick draft. Renders are written to `media/videos/<script>/<quality>/`.

---

### Sky Shredder Missile Research

**Location:** [`./missiles`](./missiles)

An analysis of how Sky Shredder missiles move relative to their targets.

- **`missiles.csv`** logs sampled positions over time with the columns `t` (tick), `id` (entity ID), `type` (`missile` or `target`), and `x`, `y`, `z` (world coordinates).
- **`missiles.ipynb`** uses [Polars](https://pola.rs/) and [Matplotlib](https://matplotlib.org/) to:
  1. De-duplicate and sort the records, then list the unique missile IDs.
  2. Plot each missile's `x`, `y`, and `z` against time.
  3. Estimate speed from frame-to-frame displacement.
  4. Compute heading with `np.arctan2` and unwrap it to track turning rate, converting to degrees per tick.
  5. Draw trajectories in 2D for a sample of missiles, overlaid with their targets.

To rerun the notebook you need `numpy`, `polars`, and `matplotlib`.

---

### Wingmonkey Targeting Research

**Research Results**: <https://youtu.be/Hp2s2J_aky0>

**Location:** [`./wingmonkey`](./wingmonkey)

Reverse-engineering of the **Wingmonkey** targeting option in Monkey Ace. The original game code is not available here. These files are my own reconstructions from decompiled output, so they may be inaccurate or incomplete.

| File | Description |
| --- | --- |
| `Process.py` | Python rewrite of the attack `Process` method: target validation, grouping, leader/follower logic, and the per-frame delta time |
| `updatemovement.py` | Python rewrite of `UpdateMovement`: chooses between chasing a bloon and following a flyover or group position, and checks whether the overshoot point is still on the path |
| `UpdateMovement.c` | Raw Ghidra decompilation of `WingmonkeyPattern.UpdateMovement`, which the Python version was based on |
| `data.txt` | Timestamped numeric log captured while testing the behaviour |

---

