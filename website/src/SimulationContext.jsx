import { createContext, useContext } from "solid-js";
import { createSignal } from "solid-js";

const SimulationContext = createContext();
export function SimulationProvider(props) {

    const [time, setTime] = createSignal(0);

    const [paused, setPaused] = createSignal(false);

    return (
        <SimulationContext.Provider value={
          {
            time, setTime, paused, setPaused
          }
        }>
          {props.children}
        </SimulationContext.Provider>
      );
}

export function useSimulationContext() { return useContext(SimulationContext); }