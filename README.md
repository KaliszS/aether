# aether

Interactive visualization of cosmic scale and physics. Rust + WebAssembly, rendered with wgpu (WebGPU, WebGL2 fallback), UI in Leptos.

Live: https://kaliszs.github.io/aether/

## Requirements

- Rust (stable, pinned in `rust-toolchain.toml`)
- [Trunk](https://trunkrs.dev)

## Development

```sh
cd app
trunk serve --open    # http://127.0.0.1:8080, rebuilds on change
trunk build --release # static files in app/dist
```

## Deployment

Every push to `main` builds the app and publishes it to GitHub Pages (`.github/workflows/deploy.yml`).

## Layout

- `engine/` – rendering and simulation (wgpu, WGSL shaders)
- `app/` – Leptos UI shell, canvas, render loop
