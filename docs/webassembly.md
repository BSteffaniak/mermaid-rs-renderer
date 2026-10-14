# Import-free WebAssembly

With default features disabled, the library supports `wasm32-unknown-unknown`
without WASI or browser imports. Diagnostic timing values are zero on this target
because no clock is available. Native timing behavior is unchanged.

System-font discovery and disk caches are unavailable on this target. Without a
caller font, existing layout fallback measurements are used. Applications can
call `set_text_measurement_font(font_bytes, face_index)` once before rendering to
use deterministic metrics. The selected face is process-wide and applies to all
requested families. The caller owns font licensing and should configure matching
SVG theme fonts; this API does not embed font bytes into SVG.

Run actual execution tests (Node, no npm dependencies):

```sh
cargo build --no-default-features --release --target wasm32-unknown-unknown --example wasm_smoke
node scripts/test-wasm.cjs target/wasm32-unknown-unknown/release/examples/wasm_smoke.wasm /path/to/font.ttf
```

The optional font argument exercises caller-supplied metrics as well as default
fallbacks. The probe covers seven diagram fixtures, detailed timings, invalid
fonts/diagrams, deterministic repeated renders and fresh import-free instances.
