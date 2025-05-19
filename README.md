# mvn-rs

Rust implementation of a multivariate normal distribution with WebAssembly bindings.

## Building

To build the Rust library normally:

```bash
cargo build
```

To build the WebAssembly package (requires `wasm-pack`):

```bash
wasm-pack build -t node --release -- --features wasm
```

This will generate an npm package in the `pkg` directory that can be published to npm.

## Usage

```rust
use mvn::MultivariateNormal;

let mean = vec![0.0, 0.0];
let cov = vec![1.0, 0.0, 0.0, 1.0];
let mvn = MultivariateNormal::new(mean, cov, 2);
let sample = mvn.sample();
```
