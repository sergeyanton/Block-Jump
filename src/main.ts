import init, { Game } from "../public/wasm/wasm_crate.js";

async function run() {
  await init(); 

  const canvas = document.getElementById("gameCanvas") as HTMLCanvasElement;
  const crc = canvas.getContext("2d")!;

  const game = new Game();
  
  document.addEventListener('keydown', (e) => {
    if (e.code === 'Space') {
      game.jump();
    }
  });

  document.addEventListener('keydown', (e) => {
    if (e.code === 'Space') {
        game.jump();
    }
    if (e.code === 'KeyR') {
        game.restart();
    }
});

  let lastTime = performance.now();

  function gameLoop() {
    const now = performance.now();
    const deltaTime = (now - lastTime) / 1000; 
    lastTime = now;

    game.update(deltaTime); 
    game.render(crc);

    requestAnimationFrame(gameLoop);
  }

  gameLoop(); 
}

run();
