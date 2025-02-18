// import logo from './logo.svg';
// import styles from './App.module.css';
import Background from './Background';
import EventsList from './EventsList';
import { createSignal, onMount, onCleanup, Show } from "solid-js";

import { useAceContext } from './Ace'

import { TICKS_PER_MS, PI, TAU, SIZE, DOT_SIZE, omega, speed, setSpeed, TARGETING_NAMES_TO_NUMBER, BACE_SPEED ,SHREDDER_SPEED } from './utils/Constants.jsx'
import { interpolateRainbow } from './utils/ColorUtils'


const red = '#FC1354'
const yellow = '#FDDE5A'
const green = '#0FFEAA'
const blue = '#3DB9F6'



function App() {
    const { aceState, tab, untab, reverse, simulate, circlePos, figureEightPos, figureInfinitePos, targetPoint, events, setEvents, sortEvents, setDefault } = useAceContext();

    const [btd6perspective, setBTD6perspective] = createSignal(false)

    const squishfactor = () => { return btd6perspective() ? Math.cos(Math.PI / 6) : 1 }

    function dot(ctx, x, y, dotsize) {
        y *= squishfactor()
        x += SIZE / 2
        y += SIZE / 2
        ctx.fillRect(x - dotsize / 2, y - dotsize / 2, dotsize, dotsize);
    }

    function line(ctx, x1, y1, x2, y2, color, width = DOT_SIZE / 2) {

        y1 *= squishfactor()
        y2 *= squishfactor()

        x1 += SIZE / 2
        y1 += SIZE / 2
        x2 += SIZE / 2
        y2 += SIZE / 2

        ctx.strokeStyle = color
        ctx.lineWidth = width

        ctx.beginPath()
        ctx.moveTo(x1, y1)
        ctx.lineTo(x2, y2)
        ctx.stroke()
    }

    function arrow(ctx, x, y, theta, length = DOT_SIZE * 4, color = 'whitesmoke', width = DOT_SIZE / 4) {

        let tip_x = x + length * Math.cos(theta)
        let tip_y = y + length * Math.sin(theta)

        line(ctx, x, y, tip_x, tip_y, color, width)

        // triangle
        tip_y *= squishfactor()
        tip_x += SIZE / 2
        tip_y += SIZE / 2

        const TRIANGLE_SIDE = DOT_SIZE;
        let x1 = tip_x + TRIANGLE_SIDE * Math.cos(theta - 5 / 6 * PI)
        let y1 = tip_y + TRIANGLE_SIDE * Math.sin(theta - 5 / 6 * PI)
        let x2 = tip_x + TRIANGLE_SIDE * Math.cos(theta + 5 / 6 * PI)
        let y2 = tip_y + TRIANGLE_SIDE * Math.sin(theta + 5 / 6 * PI)

        ctx.fillStyle = color
        ctx.beginPath();
        ctx.moveTo(tip_x, tip_y);
        ctx.lineTo(x1, y1);
        ctx.lineTo(x2, y2);
        ctx.fill();
    }

    let [time, setTime] = createSignal(0);

    // canvas
    let canvas, historyCanvas;
    let ctx, history_ctx;
    onMount(() => {

        const searchParams = new URLSearchParams(window.location.search)
        const data = searchParams.get('events')
        if (data) {
            let compressedEvents = JSON.parse(atob(data))
            let parsedEvents = compressedEvents.map(arr => {
                return { targeting: arr[0], time: arr[1], completed: false }
            })
            setEvents(parsedEvents)
        }

        ctx = canvas.getContext("2d");
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

        if (dt > 50) return accumulatedPausedTime += dt; // window was probably out of focus so basically treat as if it is paused

        setEstimatedFPS(1000 / dt)

        let dtick = dt * TICKS_PER_MS;
        setTime(t);

        ctx.clearRect(0, 0, canvas.width, canvas.height);

        simulate(dtick, time())

        ctx.fillStyle = blue;
        dot(ctx, circlePos().x, circlePos().y, DOT_SIZE)
        dot(ctx, figureEightPos().x, figureEightPos().y, DOT_SIZE)
        dot(ctx, figureInfinitePos().x, figureInfinitePos().y, DOT_SIZE)



        if (stampHistory) dot(history_ctx, aceState.x, aceState.y, DOT_SIZE / 2.5)

        line(ctx, targetPoint().x, targetPoint().y, aceState.x, aceState.y, yellow)

        ctx.fillStyle = green
        if (!aceState.onPath) dot(ctx, aceState.x, aceState.y, DOT_SIZE)
        // dot(ctx, aceState.x + 10*Math.cos(aceState.theta), aceState.y+10 * Math.sin(aceState.theta), DOT_SIZE / 2) // debugging aceState.theta

        arrow(ctx, aceState.x, aceState.y, aceState.theta)
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

        // sort events
        sortEvents()
    }

    addEventListener("keydown", (event) => {
        if (event.repeat) return; // ignore held down keys spamming events
        
        if (event.target.tagName == 'INPUT' && event.target.placeholder == "Press your hotkey") return // the user is setting a hotkey instead

        if (event.code == 'Space') toggle()
        else if (event.code == leftHotkey())
            untab(time())
        else if (event.code == rightHotkey())
            tab(time())
        else if (event.code == reverseHotkey())
            reverse(time())
        else return;

        event.preventDefault();

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


    function changeAce(newSpeed) {
        // attempt to re-calibrate events
        setEvents({ from: 0, to: events.length - 1 }, "time", time => time * speed() / newSpeed)

        setSpeed(newSpeed)
        reset()
    }



    return <>
        <div class='flexbox'>
            <div id="events_container" class='bubble'>
                <h1>Events</h1>
                <p>Targeting changes will be recorded here. When you press <span class='blue'>Reset</span>, these targeting changes will be replayed accordingly.</p>
                <EventsList />
            </div>
            <div id='canvas-container'>
                <div class="layered">
                    <Background squishfactor={squishfactor()} />
                    <canvas ref={historyCanvas} width={SIZE} height={SIZE} />
                    <canvas ref={canvas} width={SIZE} height={SIZE} />
                </div>
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
                        <div class='w-20'>Change Ace Type <br /> (this will <span class='blue'>Reset</span>, and attempt to change event timings accordingly)</div>
                        <div class='flexbox gap-half'>
                            <input type="radio" name="fav_language" value={BACE_SPEED} checked={speed() == BACE_SPEED} onChange={(e) => { changeAce(BACE_SPEED) }} />
                            <p class='m-0'>Base Ace</p>
                        </div>
                        <div class='flexbox gap-half'>
                            <input type="radio" name="fav_language" value={SHREDDER_SPEED} checked={speed() == SHREDDER_SPEED} onChange={(e) => { changeAce(SHREDDER_SPEED) }} />
                            <p class='m-0'>Shredder</p>
                        </div>
                        <div class='flexbox'>
                            <p class='m-0'>BTD6 perspective:</p>
                            <label class="switch">
                                <input type="checkbox" onChange={(e) => { setBTD6perspective(btd6perspective => !btd6perspective); reset() }} />
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
                <div class='bubble'>
                    <h2>Import from POFV</h2>
                    <label for="file-upload" class="custom-file-upload">Upload AcePositions.csv</label>
                    <input type="file" id="file-upload" accept=".csv" onClick={(e) => e.target.value = ''} onChange={(e) => {
                        let file = e.target.files[0]
                        const reader = new FileReader();

                        // gets triggered when we read the file
                        reader.addEventListener('load', (event) => {
                            const contents = event.target.result;
                            const lines = contents.split('\n')

                            const initTokens = lines[1].split(',')

                            let fileEvents = []

                            let currentPhase = parseFloat(initTokens[2]) // phase here is 0 - 1
                            let currentTargeting = initTokens[3] // stored as strings e.g. 'Circle'

                            fileEvents.push({ targeting: TARGETING_NAMES_TO_NUMBER[currentTargeting], time: 0, completed: false })


                            // omega is radians per "frame" where "frame" is 1/60 s
                            let period = TAU / omega() / 60;

                            let time = (1 + currentPhase) * period // assume the ace already went one round
                            let reverse = 1;

                            // guessimate velocity and setSpeed accordingly
                            let [x1, y1, phase1] = lines[1].split(',')
                            let [x2, y2, phase2] = lines[2].split(',')

                            if (phase2 - phase1 > 0.9) phase2 += 1 // I hate edge cases where phase goes from 0.99 -> 0.01
                            else if (phase1 - phase2 > 0.9) phase1 += 1 // reverse! edge case where phase goes from 0.01 -> 0.99

                            let est_v = Math.sqrt((x1-x2) ** 2 + (y1-y2) ** 2) * omega() / ((phase2 - phase1) * TAU)
                            console.log(est_v)
                            const TOLERANCE = 0.05

                            if (Math.abs(est_v - BACE_SPEED) < TOLERANCE) setSpeed(1) // base ace on path
                            else if (Math.abs(est_v - 1.2 * BACE_SPEED) < TOLERANCE) setSpeed(1) // base ace off path
                            else if (Math.abs(est_v - SHREDDER_SPEED) < TOLERANCE) setSpeed(1.2) // shredder on path
                            else if (Math.abs(est_v - 1.2 * SHREDDER_SPEED) < TOLERANCE) setSpeed(1.2) // shredder off path

                            for (let i = 2; i < lines.length - 1; i++) {
                                let [x, y, phase, targeting] = lines[i].split(',')
                                phase = parseFloat(phase)

                                if (Math.abs(phase - currentPhase) > 0.9) {
                                    time += (phase + reverse - currentPhase) * period // gaming
                                }
                                else {
                                    time += (phase - currentPhase) * period

                                    // reverse
                                    if (currentPhase > phase * reverse) {
                                        fileEvents.push({ targeting: 3, time: time * 1000, completed: false })
                                    }
                                }

                                if (targeting != currentTargeting) {
                                    fileEvents.push({ targeting: TARGETING_NAMES_TO_NUMBER[targeting], time: time * 1000, completed: false })
                                    currentTargeting = targeting;
                                }

                                currentPhase = phase

                            }

                            setEvents(fileEvents)
                            reset()
                        });

                        // actually read the file
                        reader.readAsText(file);
                    }} />
                </div>
            </div>
        </div>

    </>;
}

export default App;