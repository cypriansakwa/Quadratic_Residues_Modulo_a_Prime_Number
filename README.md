This Rust program computes all quadratic residues modulo a given odd prime number $p$ using Euler's Criterion. A quadratic residue is an integer $a$ such that there exists an integer $x$ satisfying the congruence $x^2 \equiv a \mod p$.

## Overview

The program implements the following key functions:

- `list_quadratic_residues(p: u64) -> Result<Vec<u64>, &'static str>`: Computes all quadratic residues modulo \( p \).
- `is_coprime(a: u64, p: u64) -> bool`: Checks if \( a \) is coprime to \( p \).
- `is_quadratic_residue(a: u64, p: u64) -> Result<bool, &'static str>`: Determines if \( a \) is a quadratic residue modulo \( p \) using Euler's Criterion.
- `gcd(mut a: u64, mut b: u64) -> u64`: Computes the greatest common divisor (GCD) of two numbers.
- `mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64`: Performs modular exponentiation to compute \( (base^{exp}) \mod modulus \).

## Usage

To run the program, ensure you have Rust installed on your machine. Then, use the following commands:

```bash
cargo build --release
cargo run

   git clone https://github.com/cypriansakwa/Quadratic_Residues_Modulo_a_Prime_Number.git
   cd Quadratic_Residues_Modulo_a_Prime_Number
