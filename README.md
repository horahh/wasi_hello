# wasi_hello

based on :
```
https://wasmbyexample.dev/examples/wasi-hello-world/wasi-hello-world.rust.en-us
```

## Purpose

This example exposes a filesystem where it is indicated externally by the wasmtime runtime where the virtual 
filesystem mounts in the host filesystem.

## Run Instructions

wasmtime --mapdir /helloworld::. target/wasm32-wasi/debug/wasi_hello.wasm


