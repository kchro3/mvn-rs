const { performance } = require('perf_hooks');
const wasm = require('../mvn/pkg');
const { SimpleMultivariateNormal } = require('../js/simple_mvn');

const dim = 2;
const mean = [0, 0];
const cov = [1, 0, 0, 1];

const wasmMvn = new wasm.MultivariateNormal(mean, cov, dim);
const jsMvn = new SimpleMultivariateNormal(mean, cov, dim);

function bench(name, fn, iter) {
  const start = performance.now();
  for (let i = 0; i < iter; i++) fn();
  const end = performance.now();
  console.log(`${name}: ${(end - start).toFixed(2)} ms`);
}

const ITER = 10000;
bench('wasm', () => wasmMvn.sample(), ITER);
bench('js', () => jsMvn.sample(), ITER);
