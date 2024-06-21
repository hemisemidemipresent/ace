import {
    TICKS_PER_MS,
    PI, TAU, SCALE, R, v, v_c, MAX_TURN_PER_TICK, OMEGA, SIZE, DOT_SIZE, TARGETING
} from './Constants'

function constrain_to_pi(x) {
    return (x + TAU + PI) % TAU - PI
}

function constrain_to_tau(x) {
    return (x + TAU) % TAU
}
export {
    constrain_to_pi, constrain_to_tau
}