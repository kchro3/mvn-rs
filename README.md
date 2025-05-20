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

## JavaScript Interface

After building the WebAssembly package, you can access the distribution from Node.js:

```javascript
const mvn = require('./mvn/pkg');
const dist = new mvn.MultivariateNormal([0, 0], [1, 0, 0, 1], 2);
console.log(dist.sample());
```

## Benchmarks

Simple performance benchmarks can be run with:

```bash
npm run benchmark
```

This compares the WebAssembly implementation with a pure JavaScript fallback located in `js/simple_mvn.js`.
