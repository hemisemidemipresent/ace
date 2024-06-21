import { createSignal, onMount, onCleanup, createEffect, Show } from "solid-js";

import {TARGETING_NAMES} from './utils/Constants'
import { useAceContext } from "./Ace";

function EventsList(props) {
    const { events, setEvents} = useAceContext()
    return <For each={events}>
        {(event, i) => {
            return <div class={ event.completed ? "event_completed": ""}>
                <p>{TARGETING_NAMES[event.targeting]}</p>
                <p>time: <input type="number" value={Math.round(event.time)} onChange={(e)=>{
                    setEvents(i(), "time", e.target.value)
                }}></input></p>
            </div>
        }}
    </For>
}

export default EventsList