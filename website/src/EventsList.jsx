import { createSignal, onMount, onCleanup, createEffect, Show } from "solid-js";

import {TARGETING_NAMES} from './Constants'

function EventsList(props) {

    return <For each={props.events()}>
        {(event) => {
            return <div>
                <p>{TARGETING_NAMES[event.targeting]}</p>
            </div>
        }}
    </For>
}

export default EventsList