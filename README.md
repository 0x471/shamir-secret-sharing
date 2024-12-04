# **Shamir's Secret Sharing**

This is an implementation of **Shamir's Secret Sharing** scheme on the **BN254** field using the **arkworks** library. It allows splitting a secret into multiple shares such that the secret can only be reconstructed when a minimum number of shares (threshold) are available. This ensures both fault tolerance and security for sensitive information.

---

## **Overview**

### **What is Shamir's Secret Sharing?**
Shamir's Secret Sharing is a cryptographic technique that allows a secret (e.g., a password, private key, or any sensitive data) to be divided into \( n \) shares. To reconstruct the secret, at least \( k \) (threshold) shares are required. It is based on polynomial interpolation and works over a finite field.

- **Security**: With fewer than \( k \) shares, the secret cannot be reconstructed.
- **Fault Tolerance**: Even if some shares are lost, the secret can still be recovered using any \( k \) shares.

---

### **Dependencies**
Add the following dependencies to your `Cargo.toml` file:
```toml
[dependencies]
ark-bn254 = "0.5.0"
ark-ff = "0.5.0"
ark-poly = "0.5.0"
rand = "0.8.5"
```