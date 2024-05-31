import { createSignal, onMount, onCleanup, createEffect, Show } from "solid-js";

import {TARGETING_NAMES} from './Constants'

function EventsList(props) {
    return <For each={props.events}>
        {(event) => {
            console.log('a')
            return <div class={event.completed?"event_completed":""}>
                <p>{TARGETING_NAMES[event.targeting]}</p>
                <p>time: <input type="number" value={event.time}></input></p>
            </div>
        }}
    </For>
}

export default EventsList