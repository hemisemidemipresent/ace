import { TAU } from './Constants.jsx'


// modified from https://github.com/d3/d3-scale-chromatic/blob/main/src/sequential-multi/rainbow.js
function interpolateRainbow(t) {
    t /= TAU // t ranges from 0 - 1
    t = (t + 0.2) % 1;
    let ch = cubehelix(
      360 * t - 100, // convert to degrees + rainbow shift
      2 - 1.5 * Math.abs(t - 0.5),
      0.9 - 0.8 * Math.abs(t - 0.5)
    );

    return ch.join(',')
}

// adapted from https://github.com/d3/d3-color/blob/main/src/cubehelix.js
// h - angle (degrees)
// s - hue
// l - frac
function cubehelix(angle, hue, frac) {

    angle += 120 // angle shift
    angle *= TAU/360 // convert back to radians


    frac = +frac;
    const amp = hue * frac * (1 - frac);

    // Calculate color intensities
    const cos_a = Math.cos(angle);
    const sin_a = Math.sin(angle);

    const r = frac + amp * (-0.14861 * cos_a + 1.78277 * sin_a);
    const g = frac + amp * (-0.29227 * cos_a - 0.90649 * sin_a);
    const b = frac + amp * (+1.97294 * cos_a);

    // Apply gamma correction and clamp to [0, 1]
    // const gammaCorrected = c => Math.pow(c, gamma);
    // const clamp = c => Math.max(0, Math.min(1, c));

    return [
        Math.floor(r * 255),
        Math.floor(g * 255),
        Math.floor(b * 255)
    ]
}

export { interpolateRainbow }