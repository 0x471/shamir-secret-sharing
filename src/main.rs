use ark_bn254::Fr as Field;
use ark_ff::{Field as ArkField, UniformRand};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use rand::rngs::OsRng;

struct Point {
    x: Field,
    y: Field,
}

struct ShamirSecretShare {
    total_shares: usize,
}

impl ShamirSecretShare {
    fn new(total_shares: usize) -> Self {
        Self {
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
                    //let y_j = point_j.y;
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

fn main() {
    let secret = Field::from(1234u64); // The secret to be shared
    let threshold = 3; // Minimum number of shares required to reconstruct the secret
    let mut rng = OsRng;

    let shamir = ShamirSecretShare::new(threshold);

    // Step 1: Generate shares
    let shares = shamir.generate_shares(secret, threshold, &mut rng);
    println!("Generated Shares:");
    for (i, share) in shares.iter().enumerate() {
        println!("Share {}: x = {}, y = {}", i + 1, share.x, share.y);
    }

    // Step 2: Reconstruct the secret using the first `threshold` shares
    let reconstruction_shares = &shares[0..threshold]; // Take the first `threshold` shares
    let reconstructed_secret = ShamirSecretShare::reconstruct_secret(reconstruction_shares, threshold);

    println!("\nOriginal Secret: {}", secret);
    println!("Reconstructed Secret: {}", reconstructed_secret);

    assert_eq!(secret, reconstructed_secret);
    println!("\nSecret reconstruction successful!");
}
