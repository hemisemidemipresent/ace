// display
const SIZE = 720
const DOT_SIZE = 12
const SCALE = 3;

// mathematical constants
const PI = Math.PI;
const TAU = 2 * Math.PI;

const TICKS_PER_MS = 60 / 1000; // 60 ticks per second (in theory)
const R = 80 * SCALE;
const v = 1 * SCALE; // for shredder change this
const v_c = v * 1.2;
const MAX_TURN_PER_TICK = PI / 30;
const OMEGA = v / R;

const TARGETING = {
    CIRCLE: 0,
    INFINITY: 1,
    EIGHT: 2
}

const TARGETING_NAMES = ['Circle', 'Figure Infinite', 'Figure Eight', 'Reverse']

export {
    TICKS_PER_MS,
    PI, TAU, SCALE, R, v, v_c, MAX_TURN_PER_TICK, OMEGA, SIZE, DOT_SIZE, TARGETING, TARGETING_NAMES
}