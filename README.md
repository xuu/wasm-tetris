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
