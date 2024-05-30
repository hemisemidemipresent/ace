import { createSignal, onMount, onCleanup } from "solid-js";

function Menu() {

    const [paused, setPaused] = createSignal(false);
    const toggle = () => setPaused(!paused())

    return (
        <>
            <Show when={paused()} fallback={<button onClick={toggle}>Pause</button>}>
                <button onClick={toggle}>Play</button>
            </Show>
            
        </>
    );
}

export default Menu;
