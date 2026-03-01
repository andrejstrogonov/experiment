import React, { useState, useEffect } from 'https://esm.sh/react@18';
import ReactDOM from 'https://esm.sh/react-dom@18/client';
// after building, copy `pkg/*` into this directory; imports reference local files
import init, { start_app } from './web_frontend.js';

const defaultScript = `entity Plane {
  components: [Physics];
  on Tick(dt) { move(velocity * dt); }
}`;

function App() {
  const [script, setScript] = useState(defaultScript);
  const [initialized, setInitialized] = useState(false);
  const [started, setStarted] = useState(false);

  useEffect(() => {
    // preload WASM module once
    init().then(() => setInitialized(true));
  }, []);

  const handleStart = () => {
    if (!initialized) return;
    start_app('viewport', script);
    setStarted(true);
  };

  const loadCube = () => setScript(`entity Cube {
  components: [Transform, Physics];
  on Tick(dt) {
    rotateX(0.01);
    rotateY(0.015);
    rotateZ(0.02);
  }
}`);

  const loadAirplane = () => setScript(`entity Plane {
  components: [Physics];
  on Tick(dt) {
    move(velocity * dt);
  }
}`);

  const reload = () => window.location.reload();

  return (
    <div className="layout">
      <div className="left">
        <h3>Meta Script</h3>
        <textarea
          value={script}
          onChange={e => setScript(e.target.value)}
        />
        <div>
          <button onClick={handleStart}>Start</button>
          <button onClick={reload}>Reload</button>
          <button onClick={loadCube}>Load Cube Example</button>
          <button onClick={loadAirplane}>Load Airplane Example</button>
        </div>
        <p>Type a script in the meta-language and click Start.</p>
      </div>
      <div className="right" style={{position:'relative'}}>
        <canvas id="viewport" width="900" height="600" style={{border:'1px solid #222'}}></canvas>
        {!started && (
          <div style={{position:'absolute',top:0,left:0,right:0,bottom:0,display:'flex',alignItems:'center',justifyContent:'center',color:'white',pointerEvents:'none'}}>
            <span>Click &quot;Start&quot; to run the script</span>
          </div>
        )}
      </div>
    </div>
  );
}

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);

