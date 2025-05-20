use nalgebra::{DMatrix, DVector};
use rand::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Multivariate normal distribution
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone)]
pub struct MultivariateNormal {
    mean: DVector<f64>,
    cov: DMatrix<f64>,
    chol: DMatrix<f64>,
}

impl MultivariateNormal {
    /// Create a new distribution with given mean vector and covariance matrix.
    /// The covariance matrix must be positive definite.
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(mean: Vec<f64>, cov: Vec<f64>, dim: usize) -> Self {
        let mean = DVector::from_vec(mean);
        let cov = DMatrix::from_row_slice(dim, dim, &cov);
        let chol = cov.cholesky().expect("Covariance not PD").l();
        Self { mean, cov, chol }
    }

    /// Sample a random vector from the distribution.
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn sample(&self) -> Vec<f64> {
        let mut rng = thread_rng();
        let normal = rand_distr::StandardNormal;
        let z: DVector<f64> = DVector::from_iterator(self.mean.len(), (0..self.mean.len()).map(|_| normal.sample(&mut rng)));
        let sample = &self.mean + &self.chol * z;
        sample.iter().cloned().collect()
    }

    /// Get the mean vector as a Vec.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn mean(&self) -> Vec<f64> {
        self.mean.iter().cloned().collect()
    }

    /// Get the covariance matrix as a row-major Vec.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn covariance(&self) -> Vec<f64> {
        self.cov.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_dim() {
        let mean = vec![0.0, 0.0];
        let cov = vec![1.0, 0.0, 0.0, 1.0];
        let mvn = MultivariateNormal::new(mean, cov, 2);
        let sample = mvn.sample();
        assert_eq!(sample.len(), 2);
    }

    #[test]
    fn test_mean_and_cov() {
        let mean = vec![1.0, -1.0];
        let cov = vec![2.0, 0.5, 0.5, 1.0];
        let mvn = MultivariateNormal::new(mean.clone(), cov.clone(), 2);
        assert_eq!(mvn.mean(), mean);
        assert_eq!(mvn.covariance(), cov);
    }

    #[test]
    #[should_panic(expected = "Covariance not PD")]
    fn test_non_pd_covariance() {
        // Covariance matrix with zero determinant is not positive definite
        let mean = vec![0.0, 0.0];
        let cov = vec![1.0, -1.0, -1.0, 1.0];
        MultivariateNormal::new(mean, cov, 2);
    }
}
