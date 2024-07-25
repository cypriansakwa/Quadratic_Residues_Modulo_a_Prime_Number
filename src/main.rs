/// Compute all quadratic residues modulo a prime number p using Euler's Criterion
fn list_quadratic_residues(p: u64) -> Result<Vec<u64>, &'static str> {
    if p < 2 || p % 2 == 0 {
        return Err("p must be an odd prime");
    }

    let mut residues = Vec::new();

    for a in 1..p {
        if is_coprime(a, p) && is_quadratic_residue(a, p)? {
            residues.push(a);
        }
    }

    Ok(residues)
}

/// Determine if a number is coprime to p
fn is_coprime(a: u64, p: u64) -> bool {
    gcd(a, p) == 1
}

/// Check if a is a quadratic residue modulo p using Euler's Criterion
fn is_quadratic_residue(a: u64, p: u64) -> Result<bool, &'static str> {
    let exponent = (p - 1) / 2;
    let result = mod_exp(a, exponent, p);

    if result == 1 {
        Ok(true) // a is a quadratic residue modulo p
    } else if result == p - 1 {
        Ok(false) // a is a non-residue modulo p
    } else {
        Err("Unexpected result")
    }
}

/// Compute greatest common divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Modular exponentiation: (base^exp) % modulus
fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    base = base % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }

    result
}

fn main() {
    let p = 7;

    match list_quadratic_residues(p) {
        Ok(residues) => println!("Quadratic residues modulo {}: {:?}", p, residues),
        Err(err) => println!("Error: {}", err),
    }
}
