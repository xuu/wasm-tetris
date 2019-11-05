# Wasm Tetris

Tetris game in WebAssembly.

![Snapshot](/snapshot.png)

## Development setup

Install [Rust](https://www.rust-lang.org/learn/get-started) and [Wasm-Pack](https://rustwasm.github.io/wasm-pack/installer/).

Build:

```
wasm-pack build --out-dir examples/pkg --target web
```

Run a local server at `/examples` then open `/examples/index.html` in a modern browser.

## References

- https://en.wikipedia.org/wiki/Tetris
- https://github.com/rustwasm/wasm-bindgen
- https://developer.mozilla.org/en-US/docs/WebAssembly

## License

[MIT](LICENSE)
