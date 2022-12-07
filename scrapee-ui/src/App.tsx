import { useState } from 'react';
import reactLogo from './assets/react.svg';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

const colors = {
  'Hunter Green': '386641',
  'May Green': '6a994e',
  'Android Green': 'a7c957',
  Eggshell: 'f2e8cf',
  'Bittersweet Shimmer': 'bc4749',
};

function App() {
  const [greetMsg, setGreetMsg] = useState('');
  const [name, setName] = useState('');

  async function seed_saraba() {
    await invoke('plugin:scrapee|seed_saraba');
  }

  async function start() {
    await invoke('plugin:scrapee|start_job');
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <div className="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={() => seed_saraba()}>
            Seed Saraba
          </button>

          <button type="button" onClick={() => start()}>
            Start
          </button>
        </div>
      </div>
      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
