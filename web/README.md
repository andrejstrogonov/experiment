# web_frontend (WASM UI)

This folder contains a minimal WebAssembly frontend that provides:
- a textarea to type meta-language scripts
- a canvas renderer for a simple airplane
- integration so that the script's `on Tick(dt)` event affects airplane state

Build (needs `wasm-pack`):

```bash
cd web
wasm-pack build --target web
# then serve web/static (e.g. with simple HTTP server)
python -m http.server --directory web/static 8000
open http://localhost:8000
```

Notes:
- The crate embeds a small parser and executor (a lightweight subset of the project's meta-language and runtime).
- Use the textarea to write `entity Plane { on Tick(dt) { move(velocity * dt); } }` and click Start.
