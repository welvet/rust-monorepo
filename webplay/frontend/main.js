import init, { run_app } from './pkg/webplay_frontend.js';

async function main() {
   await init('/pkg/webplay_frontend_bg.wasm');
   run_app();
}

main().then();
