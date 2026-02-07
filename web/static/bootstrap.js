import init, { start_app } from '../pkg/web_frontend.js';

async function run() {
  await init();
  const startBtn = document.getElementById('start');
  const reloadBtn = document.getElementById('reload');
  const scriptArea = document.getElementById('script');

  startBtn.addEventListener('click', () => {
    const script = scriptArea.value;
    start_app('viewport', script);
  });

  reloadBtn.addEventListener('click', () => location.reload());
}

run();
