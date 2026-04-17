# Tokay Web Playground

A minimal browser playground for Tokay using WebAssembly.

## Hosted Version

Visit the [live playground](https://playground.tokay.dev) to try Tokay in your browser.

To enable the hosted version, go to Repository Settings > Pages and set the source to "Deploy from a branch" with branch "gh-pages" and folder "/ (root)".

## Local Development

Install `wasm-pack` if you do not already have it:

```bash
cargo install wasm-pack
```

Then build the wasm package:

```bash
cd web-playground
wasm-pack build --target web
```

## Run

Serve the `web-playground` folder over HTTP and open `index.html` in your browser:

```bash
cd web-playground
python3 -m http.server 8000
```

Open `http://localhost:8000` and use the playground.

## Notes

- This playground uses the repository library under `..` with `default-features = false`.
- The `run` function compiles and executes Tokay code with optional input, then returns output or an error message.
