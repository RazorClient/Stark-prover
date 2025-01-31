# Polynomial Module Documentation

The `Polynomial` module provides a robust and efficient implementation of polynomials over finite fields in Rust. Leveraging Rust's powerful type system and operator overloading, this module offers intuitive and performant polynomial arithmetic suitable for various applications, including cryptography, coding theory, and algebraic computations.

## Table of Contents

1. [Overview](#overview)
2. [Struct Definition](#struct-definition)
3. [Creating Polynomials](#creating-polynomials)
    - [Using `Polynomial::new`](#using-polynomialnew)
    - [Using `Polynomial::zero`](#using-polynomialzero)
    - [Using `FromIterator`](#using-fromiterator)
4. [Arithmetic Operations](#arithmetic-operations)
    - [Addition (`+`, `+=`)](#addition-+-)
    - [Subtraction (`-`, `-=`)](#subtraction--)
    - [Multiplication (`*`, `*=`)](#multiplication-*-)
    - [Division (`/`, `/=`)](#division-/--)
    - [Remainder (`%`, `%=`)](#remainder-%--)
    - [Negation (`-`)](#negation--)
5. [Scalar Operations](#scalar-operations)
    - [Scalar Multiplication](#scalar-multiplication)
    - [Scalar Division](#scalar-division)
6. [Evaluation and Composition](#evaluation-and-composition)
    - [Evaluating a Polynomial](#evaluating-a-polynomial)
    - [Composing Polynomials](#composing-polynomials)
7. [Examples](#examples)
    - [Basic Operations](#basic-operations)
    - [Evaluation and Composition](#evaluation-and-composition-example)
8. [Traits and Implementations](#traits-and-implementations)
    - [Operator Overloading](#operator-overloading)
    - [Function Traits (`Fn`, `FnMut`, `FnOnce`)](#function-traits-fn-fnmut-fnonce)
    - [Iterators](#iterators)

---

## Overview

The `Polynomial` struct represents a polynomial over a finite field defined by a modulus. It supports a wide range of operations, including addition, subtraction, multiplication, division, scalar operations, evaluation, and composition. The implementation ensures that trailing zero coefficients are automatically trimmed to maintain the correct degree of the polynomial.

## Struct Definition

```rust
#[derive(Clone, Debug)]
pub struct Polynomial<const MODULUS: u64> {
    pub coefficients: Vec<FieldElement<MODULUS>>,
    pub degree: isize,
}
```

- **`coefficients`**: A vector where `coefficients[i]` corresponds to the coefficient of the \( x^i \) term.
- **`degree`**: The degree of the polynomial. A degree of `-1` indicates the zero polynomial.

## Creating Polynomials

### Using `Polynomial::new`

Creates a new polynomial from a vector of coefficients. Trailing zeros are automatically trimmed, and the degree is set accordingly.

```rust
use crate::fields::FieldElement;
use crate::polynomial::Polynomial;

const MODULUS: u64 = 17;

// Example: p(x) = 3 + 2x + 5x^2
let coeffs = vec![
    FieldElement::<MODULUS>::new(3),
    FieldElement::<MODULUS>::new(2),
    FieldElement::<MODULUS>::new(5),
];
let p = Polynomial::<MODULUS>::new(coeffs);

println!("Polynomial p(x): {:?}", p);
```

### Using `Polynomial::zero`

Creates the zero polynomial.

```rust
let zero_poly = Polynomial::<17>::zero();

println!("Zero Polynomial: {:?}", zero_poly);
```

### Using `FromIterator`

Constructs a polynomial from any iterator of `FieldElement<MODULUS>`.

```rust
use std::iter::FromIterator;

let coeffs = vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(4),
    FieldElement::<17>::new(0),
    FieldElement::<17>::new(7),
];
let p = Polynomial::from_iter(coeffs);

println!("Polynomial from iterator: {:?}", p);
```

## Arithmetic Operations

The `Polynomial` struct implements standard arithmetic operations using Rust's operator overloading.

### Addition (`+`, `+=`)

#### Using `+` Operator

```rust
let p1 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
]);
let p2 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(3),
    FieldElement::<17>::new(4),
]);
let sum = p1 + p2; // sum = [4, 6]

println!("Sum: {:?}", sum);
```

#### Using `+=` Operator

```rust
let mut p1 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
]);
let p2 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(3),
    FieldElement::<17>::new(4),
]);
p1 += p2; // p1 becomes [4, 6]

println!("p1 after += p2: {:?}", p1);
```

### Subtraction (`-`, `-=`)

#### Using `-` Operator

```rust
let p1 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(5),
    FieldElement::<17>::new(7),
]);
let p2 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(2),
    FieldElement::<17>::new(3),
]);
let difference = p1 - p2; // difference = [3, 4]

println!("Difference: {:?}", difference);
```

#### Using `-=` Operator

```rust
let mut p1 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(5),
    FieldElement::<17>::new(7),
]);
let p2 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(2),
    FieldElement::<17>::new(3),
]);
p1 -= p2; // p1 becomes [3, 4]

println!("p1 after -= p2: {:?}", p1);
```

### Multiplication (`*`, `*=`)

#### Using `*` Operator

```rust
let p1 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
]);
let p2 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(3),
    FieldElement::<17>::new(4),
]);
let product = p1 * p2; // product = [3, 10, 8]

println!("Product: {:?}", product);
```

#### Using `*=` Operator

```rust
let mut p1 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
]);
let p2 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(3),
    FieldElement::<17>::new(4),
]);
p1 *= p2; // p1 becomes [3, 10, 8]

println!("p1 after *= p2: {:?}", p1);
```

### Division (`/`, `/=`)

Performs polynomial long division. The division operation requires that the divisor polynomial divides the dividend polynomial without a remainder.

#### Using `/` Operator

```rust
let dividend = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(6),
    FieldElement::<17>::new(11),
    FieldElement::<17>::new(7),
]);
let divisor = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(2),
    FieldElement::<17>::new(3),
]);
let quotient = dividend / divisor; // quotient = [3, 1]

println!("Quotient: {:?}", quotient);
```

**Note**: If the division results in a non-zero remainder, a panic occurs.

#### Using `/=` Operator

```rust
let mut dividend = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(6),
    FieldElement::<17>::new(11),
    FieldElement::<17>::new(7),
]);
let divisor = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(2),
    FieldElement::<17>::new(3),
]);
dividend /= divisor; // dividend becomes [3, 1]

println!("Dividend after /= divisor: {:?}", dividend);
```

### Remainder (`%`, `%=`)

Calculates the remainder of polynomial division.

#### Using `%` Operator

```rust
let dividend = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(6),
    FieldElement::<17>::new(11),
    FieldElement::<17>::new(7),
]);
let divisor = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(2),
    FieldElement::<17>::new(3),
]);
let remainder = dividend % divisor; // remainder = [0]

println!("Remainder: {:?}", remainder);
```

#### Using `%=` Operator

```rust
let mut dividend = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(6),
    FieldElement::<17>::new(11),
    FieldElement::<17>::new(7),
]);
let divisor = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(2),
    FieldElement::<17>::new(3),
]);
dividend %= divisor; // dividend becomes [0]

println!("Dividend after %= divisor: {:?}", dividend);
```

### Negation (`-`)

Negates a polynomial by negating each of its coefficients.

```rust
let p = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
]);
let neg_p = -p; // neg_p = [16, 15]

println!("Negated Polynomial: {:?}", neg_p);
```

## Scalar Operations

### Scalar Multiplication

Multiplies each coefficient of the polynomial by a scalar from the field.

#### Using `*` Operator

```rust
let p = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
]);
let scalar = FieldElement::<17>::new(3);
let product = p * scalar; // product = [3, 6]

println!("Scalar Product: {:?}", product);
```

#### Using `*=` Operator

```rust
let mut p = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
]);
let scalar = FieldElement::<17>::new(3);
p *= scalar; // p becomes [3, 6]

println!("p after *= scalar: {:?}", p);
```

### Scalar Division

Divides each coefficient of the polynomial by a scalar from the field (equivalent to multiplying by the scalar's inverse).

#### Using `/` Operator

```rust
let p = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(6),
    FieldElement::<17>::new(9),
]);
let scalar = FieldElement::<17>::new(3);
let quotient = p / scalar; // quotient = [2, 3]

println!("Scalar Quotient: {:?}", quotient);
```

#### Using `/=` Operator

```rust
let mut p = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(6),
    FieldElement::<17>::new(9),
]);
let scalar = FieldElement::<17>::new(3);
p /= scalar; // p becomes [2, 3]

println!("p after /= scalar: {:?}", p);
```

**Note**: Dividing by zero will panic.

## Evaluation and Composition

### Evaluating a Polynomial

Evaluates the polynomial at a given field element using Horner's method.

#### Using `evaluate` Method

```rust
let p = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(3), // constant term
    FieldElement::<17>::new(2), // x^1 term
    FieldElement::<17>::new(5), // x^2 term
]);
let x = FieldElement::<17>::new(4);
let value = p.evaluate(x); // Evaluates 3 + 2*4 + 5*4^2 mod 17

println!("p(4) = {:?}", value); // Expected: 3 + 8 + 5*16 = 3 + 8 + 80 = 91 ≡ 6 mod 17
```

#### Using Function Traits (`Fn`, `FnMut`, `FnOnce`)

The `Polynomial` struct implements `Fn`, allowing you to call a polynomial like a function.

```rust
let p = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(3),
    FieldElement::<17>::new(2),
    FieldElement::<17>::new(5),
]);
let x = FieldElement::<17>::new(4);
let value = p(x); // Equivalent to p.evaluate(x)

println!("p(4) using Fn trait: {:?}", value); // Expected: 6
```

### Composing Polynomials

Composes two polynomials, effectively evaluating one polynomial with another as its input.

```rust
let p = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
]); // p(x) = 1 + 2x

let q = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(3),
    FieldElement::<17>::new(4),
]); // q(x) = 3 + 4x

let composition = p.compose(&q); // composition = p(q(x)) = 1 + 2*(3 + 4x) = 7 + 8x

println!("Composition p(q(x)): {:?}", composition);
```

Alternatively, using function traits:

```rust
let composition = p(q); // Equivalent to p.compose(&q)

println!("Composition using Fn trait p(q(x)): {:?}", composition);
```

## Examples

### Basic Operations

```rust
use crate::fields::FieldElement;
use crate::polynomial::Polynomial;

const MODULUS: u64 = 17;

// Creating polynomials
let p = Polynomial::<MODULUS>::new(vec![
    FieldElement::<MODULUS>::new(1),
    FieldElement::<MODULUS>::new(2),
    FieldElement::<MODULUS>::new(3),
]); // p(x) = 1 + 2x + 3x^2

let q = Polynomial::<MODULUS>::new(vec![
    FieldElement::<MODULUS>::new(4),
    FieldElement::<MODULUS>::new(5),
]); // q(x) = 4 + 5x

// Addition
let sum = &p + &q; // sum(x) = [5, 7, 3]

println!("Sum of p and q: {:?}", sum);

// Subtraction
let difference = &p - &q; // difference(x) = [14, 14, 3]

println!("Difference of p and q: {:?}", difference);

// Multiplication
let product = &p * &q; // product(x) = [4, 13, 22, 15]

println!("Product of p and q: {:?}", product);

// Scalar Multiplication
let scalar = FieldElement::<MODULUS>::new(3);
let scalar_product = &p * scalar; // scalar_product(x) = [3, 6, 9]

println!("Scalar Product of p and 3: {:?}", scalar_product);

// Evaluation
let x = FieldElement::<MODULUS>::new(2);
let value = p.evaluate(x); // Evaluates 1 + 2*2 + 3*4 = 1 + 4 + 12 = 17 ≡ 0 mod 17

println!("p(2) = {:?}", value); // Expected: 0

// Composition
let composition = p.compose(&q); // p(q(x)) = 1 + 2*(4 + 5x) + 3*(4 + 5x)^2
// Compute step by step:
// 2*(4 + 5x) = 8 + 10x ≡ 8 + 10x mod 17
// (4 + 5x)^2 = 16 + 40x + 25x^2 ≡ 16 + 6x + 8x^2 mod 17
// 3*(16 + 6x + 8x^2) = 48 + 18x + 24x^2 ≡ 14 + 1x + 7x^2 mod 17
// Sum: 1 + (8 + 10x) + (14 + x + 7x^2) = 23 + 11x + 7x^2 ≡ 6 + 11x + 7x^2 mod 17

println!("Composition p(q(x)): {:?}", composition); // Expected: [6, 11, 7]
```

### Evaluation and Composition Example

```rust
use crate::fields::FieldElement;
use crate::polynomial::Polynomial;

const MODULUS: u64 = 23;

// Define polynomials
let p = Polynomial::<MODULUS>::new(vec![
    FieldElement::<MODULUS>::new(2),
    FieldElement::<MODULUS>::new(3),
    FieldElement::<MODULUS>::new(1),
]); // p(x) = 2 + 3x + x^2

let q = Polynomial::<MODULUS>::new(vec![
    FieldElement::<MODULUS>::new(1),
    FieldElement::<MODULUS>::new(4),
]); // q(x) = 1 + 4x

// Evaluate p at q(x)
let composed = p.compose(&q); // p(q(x)) = 2 + 3*(1 + 4x) + (1 + 4x)^2
// Compute step by step:
// 3*(1 + 4x) = 3 + 12x
// (1 + 4x)^2 = 1 + 8x + 16x^2
// Sum: 2 + (3 + 12x) + (1 + 8x + 16x^2) = 6 + 20x + 16x^2

println!("Composed Polynomial p(q(x)): {:?}", composed);

// Verify the composition
assert_eq!(composed, Polynomial::<MODULUS>::new(vec![
    FieldElement::<MODULUS>::new(6),
    FieldElement::<MODULUS>::new(20),
    FieldElement::<MODULUS>::new(16),
]));

println!("Composition verified successfully.");
```

## Traits and Implementations

### Operator Overloading

The `Polynomial` struct implements various traits from the `std::ops` module to allow for intuitive arithmetic operations.

- **`Add`, `AddAssign`**: For polynomial addition.
- **`Sub`, `SubAssign`**: For polynomial subtraction.
- **`Mul`, `MulAssign`**: For polynomial and scalar multiplication.
- **`Div`, `DivAssign`**: For polynomial and scalar division.
- **`Rem`, `RemAssign`**: For polynomial remainder operations.
- **`Neg`**: For polynomial negation.

#### Example

```rust
let p1 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
]);
let p2 = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(3),
    FieldElement::<17>::new(4),
]);

// Addition
let sum = p1 + p2;

// Subtraction
let difference = p1 - p2;

// Multiplication
let product = p1 * p2;

// Negation
let neg_p1 = -p1;

println!("Sum: {:?}", sum);
println!("Difference: {:?}", difference);
println!("Product: {:?}", product);
println!("Negated p1: {:?}", neg_p1);
```

### Function Traits (`Fn`, `FnMut`, `FnOnce`)

Polynomials can be used as functions to evaluate or compose with other polynomials. This is achieved by implementing the `Fn`, `FnMut`, and `FnOnce` traits.

- **`Fn<(Polynomial<M>,)>`**: For composing with another polynomial.
- **`Fn<(T,)>` where `T: Into<FieldElement<M>>`**: For evaluating at a field element.

#### Example

```rust
let p = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
]); // p(x) = 1 + 2x

let q = Polynomial::<17>::new(vec![
    FieldElement::<17>::new(3),
    FieldElement::<17>::new(4),
]); // q(x) = 3 + 4x

// Using Fn trait for composition
let composition = p(q); // Equivalent to p.compose(&q)

println!("Composition p(q(x)): {:?}", composition);

// Using Fn trait for evaluation
let x = FieldElement::<17>::new(5);
let value = p(x); // Equivalent to p.evaluate(x)

println!("p(5) = {:?}", value);
```

### Iterators

The `Polynomial` struct implements `FromIterator`, allowing you to create polynomials from iterators of coefficients.

```rust
use std::iter::FromIterator;

let coeffs = vec![
    FieldElement::<17>::new(1),
    FieldElement::<17>::new(2),
    FieldElement::<17>::new(3),
];
let p = Polynomial::from_iter(coeffs);

println!("Polynomial from iterator: {:?}", p);
```

## Error Handling

- **Division by Zero Polynomial**: Attempting to divide by a zero polynomial will cause a panic.

    ```rust
    let p = Polynomial::<17>::new(vec![FieldElement::<17>::new(1)]);
    let zero = Polynomial::<17>::zero();
    let _ = p / zero; // Panics: "Division by zero polynomial"
    ```

- **Division with Non-zero Remainder**: If polynomial division results in a non-zero remainder, a panic occurs when using the `/` operator.

    ```rust
    let dividend = Polynomial::<17>::new(vec![
        FieldElement::<17>::new(1),
        FieldElement::<17>::new(0),
    ]);
    let divisor = Polynomial::<17>::new(vec![
        FieldElement::<17>::new(1),
    ]);
    let quotient = dividend / divisor; // Works: quotient = [1, 0]

    let dividend = Polynomial::<17>::new(vec![
        FieldElement::<17>::new(1),
        FieldElement::<17>::new(1),
    ]);
    let divisor = Polynomial::<17>::new(vec![
        FieldElement::<17>::new(2),
    ]);
    let quotient = dividend / divisor; // Panics: "Polynomial division remainder is not zero"
    ```

- **Scalar Division by Zero**: Dividing a polynomial by a scalar that is zero will cause a panic.

    ```rust
    let p = Polynomial::<17>::new(vec![FieldElement::<17>::new(1)]);
    let scalar = FieldElement::<17>::new(0);
    let _ = p / scalar; // Panics: "Division by zero in a finite field is not allowed."
    ```

## shit to change

- **Naive Multiplication**: The current multiplication implementation uses a naive nested loop approach, resulting in \( O(n^2) \) time complexity. For polynomials with large degrees, consider implementing more efficient algorithms like Karatsuba or FFT-based multiplication.

- **Division Algorithm**: The division uses a naive long division approach. Optimizing this can lead to performance improvements for polynomials with high degrees.

