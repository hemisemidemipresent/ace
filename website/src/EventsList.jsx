import { produce } from "solid-js/store";

import { TARGETING_NAMES } from './utils/Constants.jsx'
import { useAceContext } from "./Ace";

function EventsList(props) {
    const { events, setEvents } = useAceContext()
    return <>
    <Show when={events.length > 0}>
        <div class="flexbox-start flexbox-vert mb-1">
        <button onClick={()=>{
            setEvents([])
        }} class='red border-red'>Clear Events</button>
        <button onClick={()=>{
            const compressed = JSON.stringify(events.map(event => [event.targeting, event.time]));
            navigator.clipboard.writeText(`${location.protocol}//${location.host}${location.pathname}?events=${btoa(compressed)}`);
            alert('URL exported to clipboard!')
            
        }} class='yellow border-yellow'>Export Events</button>
        </div>
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
                    }}>
                        <b>-</b>
                    </button>
                
                    </div>
                
            </div>
        }}
    </For>
    </>
}

export default EventsList