import { PI, TAU } from './Constants.jsx'

function constrain_to_pi(x) {
    return (x + TAU + PI) % TAU - PI
}

function constrain_to_tau(x) {
    return (x + TAU) % TAU
}
export {
    constrain_to_pi, constrain_to_tau
}