<div align="center">

  <h1>PoW JavaScript library</h1>

<strong>JavaScript library to generate PoW for mCaptcha</strong>

[![0.1.0](https://img.shields.io/badge/Rust_docs-master-dea584)](https://mcaptcha.github.io/browser/rust/mcaptcha_browser/index.html)
[![0.1.0](https://img.shields.io/badge/TypeScript_docs-master-2b7489)](https://mcaptcha.github.io/browser/ts/docs/modules.html)
![Build)](<https://github.com/mCaptcha/browser/workflows/CI%20(Linux)/badge.svg>)
[![dependency status](https://deps.rs/repo/github/mCaptcha/browser/status.svg)](https://deps.rs/repo/github/mCaptcha/browser)
<br />
[![codecov](https://codecov.io/gh/mCaptcha/browser/branch/master/graph/badge.svg)](https://codecov.io/gh/mCaptcha/browser)

</div>

**NOTE:** wasm compilation currently requires `rustc` nightly and
wasm optimization of this library will have to be done manually at the
moment. Please refer to https://github.com/rustwasm/wasm-pack/issues/886
for more information.

### Optimization:

```
$ /path/to/wasm-opt pkg/pow_bg.wasm -o pkg/pow_bg.wasm -O --enable-mutable-globals
```

My `/path/to/wasm-opt` is `~/.cache/.wasm-pack/wasm-opt-4d7a65327e9363b7/wasm-opt`

---

<h2>  Default documentation provided by Rust wasm: </h2>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
