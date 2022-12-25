# leaf-wasm

goal to be a simple, fast, and safe web assembly serverless platform, like laf.

## leaf-cli

leaf-cli is a command line tool for leaf-wasm to create, build, and deploy wasm projects.

- create a new project

```bash
leaf-cli new my-project
```

- compile project to wasm

```bash
cd my-project
leaf-cli compile
```

- run wasm in local

```bash
leaf-cli up
```

## Build from source

install rust, then run

```bash
cargo build --release
```
