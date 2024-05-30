// import logo from './logo.svg';
// import styles from './App.module.css';
import Background from './Background';
import EventsList from './EventsList';
import { createSignal, onMount, onCleanup, createEffect, Show } from "solid-js";

import {
    TICKS_PER_MS,
    PI, TAU, SCALE, R, v, v_c, MAX_TURN_PER_TICK, OMEGA, SIZE, DOT_SIZE, TARGETING
} from './Constants'

let ace = { x: R * 3 / 8, y: 0, theta: 0 };
let onPath = false;



function constrain_to_pi(x) {
    return (x + TAU + PI) % TAU - PI
}

function constrain_to_tau(x) {
    return (x + TAU) % TAU
}

function dot(ctx, x, y, dotsize) {
    x += SIZE / 2
    y += SIZE / 2
    ctx.fillRect(x - dotsize / 2, y - dotsize / 2, dotsize, dotsize);
}





function App() {
    let [time, setTime] = createSignal(0);
    const [phase, setPhase] = createSignal(0); // phase is in radians // might make this into context




    const circlePos = () => {
        let x = R * Math.cos(phase());
        let y = R * Math.sin(phase());
        return { x, y };
    };
    const figureEightPos = () => {
        if (phase() <= PI) {
            let x = R / 2 * Math.cos(phase() * 2 + PI / 2);
            let y = R / 2 * Math.sin(phase() * 2 + PI / 2);
            y -= R / 2;
            return { x, y };
        }
        let x = -R / 2 * Math.cos((phase() * 2 - TAU) - PI / 2);
        let y = R / 2 * Math.sin((phase() * 2 - TAU) - PI / 2);
        y += R / 2;
        return { x, y };

    };
    const figureInfinitePos = () => {
        if (phase() <= PI) {
            let x = R / 2 * Math.cos(phase() * 2 + PI);
            let y = R / 2 * Math.sin(phase() * 2 + PI);
            x += R / 2;
            return { x, y };
        }
        let x = R / 2 * Math.cos((phase() * 2 - TAU));
        let y = -R / 2 * Math.sin((phase() * 2 - TAU));
        x -= R / 2;
        return { x, y };

    };

    const [targeting, setTargeting] = createSignal(TARGETING.CIRCLE);
    const targetPoint = () => {
        if (targeting() == TARGETING.CIRCLE) return circlePos();
        if (targeting() == TARGETING.EIGHT) return figureEightPos();
        if (targeting() == TARGETING.INFINITY) return figureInfinitePos();
    }
    const [reverse, setReverse] = createSignal(1); // 1 - normal; -1 - reverse

    // canvas
    let canvas, historyCanvas;
    let ctx, history_ctx;
    onMount(() => {
        ctx = canvas.getContext("2d");
        history_ctx = historyCanvas.getContext("2d");
        history_ctx.fillStyle = '#ffff00' // yellow
        let frame = requestAnimationFrame(loop);
        onCleanup(() => cancelAnimationFrame(frame));
    });

    // animation/simulation loop
    let stampHistory = true;
    const [estimatedFPS, setEstimatedFPS] = createSignal(0);

    const [paused, setPaused] = createSignal(false);
    const toggle = () => setPaused(!paused());
    let accumulatedPausedTime = 0;
    function loop(t) {

        requestAnimationFrame(loop);

        if (paused()) return accumulatedPausedTime = t - time();

        t -= accumulatedPausedTime;

        let dt = t - time()

        if (dt > 500) return accumulatedPausedTime += dt; // window was probably out of focus so basically treat as if it is paused

        setEstimatedFPS(1000/dt)

        let dtick = dt * TICKS_PER_MS;
        setTime(t);
        setPhase(phase => constrain_to_tau(phase + OMEGA * dtick * reverse()))

        ctx.clearRect(0, 0, canvas.width, canvas.height);

        ctx.fillStyle = "#00ff00";
        dot(ctx, circlePos().x, circlePos().y, DOT_SIZE)
        dot(ctx, figureEightPos().x, figureEightPos().y, DOT_SIZE)
        dot(ctx, figureInfinitePos().x, figureInfinitePos().y, DOT_SIZE)

        let target = targetPoint();

        if (onPath) {
            ace.x = target.x
            ace.y = target.y

            if (targeting() == TARGETING.CIRCLE)
                ace.theta = -PI / 2 - phase()
            else if (targeting() == TARGETING.EIGHT) {
                if (phase() < PI)
                    ace.theta = PI - 2 * phase()
                else
                    ace.theta = 2 * (phase() - PI) - PI
            }
            else if (targeting() == TARGETING.INFINITY) {
                if (phase() < PI)
                    ace.theta = PI / 2 - 2 * phase()
                else
                    ace.theta = 2 * phase() + PI / 2
            }
            else throw new Error('WTF')
            // temp
            ace.theta = -ace.theta

            if (reverse() == -1) ace.theta += PI;

            // dot(ctx, ace.x + 10*Math.cos(ace.theta), ace.y+10 * Math.sin(ace.theta), DOT_SIZE / 2) // debugging ace.theta

        }
        else {
            let delta_x = target.x - ace.x;
            let delta_y = target.y - ace.y;

            let angle = Math.atan2(delta_y, delta_x)
            let angle_diff = constrain_to_pi(angle - ace.theta)


            if (Math.abs(angle_diff) < MAX_TURN_PER_TICK * dtick)
                ace.theta += Math.sign(angle_diff) * angle_diff
            else
                ace.theta += Math.sign(angle_diff) * MAX_TURN_PER_TICK * dtick
            ace.theta = constrain_to_pi(ace.theta)

            let step = v_c * dtick;

            ace.x += step * Math.cos(ace.theta);
            ace.y += step * Math.sin(ace.theta);

            let dist_squared = (target.x - ace.x) ** 2 + (target.y - ace.y) ** 2

            if (dist_squared < 1) onPath = true;
        }

        if (stampHistory) dot(history_ctx, ace.x, ace.y, DOT_SIZE / 2)
        ctx.fillStyle = onPath ?'#00ccff': '#ff0000'
        dot(ctx, ace.x, ace.y, DOT_SIZE)

        dot(ctx, ace.x + 10*Math.cos(ace.theta), ace.y+10 * Math.sin(ace.theta), DOT_SIZE / 2) // debugging ace.theta


    }

    function clearHistory() {
        history_ctx.clearRect(0, 0, historyCanvas.width, historyCanvas.height)
    }

    const [events, setEvents] = createSignal([])

    addEventListener("keydown", (event) => {
        if (event.target.tagName == 'INPUT') return // the user is setting a hotkey instead

        event.preventDefault();

        if (event.code == 'Space') return toggle()

        if (event.code == leftHotkey()) {
            setTargeting((targeting() + 2) % 3)
            setEvents([...events(), {targeting: targeting(), time: time(), completed: true}])
        } else if (event.code == rightHotkey()) {
            setTargeting((targeting() + 1) % 3)
            setEvents([...events(), {targeting: targeting(), time: time(), completed: true}])
        }
        else if (event.code == reverseHotkey()) {
            setReverse(-reverse());
            setPhase(phase() + PI / 5 * reverse())
            setEvents([...events(), {targeting: 3, time: time(), completed: true}])
        }
        else return;

        // only if a mutation actually happened
        onPath = false;
        
    });

    const defaultHotkeys = {
        LEFT: 'KeyQ',
        RIGHT: 'Tab',
        REVERSE: 'KeyS'
    }

    // im sure there is a better way to do this reactivity
    const [leftHotkey, setLeftHotkey] = createSignal(defaultHotkeys.LEFT);
    const [rightHotkey, setRightHotkey] = createSignal(defaultHotkeys.RIGHT);
    const [reverseHotkey, setReverseHotkey] = createSignal(defaultHotkeys.REVERSE);

    const updateLeftKey = (e) => {
        e.preventDefault();
        setLeftHotkey(e.code);
    };
    const updateRightKey = (e) => {
        e.preventDefault();
        setRightHotkey(e.code);
    };
    const updateReverseKey = (e) => {
        e.preventDefault();
        setReverseHotkey(e.code);
    };

    return <>
        <div class='flexbox'>
            <div id="events_container">
                <h1>Events</h1>
                <EventsList events={events} />
            </div>
            <div class="layered">
                <Background />
                <canvas ref={historyCanvas} width={SIZE} height={SIZE} />
                <canvas ref={canvas} width={SIZE} height={SIZE} />
            </div>
            <div >
                <Show when={paused()} fallback={<button onClick={toggle}>Pause</button>}>
                    <button onClick={toggle}>Play</button>
                </Show>
                <button onClick={clearHistory}>Clear Path</button>
                <p>phase: {(phase() * 8 / TAU).toFixed(3)} hemidemisemicycles</p>
                <p>time: {time()} ms</p>
                <p>estimated fps: {Math.round(estimatedFPS())} fps</p>
                <p>Stamp History: <input type="checkbox" checked onChange={(e)=>stampHistory=!stampHistory}></input></p>
                <div>
                    <p>"Tab" Hotkey: <input type="text" onKeyDown={updateLeftKey} value={rightHotkey()} placeholder="Press your hotkey" readOnly /></p>
                    <p>"untab" Hotkey: <input type="text" onKeyDown={updateRightKey} value={leftHotkey()} placeholder="Press your hotkey" readOnly /></p>
                    <p>Reverse Hotkey: <input type="text" onKeyDown={updateReverseKey} value={reverseHotkey()} placeholder="Press your hotkey" readOnly /></p>
                </div>
            </div>
        </div>
        
    </>;
}

export default App;
