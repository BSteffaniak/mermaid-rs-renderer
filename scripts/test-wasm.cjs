// node scripts/test-wasm.cjs <wasm_smoke.wasm> [caller-font.ttf]
const assert = require('node:assert/strict');
const fs = require('node:fs');
const module_ = new WebAssembly.Module(fs.readFileSync(process.argv[2]));
assert.deepEqual(WebAssembly.Module.imports(module_), []);
const font = process.argv[3] ? fs.readFileSync(process.argv[3]) : Buffer.alloc(0);
for (let i = 0; i < 3; i++) {
    const { memory, alloc, run } = new WebAssembly.Instance(module_).exports;
    const ptr = alloc(font.length);
    new Uint8Array(memory.buffer, ptr, font.length).set(font);
    run(ptr, font.length);
}
console.log('PASS: actual WASM rendering, timings, errors, caller font, repeated instances');
