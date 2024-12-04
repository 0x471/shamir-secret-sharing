use ark_bn254::Fr as Field;
use ark_ff::{Field as ArkField, UniformRand};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use rand::rngs::OsRng;

struct Point {
    x: Field,
    y: Field,
}

struct ShamirSecretShare {
    threshold: usize,
    total_shares: usize,
}

impl ShamirSecretShare {
    fn new(threshold: usize, total_shares: usize) -> Self {
        Self {
            threshold,
            total_shares,
        }
    }

    fn generate_polynomial(
        secret: Field,
        degree: usize,
        rand: &mut OsRng,
    ) -> DensePolynomial<Field> {
        let mut coefficients = vec![secret];
        for _ in 1..degree {
            let coefficient = Field::rand(rand);
            coefficients.push(coefficient)
        }
        DensePolynomial::from_coefficients_vec(coefficients)
    }

    fn generate_shares(&self, secret: Field, degree: usize, rand: &mut OsRng) -> Vec<Point> {
        let poly = Self::generate_polynomial(secret, degree, rand);
        (1..=self.total_shares)
        .map(|i| {
            let x = Field::from(i as u64);
            let y = poly.evaluate(&x);
            Point { x, y }
        })
        .collect()
    }
}
