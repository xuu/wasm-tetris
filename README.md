# Wasm Tetris

Tetris game WebAssembly demo.

![Snapshot](/snapshot.png)

## Development setup

Install [Rust](https://www.rust-lang.org/learn/get-started) and [Wasm-Pack](https://rustwasm.github.io/wasm-pack/installer/).

Build:

```
wasm-pack build --target web
```

Setup a local server (`python3 -m http.server` or [serve](https://github.com/zeit/serve)) in the root then open `index.html` in a modern browser.

## References

- https://en.wikipedia.org/wiki/Tetris
- https://github.com/rustwasm/wasm-bindgen
- https://developer.mozilla.org/en-US/docs/WebAssembly

## License

[MIT](LICENSE)
