import { produce } from "solid-js/store";

import { TARGETING_NAMES } from './utils/Constants.jsx'
import { useAceContext } from "./Ace";
import { useSimulationContext } from "./SimulationContext.jsx";

function EventsList(props) {
    const { events, setEvents, setAceState } = useAceContext()
    const { time, setTime, paused, setPaused } = useSimulationContext()
    return <>
    <Show when={events.length > 0}>
        <div class="flexbox-start flexbox-vert mb-1">
        <button onClick={()=>{
            setEvents([])
        }} class='red border-red'>Clear Events</button>
        <button onClick={()=>{
            const compressed = JSON.stringify(events.map(event => [event.eventType, event.time]));
            navigator.clipboard.writeText(`${location.protocol}//${location.host}${location.pathname}?events=${btoa(compressed)}`);
            alert('URL exported to clipboard!')
            
        }} class='yellow border-yellow'>Export Events</button>
        </div>
    </Show>
    <For each={events}>
        {(event, i) => {

            if (event.eventType == 4) {
                return <div>
                  <div class="flexbox gap-half">
                    <p class="m-0 font-bold yellow">Save State</p>
                    <button class="squareBtn border-thin" onClick={(e) => {
                        setAceState(event.savedAceState)

                        // sets all the remaining events to completed: false
                        setEvents({ from: i(), to: events.length - 1 }, "completed", false)
                        // go back in time
                        setTime(event.time)
                    }}>▶</button>
                  </div>
                    <div class="flexbox">
                        <p>time: {Math.round(event.time)}</p>
                        <button class='squareBtn red border-red' onClick={(e) => {
                            setEvents(
                                produce((events) => {
                                    events.splice(i(), 1);
                                })
                            )
                        }}>
                            <b>x</b>
                        </button>
                    </div>
                </div>
            }

            return <div class={ event.completed ? "event_completed": ""}>
                <div class="flexbox gap-half">
                    <p class="m-0">{TARGETING_NAMES[event.eventType]}</p>
                    <button class="squareBtn border-thin" onClick={(e) => {
                        setEvents(i(), "eventType", (event.eventType + 1) % 4)
                    }}>-&gt;</button>
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
                        <b>x</b>
                    </button>
                
                </div>
                
            </div>
        }}
    </For>
    </>
}

export default EventsList