import { produce } from "solid-js/store";

import { TARGETING_NAMES } from './utils/Constants.jsx'
import { useAceContext } from "./Ace";

function EventsList(props) {
    const { events, setEvents } = useAceContext()
    return <>
    <Show when={events.length > 0}>
        <button onClick={()=>{
            setEvents([])
        }} class='red border-red mb-1'>Clear Events</button>
    </Show>
    <For each={events}>
        {(event, i) => {
            return <div class={ event.completed ? "event_completed": ""}>
                <div class="flexbox gap-half">
                    <p class="m-0">{TARGETING_NAMES[event.targeting]}</p>
                    <button class="squareBtn border-thin" onClick={(e) => {
                        setEvents(i(), "targeting", (event.targeting + 1) % 4)
                    }}>▶</button>
                </div>
                <div class="flexbox">
                    <p>time: </p>
                    <input type="number" value={Math.round(event.time)} onChange={(e) => {
                        setEvents(i(), "time", e.target.value)
                    }}></input>
                    <button class='squareBtn red border-red' onClick={(e) => {
                        setEvents(
                            produce((events) => {
                                events.splice(i(), 1);
                            })
                        )
                    }}>-</button>
                
                    </div>
                
            </div>
        }}
    </For>
    </>
}

export default EventsList