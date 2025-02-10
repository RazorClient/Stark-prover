# FieldElement Library

This library provides a simple implementation of a field element for modular arithmetic over a prime modulus. It is generic over a constant `MODULUS`, which must be a prime number (though the code itself does not enforce primality, it assumes it for correct algebraic behavior).

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Usage](#usage)
    - [Examples](#examples)
- [Implementation Details](#implementation-details)
- [Testing](#testing)
- [License](#license)

## Introduction
A **field element** represents an integer `x` in the range `[0, MODULUS - 1]`. All arithmetic operations are done modulo `MODULUS`. In a proper prime field, every non-zero element has a multiplicative inverse. This library provides:
- Addition, subtraction, multiplication, and division (as multiplication by inverse).
- Negation.
- Exponentiation.
- Conversion from and to `u64`, `i128`, and byte arrays (for serialization).
- Random generation of field elements using a cryptographically secure random number generator (`OsRng`).
- Constant-time equality checks via the [`subtle`](https://docs.rs/subtle/latest/subtle/) crate.

**Note:** The code uses a const generic `MODULUS: u64`, so it's suitable for 64-bit primes. For very large primes (e.g., 256-bit or more), a more specialized library would be required.

## Features
1. **Generic Over Modulus**: You specify the modulus as a const generic parameter (e.g., `FieldElement<7>`).
2. **Constant-Time Equality**: Uses `subtle` for constant-time equality checks to help mitigate certain side-channel attacks.
3. **Modular Arithmetic**: Provides `Add`, `Sub`, `Mul`, and `Div` traits with modular behavior.
4. **Inverse Calculation**: Uses Fermat’s little theorem for computing the inverse (`a^(p-2) % p` when `p` is prime).
5. **Random Generation**: Generates random elements securely using `rand_core::OsRng`.

## Usage


**1. Import and use in your Rust code**

```rust
    use field_element::FieldElement;

    // Example with modulus = 7
    fn main() {
        let a = FieldElement::<7>::new(3);
        let b = FieldElement::<7>::new(5);
        let sum = a + b;
        println!("Sum of 3 and 5 in Field(7): {}", sum.value());
    }
```

### Examples

**1. Basic Arithmetic**
```rust
use field_element::FieldElement;

// Using modulus 7
let a = FieldElement::<7>::new(2);
let b = FieldElement::<7>::new(5);

// Addition: (2 + 5) % 7 = 0
let sum = a + b;
assert_eq!(sum.value(), 0);

// Subtraction: (5 - 2) % 7 = 3
let diff = b - a;
assert_eq!(diff.value(), 3);

// Multiplication: (2 * 5) % 7 = 3
let prod = a * b;
assert_eq!(prod.value(), 3);

// Division: (5 / 2) % 7 = (5 * 2^{-1}) % 7
// 2^{-1} in Field(7) = 4 (since 2*4 = 8 ≡ 1 mod 7)
// So (5 / 2) ≡ 5*4 = 20 ≡ 6
let quot = b / a;
assert_eq!(quot.value(), 6);
```

**2. Inverse and Exponentiation**
```rust
use field_element::FieldElement;

// Inverse
let x = FieldElement::<7>::new(3);
let x_inv = x.inverse();
assert_eq!(x_inv.value(), 5);  // 3 * 5 = 15 ≡ 1 mod 7

// Exponentiation
let y = FieldElement::<7>::new(2);
let y_pow_3 = y.pow(3); // 2^3 = 8 ≡ 1 mod 7
assert_eq!(y_pow_3.value(), 1);
```

**3. Random Generation**
```rust
use field_element::FieldElement;

let random_fe = FieldElement::<7>::random();
println!("Random element in Field(7): {}", random_fe.value());
```



### Methods
1. **new(value: u64) -> Self**  
   Creates a new `FieldElement` from a raw integer, reducing it modulo `MODULUS`.

2. **zero() -> Self**  
   Creates a zero value (`0`).

3. **one() -> Self**  
   Creates a one value (`1`).

4. **value(&self) -> u64**  
   Returns the underlying integer.

5. **random() -> Self**  
   Uses a cryptographically secure RNG to generate a random element in the field.

6. **pow(&self, exp: u64) -> Self**  
   Computes `self^exp (mod MODULUS)` using binary exponentiation.

7. **inverse(&self) -> Self**  
   Computes the multiplicative inverse of `self` using Fermat’s little theorem (`self^(MODULUS-2) mod MODULUS`), assuming `MODULUS` is prime.

8. **to_bytes(&self) -> [u8; 8]**  
   Serializes the value as a big-endian byte array.

### Traits
- **Add, AddAssign**: `(a + b) % MODULUS`
- **Sub, SubAssign**: `(a - b) % MODULUS`, ensuring non-negative result in `[0, MODULUS-1]`.
- **Mul, MulAssign**: `(a * b) % MODULUS`
- **Div, DivAssign**: `(a / b) % MODULUS = a * (b^{-1}) % MODULUS`
- **Neg**: `-a ≡ (MODULUS - a) % MODULUS` (yields the additive inverse).
- **From<i128>**: Conversion from `i128`, adjusting for negative values into the range `[0, MODULUS-1]`.
- **PartialEq, Eq**: Constant-time equality check (via `subtle` crate).

