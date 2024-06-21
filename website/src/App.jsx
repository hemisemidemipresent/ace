// import logo from './logo.svg';
// import styles from './App.module.css';
import Background from './Background';
import EventsList from './EventsList';
import { createSignal, onMount, onCleanup, createEffect, Show } from "solid-js";

import { useAceContext } from './Ace'

import {
    TICKS_PER_MS,
    PI, TAU, SCALE, R, v, v_c, MAX_TURN_PER_TICK, OMEGA, SIZE, DOT_SIZE, TARGETING
} from './utils/Constants'
import { interpolateRainbow } from './utils/ColorUtils'


const red = '#FC1354'
const yellow = '#FDDE5A'
const green = '#0FFEAA'
const blue = '#3DB9F6'

function dot(ctx, x, y, dotsize) {
    x += SIZE / 2
    y += SIZE / 2
    ctx.fillRect(x - dotsize / 2, y - dotsize / 2, dotsize, dotsize);
}

function line(ctx, x1, y1, x2, y2) {
    x1 += SIZE / 2
    y1 += SIZE / 2
    x2 += SIZE / 2
    y2 += SIZE / 2
    ctx.beginPath()
    ctx.moveTo(x1, y1)
    ctx.lineTo(x2, y2)
    ctx.stroke()
}

function App() {
    const { aceState, setAceState, tab, untab, reverse, simulate, circlePos, figureEightPos, figureInfinitePos, targetPoint, events, setEvents, setDefault } = useAceContext();

    let [time, setTime] = createSignal(0);

    // canvas
    let canvas, historyCanvas;
    let ctx, history_ctx;
    onMount(() => {
        ctx = canvas.getContext("2d");
        ctx.strokeStyle = '#fff'
        ctx.lineWidth = DOT_SIZE / 2
        history_ctx = historyCanvas.getContext("2d");
        history_ctx.fillStyle = red
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

        if (dt > 150) return accumulatedPausedTime += dt; // window was probably out of focus so basically treat as if it is paused

        setEstimatedFPS(1000 / dt)

        let dtick = dt * TICKS_PER_MS;
        setTime(t);

        ctx.clearRect(0, 0, canvas.width, canvas.height);

        ctx.fillStyle = blue;


        simulate(dtick, time())

        dot(ctx, circlePos().x, circlePos().y, DOT_SIZE)
        dot(ctx, figureEightPos().x, figureEightPos().y, DOT_SIZE)
        dot(ctx, figureInfinitePos().x, figureInfinitePos().y, DOT_SIZE)



        if (stampHistory) dot(history_ctx, aceState.x, aceState.y, DOT_SIZE / 2.5)

        line(ctx, targetPoint().x, targetPoint().y, aceState.x, aceState.y)

        ctx.fillStyle = green
        if (!aceState.onPath) dot(ctx, aceState.x, aceState.y, DOT_SIZE)


        // dot(ctx, aceState.x + 10*Math.cos(aceState.theta), aceState.y+10 * Math.sin(aceState.theta), DOT_SIZE / 2) // debugging aceState.theta


    }



    function clearHistory() {
        history_ctx.clearRect(0, 0, historyCanvas.width, historyCanvas.height)
    }


    function reset() {
        accumulatedPausedTime += time();
        setTime(0);

        clearHistory()

        setDefault() // set initial conditions to default

        // set events to all be un-completed
        // https://docs.solidjs.com/concepts/stores#range-specification
        // https://docs.solidjs.com/concepts/stores#dynamic-value-assignment
        setEvents({ from: 0, to: events.length - 1 }, "completed", false)

        // sort events (some events)
        setEvents(events => [...events].sort((a, b) => a.time - b.time)); // TODO: make this more performant
    }

    addEventListener("keydown", (event) => {
        if (event.target.tagName == 'INPUT') return // the user is setting a hotkey instead

        event.preventDefault();

        if (event.code == 'Space') return toggle()

        if (event.code == leftHotkey()) {
            untab(time())

        } else if (event.code == rightHotkey()) {
            tab(time())
        }
        else if (event.code == reverseHotkey()) {
            reverse(time())
        }
        else return;


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
            <div id="events_container" class='bubble'>
                <h1>Events</h1>
                <p>Remember to press <span class='blue'>Reset</span> after editing the timings</p>
                <EventsList />
            </div>
            <div class="layered">
                <Background />
                <canvas ref={historyCanvas} width={SIZE} height={SIZE} />
                <canvas ref={canvas} width={SIZE} height={SIZE} />
            </div>
            <div id='right_column' class='flexbox flexbox-vert'>

                <div class='bubble'>
                    <h2>Info</h2>
                    {/* <p>phase: <span class='green'> {(aceState.phase * 8 / TAU).toFixed(3)} </span> hemidemisemicycles</p> */}
                    <p>phase: <span style={`color:rgb(${interpolateRainbow(aceState.phase)})`}> {(aceState.phase * 8 / TAU).toFixed(3)} </span> hemidemisemicycles</p>
                    <p>time: {Math.round(time())} ms</p>
                    <p>estimated fps: {Math.round(estimatedFPS())} fps</p>
                </div>
                <div class="bubble">
                    <h2>Settings</h2>
                    <div class='flexbox-start flexbox-vert mb-1'>
                        <Show when={paused()} fallback={<button onClick={toggle} class='red border-red'>Pause</button>}>
                            <button onClick={toggle} class='green border-green'>Play</button>
                        </Show>
                        <button onClick={clearHistory}>Clear History</button>
                        <button onClick={reset} class='blue border-blue'>Reset</button>
                        <div class='flexbox'>
                            <p class='m-0'>Stamp History:</p>
                            <label class="switch">
                                <input type="checkbox" checked onChange={(e) => stampHistory = !stampHistory} />
                                <span class="slider round" />
                            </label>
                        </div>
                    </div>
                    
                </div>
                <div class='bubble'>
                    <h2>Hotkeys</h2>
                    <p>"Tab" Hotkey: <input type="text" onKeyDown={updateRightKey} value={rightHotkey()} placeholder="Press your hotkey" readOnly /></p>
                    <p>"untab" Hotkey: <input type="text" onKeyDown={updateLeftKey} value={leftHotkey()} placeholder="Press your hotkey" readOnly /></p>
                    <p>Reverse Hotkey: <input type="text" onKeyDown={updateReverseKey} value={reverseHotkey()} placeholder="Press your hotkey" readOnly /></p>
                    <p>Don't set them to the same hotkey I didn't bother to code stuff for that</p>
                </div>
                <div class='bubble'>
                    <h2>Credits</h2>
                    <p>This website was made with <a href="https://www.solidjs.com/">Solid JS</a></p>
                    <p>I kinda took <span class='red'>this</span> <span class='yellow'>color</span> <span class='green'>scheme</span> from <a href="https://www.youtube.com/@acegikmo">Freya Holmér</a></p>
                    <p>Source code can be found <a href="https://github.com/hemisemidemipresent/ace/tree/main/website">here</a></p>
                </div>
            </div>
        </div>

    </>;
}

export default App;