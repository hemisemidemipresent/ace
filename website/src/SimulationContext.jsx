import { createContext, useContext } from "solid-js";
import { createSignal } from "solid-js";

const SimulationContext = createContext();
export function SimulationProvider(props) {

    // see loop() and reset() in Ace.jsx to see how the timey wimey stuff works
    const [realTime, setRealTime] = createSignal(0); // real time
    const [time, setTime] = createSignal(0); // simulation time

    const [simSpeed, setSimSpeed] = createSignal(1);
    const [paused, setPaused] = createSignal(false);

    return (
        <SimulationContext.Provider value={
          {
            time, setTime, realTime, setRealTime,
            simSpeed, setSimSpeed, paused, setPaused
          }
        }>
          {props.children}
        </SimulationContext.Provider>
      );
}

export function useSimulationContext() { return useContext(SimulationContext); }