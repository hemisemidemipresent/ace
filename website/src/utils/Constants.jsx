import { createSignal } from "solid-js";

// display
// const SIZE = 720
const SIZE = 1080
const DOT_SIZE = 12
const SCALE = 3;

// mathematical constants
const PI = Math.PI;
const TAU = 2 * Math.PI;

// ace/btd6 constants
const TICKS_PER_MS = 60 / 1000; // 60 simulation ticks per second (in theory)
const R = 80 * SCALE;
const MAX_TURN_PER_TICK = PI / 30;


// speed and derived quantities
const [speed, setSpeed] = createSignal(1)
const v = () => speed() * SCALE; // for shredder change this to 1.5*
const v_c = () => v() * 1.2;
const omega = () => v() / R;

const TARGETING = {
    CIRCLE: 0,
    INFINITY: 1,
    EIGHT: 2
}

const TARGETING_NAMES = ['Circle', 'Figure Infinite', 'Figure Eight', 'Reverse']

export {
    SIZE, DOT_SIZE, SCALE,
    PI, TAU,
    TICKS_PER_MS, R, MAX_TURN_PER_TICK,
    v, v_c, omega,
    TARGETING, TARGETING_NAMES,
    speed, setSpeed
}