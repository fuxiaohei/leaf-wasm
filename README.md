# leaf-wasm

goal to be a simple, fast, and safe web assembly serverless platform, like laf.

## leaf-cli

leaf-cli is a command line tool for leaf-wasm to create, build, and deploy wasm projects.

- initialize a new project

```bash
leaf-cli init my-project
```

- build project to wasm

```bash
cd my-project
leaf-cli build
```

- run wasm in local server

```bash
leaf-cli serve
```

## Build from source

install rust, then run

```bash
cargo build --release
```
