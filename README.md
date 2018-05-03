# WebAssembly Tetris written in Rust

![Snapshot](/static/snapshot.png)

## Development setup

Necessary compiler target `wasm32-unknown-unknown` (Rust nightly only)

```
$ rustup target add wasm32-unknown-unknown
```

Install [cargo-web](https://github.com/koute/cargo-web)

```
$ cargo install cargo-web
```

Start a web dev server

```
$ cargo web start
```

## Build

```
$ cargo web build
```

## Reference

* https://github.com/koute/cargo-web
* https://github.com/koute/stdweb
* https://doc.rust-lang.org/rust-by-example/
* https://developer.mozilla.org/en-US/docs/WebAssembly
* https://en.wikipedia.org/wiki/Tetris

## License

[MIT](http://opensource.org/licenses/MIT)
