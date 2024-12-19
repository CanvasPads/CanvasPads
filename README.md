![canvaspads logo](./docs/logo_hero.png)

<h1 align="center">CanvasPads</h1>

<p align="center">
An open source drawing and painting app for making arts.
</p>

## Building CanvasPads web editor

### Install `wasm-bindgen-cli`

```sh
$ cargo install -f wasm-bindgen-cli
```

### Install binaryen

- Precompiled binaries are available on
  [Github](https://github.com/WebAssembly/binaryen/releases).

### Build `canvaspads-web`

```sh
$ cd ./crates/canvaspads-web
$ ./build+optimize.sh
```

### Build web editor

```sh
$ cd ./web
$ npm i
$ npm run build
```
