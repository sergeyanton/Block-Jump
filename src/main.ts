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

  function gameLoop() {
    game.update();
    game.render(crc);
    requestAnimationFrame(gameLoop);
  }

  gameLoop(); // Start loop
}

run();