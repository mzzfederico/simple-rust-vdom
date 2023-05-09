# VDOM into Rust

Slowly writing an implementation of a VDOM following [this article](https://dev.to/ycmjason/building-a-simple-virtual-dom-from-scratch-3d05). No idea if this has any actual speed advantage over full JavaScript, as it stills calls DOM APIs to create elements and all.

## Build for WASM

Prepare for wasm building:

```bash
rustup target add wasm32-unknown-unknown
```

next, run the actual compilation

```bash
cargo build --release --target wasm32-unknown-unknown
```

and this wraps it into a usable .js

```bash
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/rust_vdom_example.wasm
```

## Running the code

Just serve the index.html:

```bash
npx serve .
```