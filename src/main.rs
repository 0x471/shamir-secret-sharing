use ark_bn254::Fr as Field;
use ark_ff::{Field as ArkField, UniformRand};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use rand::rngs::OsRng;

struct Point {
    x: Field, // The x-coordinate of the share
    y: Field, // The y-coordinate of the share (f(x) evaluated at x)
}

struct ShamirSecretShare {
    total_shares: usize, // Total number of shares to distribute
}

impl ShamirSecretShare {
    fn new(total_shares: usize) -> Self {
        Self { total_shares }
    }

    fn generate_polynomial(
        secret: Field,
        degree: usize,
        rand: &mut OsRng,
    ) -> DensePolynomial<Field> {
        let mut coefficients = vec![secret]; // The constant term of the polynomial (a_0 = secret)
        for _ in 1..degree {
            let coefficient = Field::rand(rand); // Generate random coefficients a_1, a_2, ..., a_{k-1}
            coefficients.push(coefficient);
        }
        DensePolynomial::from_coefficients_vec(coefficients) // Construct the polynomial f(x) = a_0 + a_1x + ... + a_{k-1}x^{k-1}
    }

    fn generate_shares(&self, secret: Field, degree: usize, rand: &mut OsRng) -> Vec<Point> {
        let poly = Self::generate_polynomial(secret, degree, rand); // Generate the polynomial representing the secret
        (1..=self.total_shares)
            .map(|i| {
                let x = Field::from(i as u64); // Assign x-values 1, 2, ..., total_shares
                let y = poly.evaluate(&x); // Compute f(x) to generate the corresponding y-value (the share)
                Point { x, y } // Create a Point struct for each share
            })
            .collect() // Collect all shares into a vector
    }

    fn reconstruct_secret(shares: &[Point], threshold: usize) -> Field {
        let mut secret = Field::from(0);

        for i in 0..threshold {
            let point_x = &shares[i];
            let x_i = point_x.x; // The x-coordinate of the i-th share
            let y_i = point_x.y; // The y-coordinate of the i-th share (f(x_i))
            let mut lagrange_coeff = Field::from(1); // Initialize the Lagrange coefficient for this share

            for j in 0..threshold {
                if i != j {
                    let point_j = &shares[j];
                    let x_j = point_j.x; // The x-coordinate of the j-th share
                    let numerator = Field::from(0) - x_j; // Compute the numerator: -x_j
                    let denominator = x_i - x_j; // Compute the denominator: x_i - x_j
                    lagrange_coeff *= numerator * denominator.inverse().unwrap();
                    // Update the Lagrange coefficient with the term (-x_j / (x_i - x_j))
                }
            }

            secret += lagrange_coeff * y_i; // Add the contribution of this share: y_i * Lagrange coefficient
        }

        secret // The reconstructed secret is f(0), the constant term of the polynomial
    }
}

fn main() {
    let secret = Field::from(1337u64); // The secret to be shared
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
    let reconstructed_secret =
        ShamirSecretShare::reconstruct_secret(reconstruction_shares, threshold);

    println!("\nOriginal Secret: {}", secret);
    println!("Reconstructed Secret: {}", reconstructed_secret);

    assert_eq!(secret, reconstructed_secret);
    println!("\nSecret reconstruction successful!");
}
