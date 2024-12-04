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

    fn reconstruct_secret(shares: &[Point], threshold: usize) -> Field {
        let mut secret = Field::from(0);

        for i in 0..threshold {
            let point_x = &shares[i];
            let x_i = point_x.x;
            let y_i = point_x.y;
            let mut lagrange_coeff = Field::from(1);

            for j in 0..threshold {
                if i != j {
                    let point_j = &shares[j];
                    let x_j = point_j.x;
                    let y_j = point_j.y;
                    let numerator = Field::from(0) - x_j; // -x_j
                    let denominator = x_i - x_j; // x_i - x_j
                    lagrange_coeff *= numerator * denominator.inverse().unwrap();
                    // Modular inverse
                }
            }

            secret += lagrange_coeff * y_i;
        }

        secret
    }
}
