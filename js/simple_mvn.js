function randn() {
  let u = 0, v = 0;
  while (u === 0) u = Math.random();
  while (v === 0) v = Math.random();
  return Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v);
}

function cholesky(cov, dim) {
  const L = new Array(dim * dim).fill(0);
  for (let i = 0; i < dim; i++) {
    for (let j = 0; j <= i; j++) {
      let sum = cov[i * dim + j];
      for (let k = 0; k < j; k++) {
        sum -= L[i * dim + k] * L[j * dim + k];
      }
      if (i === j) {
        L[i * dim + j] = Math.sqrt(sum);
      } else {
        L[i * dim + j] = sum / L[j * dim + j];
      }
    }
  }
  return L;
}

class SimpleMultivariateNormal {
  constructor(mean, cov, dim) {
    this.mean = mean;
    this.dim = dim;
    this.chol = cholesky(cov, dim);
  }

  sample() {
    const z = [];
    for (let i = 0; i < this.dim; i++) z.push(randn());
    const out = [];
    for (let i = 0; i < this.dim; i++) {
      let val = this.mean[i];
      for (let j = 0; j <= i; j++) {
        val += this.chol[i * this.dim + j] * z[j];
      }
      out[i] = val;
    }
    return out;
  }
}

module.exports = { SimpleMultivariateNormal };
