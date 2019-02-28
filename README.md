# Wasm Tetris

A Rust/WebAssembly 'learning by doing' example.

![Snapshot](/snapshot.png)

## Development setup

Install Rust compiler target `wasm32-unknown-unknown` and `wasm-bindgen`:

```
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-bindgen-cli
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

[MIT](http://opensource.org/licenses/MIT)
