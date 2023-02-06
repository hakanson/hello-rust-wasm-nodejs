# Hello Rust WASM NodeJS

"Hello World" in Rust, compiled to WASM, and called from NodeJS.  Also runs inside a Visual Studio Dev Container and uses TypeScript.

Read these first:

* [Developing inside a Container](https://code.visualstudio.com/docs/devcontainers/containers)
* [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
* [Welcome to the wasm-pack docs!](https://rustwasm.github.io/docs/wasm-pack/introduction.html)

To build Rust WASM module

```bash
wasm-pack build --target nodejs
```

To run NodeJS code

```bash
cd tests/nodejs
npm install
npx ts-node hello.ts
```
