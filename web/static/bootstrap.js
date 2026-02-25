import init, { start_app } from '../pkg/web_frontend.js';

async function run() {
  await init();
  const startBtn = document.getElementById('start');
  const reloadBtn = document.getElementById('reload');
  const loadCubeBtn = document.getElementById('loadCube');
  const loadAirplaneBtn = document.getElementById('loadAirplane');
  const scriptArea = document.getElementById('script');

  startBtn.addEventListener('click', () => {
    const script = scriptArea.value;
    start_app('viewport', script);
  });

  reloadBtn.addEventListener('click', () => location.reload());

  loadCubeBtn.addEventListener('click', () => {
    scriptArea.value = `entity Cube {
  components: [Transform, Physics];
  on Tick(dt) {
    rotateX(0.01);
    rotateY(0.015);
    rotateZ(0.02);
  }
}`;
  });

  loadAirplaneBtn.addEventListener('click', () => {
    scriptArea.value = `entity Plane {
  components: [Physics];
  on Tick(dt) {
    move(velocity * dt);
  }
}`;
  });
}

run();
