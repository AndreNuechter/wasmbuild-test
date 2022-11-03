import { instantiate } from "./lib/rs_lib.generated.js";

const wasm = await instantiate();

console.log(wasm);

wasm.main();

console.log(wasm.add(1, 2));