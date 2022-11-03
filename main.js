import { instantiate } from "./lib/rs_lib.generated.js";

const wasm = await instantiate();

console.log(wasm);
console.log(wasm.add(1, 2));
console.log(wasm.concat_strings(["hello", "world"]));

wasm.greet("browser");
wasm.main();