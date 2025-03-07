import init, { hello } from "../public/wasm/wasm_crate.js";

async function runWasm() {
  await init();
  console.log(hello("yooo"));
}

runWasm();
