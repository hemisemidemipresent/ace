/* @refresh reload */
import { render } from 'solid-js/web';

import './styles/index.css';
import './styles/ToggleSwitch.css'
import App from './App';
import { AceProvider } from './Ace'
import { SimulationProvider } from './SimulationContext';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

render(() => <SimulationProvider><AceProvider><App /></AceProvider></SimulationProvider>, root);
