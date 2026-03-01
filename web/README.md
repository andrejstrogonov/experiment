# web_frontend (WASM UI)

This folder contains a WebAssembly frontend that now uses **React** for its user interface. The app provides:

- a React-managed textarea to type meta-language scripts
- a canvas renderer for a simple airplane
- integration so that the script's `on Tick(dt)` event affects airplane state

Build (needs `wasm-pack`):

```bash
cd web
# release profile produces optimized WebAssembly
wasm-pack build --target web --release
# copy generated JS/wasm into the static folder so the browser can fetch them
cp pkg/* static/
# then serve the static directory
python -m http.server --directory web/static 8000
open http://localhost:8000
```

To automate the build/copy step and avoid missing files (404 errors), a PowerShell helper script is included:

```powershell
cd web
./build.ps1    # compiles with wasm-pack and copies pkg/* into static
python -m http.server --directory web/static 8000
```

Alternatively you can run `npm run build` if you have Node installed. The script guarantees the JS and WASM assets are colocated with `bootstrap.js`, preventing 404s.
The `index.html` loads React from a CDN and mounts the application in a `<div id="root">` element. The main logic lives in `static/bootstrap.js`, which imports React/ReactDOM via `esm.sh` and initializes the WASM module.

The script language now understands `rotateX`, `rotateY`, and `rotateZ` statements and the renderer will draw either a simple airplane or a rotating cube depending on the entity name. Use the example buttons to load the cube or airplane scripts; there is also an overlay prompting you to click **Start** so you won't be greeted by a plain blue screen.

Notes:
- The crate embeds a small parser and executor (a lightweight subset of the project's meta-language and runtime).
- Use the textarea to write `entity Plane { on Tick(dt) { move(velocity * dt); } }` and click Start.
- For rotation demo: `entity Cube { components: [Transform, Physics]; on Tick(dt) { rotateX(0.01); rotateY(0.015); rotateZ(0.02); } }`.

### Testing

Two kinds of tests are available:

1. **Rust unit tests** (and optional wasm-bindgen-browser tests) live in `web/src/lib.rs`. Run them with:
   ```bash
   cd web
   cargo test
   # or for wasm/browser tests: wasm-pack test --headless --firefox
   ```

2. **End‑to‑end UI tests** using Playwright. A separate `package.json` in `web/` defines the tooling. Install dependencies and run:
   ```bash
   cd web
   npm install
   npm test
   ```
   The tests launch a headless browser, open the page, click the buttons and assert that the canvas is being painted (neither a blank blue screen nor unchanged).

All tests ensure that meta-language scripts can be inserted, the WASM module initializes properly, and the cube/plane demos render as expected.
