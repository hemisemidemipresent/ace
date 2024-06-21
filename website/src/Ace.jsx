import { createContext, useContext } from "solid-js";
import { createStore, produce } from "solid-js/store";
import { constrain_to_pi, constrain_to_tau } from './utils/AngleUtils'
import { PI, TAU, R, v_c, MAX_TURN_PER_TICK, OMEGA, TARGETING } from './utils/Constants'

const AceContext = createContext();

export function AceProvider(props) {
  const [aceState, setAceState] = createStore({
    x: R * 3 / 8, // can set default via props.x or props.y
    y: 0,
    theta: 0,
    onPath: false,
    phase: 0,
    reverse: 1,
    targeting: TARGETING.CIRCLE
  })

  const [events, setEvents] = createStore([])

  function tab(time) {
    setAceState('targeting', (aceState.targeting + 1) % 3)
    setAceState('onPath', false)
    setEvents(
      produce((events) => {
        events.push({ targeting: aceState.targeting, time: time, completed: true })
      })
    );
  }

  function untab(time) {
    setAceState('targeting', (aceState.targeting + 2) % 3)
    setAceState('onPath', false)
    setEvents(
      produce((events) => {
        events.push({ targeting: aceState.targeting, time: time, completed: true })
      })
    );
  }

  function reverse(time) {
    setAceState('reverse', -aceState.reverse)
    setAceState('onPath', false)
    setAceState('phase', aceState.phase + PI / 5 * aceState.reverse)
    setEvents(
      produce((events) => {
        events.push({ targeting: 3, time: time, completed: true })
      })
    );
  }
  function setTargeting(targeting) {
    setAceState('targeting', targeting)
    setAceState('onPath', false)
  }
  const circlePos = () => {
    let x = R * Math.cos(aceState.phase);
    let y = R * Math.sin(aceState.phase);
    return { x, y };
  };
  const figureEightPos = () => {
    if (aceState.phase <= PI) {
      let x = R / 2 * Math.cos(aceState.phase * 2 + PI / 2);
      let y = R / 2 * Math.sin(aceState.phase * 2 + PI / 2);
      y -= R / 2;
      return { x, y };
    }
    let x = -R / 2 * Math.cos((aceState.phase * 2 - TAU) - PI / 2);
    let y = R / 2 * Math.sin((aceState.phase * 2 - TAU) - PI / 2);
    y += R / 2;
    return { x, y };

  };
  const figureInfinitePos = () => {
    if (aceState.phase <= PI) {
      let x = R / 2 * Math.cos(aceState.phase * 2 + PI);
      let y = R / 2 * Math.sin(aceState.phase * 2 + PI);
      x += R / 2;
      return { x, y };
    }
    let x = R / 2 * Math.cos((aceState.phase * 2 - TAU));
    let y = -R / 2 * Math.sin((aceState.phase * 2 - TAU));
    x -= R / 2;
    return { x, y };

  };
  const targetPoint = () => {
    if (aceState.targeting == TARGETING.CIRCLE) return circlePos();
    if (aceState.targeting == TARGETING.EIGHT) return figureEightPos();
    if (aceState.targeting == TARGETING.INFINITY) return figureInfinitePos();
  }

  function simulate(dtick, time) {
    setAceState('phase', constrain_to_tau(aceState.phase + OMEGA * dtick * aceState.reverse))

    // check for events
    let index = events.findIndex((event) => !event.completed && time > event.time);
    if (index != -1) {

      if (events[index].targeting == 3) {
        reverse()
      }
      else {
        setTargeting(events[index].targeting)
      }
      setEvents(index, "completed", true);
    }
    let target = targetPoint()

    if (aceState.onPath) {
      setAceState('x', target.x)
      setAceState('y', target.y)

      if (aceState.targeting == TARGETING.CIRCLE)
        setAceState('theta', -PI / 2 - aceState.phase)
      else if (aceState.targeting == TARGETING.EIGHT) {
        if (aceState.phase < PI)
          setAceState('theta', PI - 2 * aceState.phase)
        else
          setAceState('theta', 2 * (aceState.phase - PI) - PI)
      }
      else if (aceState.targeting == TARGETING.INFINITY) {
        if (aceState.phase < PI)
          setAceState('theta', PI / 2 - 2 * aceState.phase)
        else
          setAceState('theta', 2 * aceState.phase + PI / 2)
      }
      else throw new Error('WTF')
      // temp
      setAceState('theta', -aceState.theta)

      if (aceState.reverse == -1) setAceState('theta', (theta) => theta + PI)
    }
    else {
      let delta_x = target.x - aceState.x;
      let delta_y = target.y - aceState.y;

      let angle = Math.atan2(delta_y, delta_x)

      let angle_diff = constrain_to_pi(angle - aceState.theta)

      if (Math.abs(angle_diff) < MAX_TURN_PER_TICK * dtick)
        setAceState('theta', (theta) => theta + Math.sign(angle_diff) * angle_diff)
      else
        setAceState('theta', (theta) => theta + Math.sign(angle_diff) * MAX_TURN_PER_TICK * dtick)
      setAceState('theta', constrain_to_pi(aceState.theta))

      let step = v_c * dtick;

      setAceState('x', (x) => x + step * Math.cos(aceState.theta))
      setAceState('y', (y) => y + step * Math.sin(aceState.theta))

      let dist_squared = (target.x - aceState.x) ** 2 + (target.y - aceState.y) ** 2

      if (dist_squared < 1) setAceState('onPath', true)
    }
  }

  function setDefault() {
    setAceState('x', 3 / 8 * R)
    setAceState('y', 0)
    setAceState('phase', 0)
    setAceState('theta', 0)
    setAceState('onPath', false)
    setAceState('reverse', 1)
    setAceState('targeting', TARGETING.CIRCLE)
  }

  return (
    <AceContext.Provider value={
      {
        aceState, setAceState,
        events, setEvents,
        tab, untab, reverse,
        circlePos, figureEightPos, figureInfinitePos, targetPoint,
        simulate,
        setDefault
      }
    }>
      {props.children}
    </AceContext.Provider>
  );
}

export function useAceContext() { return useContext(AceContext); }