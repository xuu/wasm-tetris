# Wasm Tetris

Tetris game WebAssembly demo.

![Snapshot](/snapshot.png)

## Development setup

Install Rust compiler target `wasm32-unknown-unknown`:

```
$ rustup target add wasm32-unknown-unknown
```

Install JS dependencies:

```
$ npm install
```

Start webpack dev server:

```
$ npm run dev
```

## References

- https://en.wikipedia.org/wiki/Tetris
- https://github.com/rustwasm/wasm-bindgen
- https://developer.mozilla.org/en-US/docs/WebAssembly

## License

[MIT](LICENSE)
