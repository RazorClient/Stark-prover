use std::ops::{Add,AddAssign ,Sub,SubAssign, Mul, MulAssign, Div,DivAssign,Neg,Rem,RemAssign};
use std::ops::{Fn, FnMut, FnOnce};


use crate::fields::FieldElement;


/// - `coefficients[i]` = coefficient for x^i term.
#[derive(Clone, Debug)]
pub struct Polynomial<const MODULUS: u64> {
    pub coefficients: Vec<FieldElement<MODULUS>>,
    pub degree: isize,
}

impl<const MODULUS: u64> Polynomial<MODULUS> {


    /// automatically trim trailing zeros and sets `self.degree`.
    pub fn new(mut coeffs: Vec<FieldElement<MODULUS>>) -> Self {
        // Trim trailing zeros
        while let Some(&last) = coeffs.last() {
            if last == FieldElement::zero() {
                coeffs.pop();
            } else {
                break;
            }
        }
        let deg = if coeffs.is_empty() {
            -1
        } else {
            (coeffs.len() - 1) as isize
        };
        Polynomial {
            coefficients: coeffs,
            degree: deg,
        }
    }

    pub fn zero() -> Self {
        Polynomial {
            coefficients: Vec::new(),
            degree: -1,
        }
    }


    fn update_degree(&mut self) {
        while let Some(&last) = self.coefficients.last() {
            if last == FieldElement::zero() {
                self.coefficients.pop();
            } else {
                break;
            }
        }
        self.degree = if self.coefficients.is_empty() {
            -1
        } else {
            (self.coefficients.len() - 1) as isize
        };
    }


    pub fn is_zero(&self) -> bool {
        self.degree == -1
    }

    pub fn leading_coefficient(&self) -> Option<FieldElement<MODULUS>> {
        if self.is_zero() {
            None
        } else {
            Some(self.coefficients[self.degree as usize])
        }
    }

    ///Horner's method: O(n).
    pub fn evaluate(&self, x: FieldElement<MODULUS>) -> FieldElement<MODULUS> {
        let mut result = FieldElement::zero();
        // Horner's method: result = (...((0 * x) + a_n)*x + a_{n-1})*x + ... + a_0
        for &coef in self.coefficients.iter().rev() {
            result = result * x + coef;
        }
        result
    }


    /// Add `rhs` polynomial to `self`, in-place.
    pub fn add_assign(&mut self, rhs: &Self) {
        if rhs.is_zero() {
            return;
        }
        let max_len = std::cmp::max(self.coefficients.len(), rhs.coefficients.len());
        self.coefficients.resize(max_len, FieldElement::zero());

        for i in 0..rhs.coefficients.len() {
            self.coefficients[i] += rhs.coefficients[i];
        }
        self.update_degree();
    }

    /// Subtract `rhs` polynomial from `self`, in-place.
    pub fn sub_assign(&mut self, rhs: &Self) {
        if rhs.is_zero() {
            return; 
        }
        let max_len = std::cmp::max(self.coefficients.len(), rhs.coefficients.len());
        self.coefficients.resize(max_len, FieldElement::zero());

        for i in 0..rhs.coefficients.len() {
            self.coefficients[i] -= rhs.coefficients[i];
        }
        self.update_degree();
    }

    pub fn mul_assign(&mut self, rhs: &Self) {
        if self.is_zero() {
            return; 
        }
        if rhs.is_zero() {
            *self = Self::zero();
            return;
        }
        let new_len = self.coefficients.len() + rhs.coefficients.len() - 1;

        let mut product = vec![FieldElement::zero(); new_len];

        // Naive nested loop
        for (i, &a) in self.coefficients.iter().enumerate() {
            if a == FieldElement::zero() {
                continue;
            }
            for (j, &b) in rhs.coefficients.iter().enumerate() {
                product[i + j] += a * b;
            }
        }

        self.coefficients = product;
        self.update_degree();
    }
    
    /// Returns (quotient, remainder) using naive polynomial long division.
    pub fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        if rhs.is_zero() {
            panic!("Division by zero polynomial");
        }
        if self.is_zero() || self.degree < rhs.degree {
            return (Self::zero(), self.clone()); 
        }

        let mut rem = self.coefficients.clone();
        let mut rem_deg = self.degree;

        // quotient has length (deg(self)-deg(rhs)+1)
        let q_len = (self.degree - rhs.degree + 1) as usize;
        let mut quotient = vec![FieldElement::zero(); q_len];

        let den_lead = rhs.coefficients[rhs.degree as usize]; 
        let den_deg = rhs.degree;

        while rem_deg >= den_deg && rem_deg != -1 {
            let lead_rem = rem[rem_deg as usize];
            let ratio = lead_rem * den_lead.inverse();


            // shift for subtracting from remainder
            let shift = (rem_deg - den_deg) as usize;
            quotient[shift] = quotient[shift] + ratio;

            // subtract (ratio * x^shift * rhs) from remainder
            for i in 0..=den_deg as usize {
                rem[i + shift] = rem[i + shift] - (ratio * rhs.coefficients[i]);
            }

            // Trim new remainder's leading zeros
            while let Some(&last) = rem.last() {
                if last == FieldElement::zero() {
                    rem.pop();
                } else {
                    break;
                }
            }
            rem_deg = if rem.is_empty() {
                -1
            } else {
                (rem.len() - 1) as isize
            };
        }

        let quot_poly = Polynomial::new(quotient);
        let rem_poly = Polynomial::new(rem);
        (quot_poly, rem_poly)
    }

    /// Scalar multiplication in-place
    pub fn scalar_mul(&mut self, scalar: FieldElement<MODULUS>) {
            for coef in self.coefficients.iter_mut() {
                *coef *= scalar;
            }
        }

    /// Scalar division in-place
    pub fn scalar_div(&mut self, scalar: FieldElement<MODULUS>) {
        if scalar == FieldElement::<MODULUS>::zero() {
            panic!("Division by zero in a finite field is not allowed.");
        }
        
        let scalar_inv = scalar.inverse();
        for coef in self.coefficients.iter_mut() {
            *coef *= scalar_inv;
        }
    }

    /// Compose `self` with `other`: return `self(other)`.
    /// i.e. p(q) = sum_{i=0}^degree( coeff[i] * [q(x)]^i ).
    pub fn compose(&self, other: &Polynomial<MODULUS>) -> Polynomial<MODULUS> {
        if self.is_zero() {
            return Polynomial::zero();
        }

        // We'll do Horner's approach from highest power to lowest:
        //   p(x) = a_n x^n + ... + a_1 x + a_0
        //   p(q) = (((0 * q) + a_n)*q + a_{n-1})*q + ... + a_0
        let mut result = Polynomial::zero();
        for &coeff in self.coefficients.iter().rev() {
            // result = result * other + coeff
            if !result.is_zero() {
                let mut temp = result.clone();
                temp.mul_assign(other);
                // add constant 'coeff'
                temp.add_assign(&Polynomial::new(vec![coeff]));
                result = temp;
            } else {
                // When result=0, result*q + coeff = [const polynomial with 'coeff']
                result = Polynomial::new(vec![coeff]);
            }
        }
        result
    }


}

///trait

impl<const M: u64> Add for Polynomial<M> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.add_assign(&rhs);
        self
    }
}

impl<const M: u64> Sub for Polynomial<M> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self.sub_assign(&rhs);
        self
    }
}
impl<const M: u64> AddAssign for Polynomial<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
    }
}

impl<const M: u64> SubAssign for Polynomial<M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs);
    }
}

impl<const M: u64> Mul for Polynomial<M> {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self.mul_assign(&rhs);
        self
    }
}


impl<const M: u64> Mul<FieldElement<M>> for Polynomial<M> {
    type Output = Self;
    fn mul(mut self, scalar: FieldElement<M>) -> Self::Output {
        for coef in self.coefficients.iter_mut() {
            *coef *= scalar;
        }
        self
    }
}

impl<const M: u64> MulAssign<FieldElement<M>> for Polynomial<M> {
    fn mul_assign(&mut self, scalar: FieldElement<M>) {
        self.scalar_mul(scalar);
    }
}

impl<const M: u64> MulAssign<Polynomial<M>> for Polynomial<M> {
    fn mul_assign(&mut self, rhs: Polynomial<M>) {
        *self = self.clone() * rhs; // Use your existing `Mul` implementation
    }
}

impl<const M: u64> Neg for Polynomial<M> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for c in self.coefficients.iter_mut() {
            *c = -*c;
        }
        self
    }
}

impl<const M: u64> Div for Polynomial<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let (q, r) = self.div_rem(&rhs);
        if !r.is_zero() {
            panic!("Polynomial division remainder is not zero");
        }
        q
    }
}


impl<const M: u64> Div<FieldElement<M>> for Polynomial<M> {
    type Output = Self;
    fn div(mut self, scalar: FieldElement<M>) -> Self::Output {
        self.div_assign(scalar);
        self
    }
}

impl<const M: u64> DivAssign<FieldElement<M>> for Polynomial<M> {
    fn div_assign(&mut self, scalar: FieldElement<M>) {
        self.scalar_div(scalar);
    }
}


impl<const M: u64> PartialEq for Polynomial<M> {
    fn eq(&self, other: &Self) -> bool {
        if self.degree != other.degree {
            return false;
        }
        if self.degree == -1 {
            return true;
        }
        for i in 0..=(self.degree as usize) {
            if self.coefficients[i] != other.coefficients[i] {
                return false;
            }
        }
        true
    }
}

// Eq deries from PartialEq
impl<const M: u64> Eq for Polynomial<M> {}

impl<const M: u64> Mul<Polynomial<M>> for FieldElement<M> {
    type Output = Polynomial<M>;
    
    fn mul(self, mut poly: Polynomial<M>) -> Polynomial<M> {
        for coef in poly.coefficients.iter_mut() {
            *coef *= self; // 
        }
        poly
    }
}


impl<const MODULUS: u64> Rem for Polynomial<MODULUS> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        // Use your existing div_rem function
        let (_, remainder) = self.div_rem(&rhs);
        remainder
    }
}


impl<const MODULUS: u64> RemAssign for Polynomial<MODULUS> {
    fn rem_assign(&mut self, rhs: Self) {
        let (_, remainder) = self.clone().div_rem(&rhs);
        *self = remainder;
    }
}

// some nightly rust problems
impl<const M: u64> FnOnce<(Polynomial<M>,)> for Polynomial<M> {
    type Output = Polynomial<M>;

    extern "rust-call" fn call_once(self, args: (Polynomial<M>,)) -> Self::Output {
        self.compose(&args.0)
    }
}

impl<const M: u64> FnMut<(Polynomial<M>,)> for Polynomial<M> {
    extern "rust-call" fn call_mut(&mut self, args: (Polynomial<M>,)) -> Self::Output {
        // FnMut means self is mutable reference
        self.compose(&args.0)
    }
}

impl<const M: u64> Fn<(Polynomial<M>,)> for Polynomial<M> {
    extern "rust-call" fn call(&self, args: (Polynomial<M>,)) -> Self::Output {
        // Fn means self is an immutable reference
        self.compose(&args.0)
    }
}

impl<const M: u64, T: Into<FieldElement<M>>> FnOnce<(T,)> for Polynomial<M> {
    type Output = FieldElement<M>;

    extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
        self.evaluate(args.0.into())
    }
}

impl<const M: u64, T: Into<FieldElement<M>>> FnMut<(T,)> for Polynomial<M> {
    extern "rust-call" fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.evaluate(args.0.into())
    }
}

impl<const M: u64, T: Into<FieldElement<M>>> Fn<(T,)> for Polynomial<M> {
    extern "rust-call" fn call(&self, args: (T,)) -> Self::Output {
        self.evaluate(args.0.into())
    }
}


// imp iterator

impl<const M: u64> FromIterator<FieldElement<M>> for Polynomial<M> {
    fn from_iter<T: IntoIterator<Item = FieldElement<M>>>(iter: T) -> Self {
        let coeffs: Vec<FieldElement<M>> = iter.into_iter().collect();
        Self::new(coeffs)
    }
}

// //tests
#[cfg(test)]
mod test_polynomials {
    use super::*;
    // use rand::Rng;




    fn generate_random_polynomial(degree: usize) -> Polynomial<7> {
        let mut coeffs: Vec<FieldElement<7>> =
            (0..degree).map(|_| FieldElement::random()).collect();
        let mut leading = FieldElement::zero();
        while leading == FieldElement::zero() {
            leading = FieldElement::random();
        }
        coeffs.push(leading);
        Polynomial::new(coeffs)
    }

 
    #[test]
    fn test_zero_polynomial() {
        let poly: Polynomial<7> = Polynomial::zero();
        assert_eq!(poly.degree, -1);
        assert!(poly.is_zero());
    }

    #[test]
    fn test_evaluate_zero_polynomial() {
        let field = FieldElement::<7>::zero();
        let poly = Polynomial::zero();
        assert_eq!(poly.evaluate(field), FieldElement::zero());
    }

    #[test]
    fn test_evaluate_constant_polynomial() {
        let field = FieldElement::<7>::zero();
        let coeffs = vec![FieldElement::new(5)];
        let poly = Polynomial::new(coeffs);
        assert_eq!(poly.evaluate(field), FieldElement::new(5));
    }

    #[test]
    fn test_poly_addition() {
        let coeffs1 = vec![FieldElement::<7>::new(2), FieldElement::new(3)];
        let coeffs2 = vec![FieldElement::<7>::new(4), FieldElement::new(1)];
        let poly1 = Polynomial::new(coeffs1);
        let poly2 = Polynomial::new(coeffs2);
        let result = poly1 + poly2;
        assert_eq!(result.coefficients[0], FieldElement::new(6));
        assert_eq!(result.coefficients[1], FieldElement::new(4));
    }

    #[test]
    fn test_poly_subtraction() {
        let coeffs1 = vec![FieldElement::<7>::new(6), FieldElement::<7>::new(5)];
        let coeffs2 = vec![FieldElement::<7>::new(4), FieldElement::<7>::new(3)];
        let poly1 = Polynomial::new(coeffs1);
        let poly2 = Polynomial::new(coeffs2);
        let result = poly1 - poly2;
    
        // (6 + 5x) - (4 + 3x) = (6 - 4) + (5 - 3)x = 2 + 2x mod 7
        assert_eq!(result.coefficients[0], FieldElement::<7>::new(2)); // (6 - 4) mod 7 = 2
        assert_eq!(result.coefficients[1], FieldElement::<7>::new(2)); // (5 - 3) mod 7 = 2
    }
    
    #[test]
    fn test_poly_multiplication() {
        let coeffs1 = vec![FieldElement::<7>::new(1), FieldElement::<7>::new(2)];
        let coeffs2 = vec![FieldElement::<7>::new(3), FieldElement::<7>::new(4)];
        let poly1 = Polynomial::new(coeffs1);
        let poly2 = Polynomial::new(coeffs2);
        let result = poly1 * poly2;
    
        // (1 + 2x) * (3 + 4x) mod 7:
        // 1*3 + (1*4 + 2*3)x + (2*4)x^2 = 3 + 10x + 8x^2
        // mod 7 => 3 + 3x + x^2
        assert_eq!(result.coefficients.len(), 3);
        assert_eq!(result.coefficients[0], FieldElement::<7>::new(3)); // 3 mod 7 = 3
        assert_eq!(result.coefficients[1], FieldElement::<7>::new(3)); // 10 mod 7 = 3
        assert_eq!(result.coefficients[2], FieldElement::<7>::new(1)); // 8 mod 7 = 1
    }
    

    #[test]
    fn test_poly_scalar_multiplication() {
        let coeffs = vec![FieldElement::<7>::new(2), FieldElement::<7>::new(3)];
        let poly = Polynomial::new(coeffs);
        let scalar = FieldElement::<7>::new(4);
        let result = poly * scalar;
    
        // (2 + 3x) * 4 mod 7:
        // (2 * 4) + (3 * 4)x = 8 + 12x
        // mod 7 => 1 + 5x
        assert_eq!(result.coefficients[0], FieldElement::<7>::new(1)); // 8 mod 7 = 1
        assert_eq!(result.coefficients[1], FieldElement::<7>::new(5)); // 12 mod 7 = 5
    }
    
    #[test]
    fn test_poly_division() {
        let coeffs1 = vec![
            FieldElement::<7>::new(1), // Constant term
            FieldElement::<7>::new(3), // x term
            FieldElement::<7>::new(2), // x^2 term
        ];
        let coeffs2 = vec![
            FieldElement::<7>::new(1), // Constant term
            FieldElement::<7>::new(1), // x term
        ];
        let poly1 = Polynomial::new(coeffs1);
        let poly2 = Polynomial::new(coeffs2);
        let (q, r) = poly1.div_rem(&poly2);
    
        // (1 + 3x + 2x^2) / (1 + x)
        // Expected quotient: 2x + 1 (mod 7)
        // Expected remainder: 0
    
        assert_eq!(q.coefficients.len(), 2, "Quotient should be of degree 1.");
        assert_eq!(q.coefficients[0], FieldElement::<7>::new(1), "Constant term of quotient should be 1.");
        assert_eq!(q.coefficients[1], FieldElement::<7>::new(2), "x term of quotient should be 2.");
        assert!(r.is_zero(), "Remainder should be zero.");
    }
    
    
    #[test]
    fn test_poly_composition() {
        let coeffs1 = vec![FieldElement::<7>::new(1), FieldElement::<7>::new(1)]; // 1 + x
        let coeffs2 = vec![FieldElement::<7>::new(2), FieldElement::<7>::new(3)]; // 2 + 3x
        let poly1 = Polynomial::new(coeffs1);
        let poly2 = Polynomial::new(coeffs2);
        let result = poly1.compose(&poly2);
    
        // Expected result: 3 + 3x (mod 7)
        assert_eq!(result.coefficients.len(), 2, "Composition should yield a polynomial of degree 1.");
        assert_eq!(result.coefficients[0], FieldElement::<7>::new(3), "Constant term should be 3.");
        assert_eq!(result.coefficients[1], FieldElement::<7>::new(3), "x term should be 3.");
    }
    

    #[test]
    fn test_poly_random_generation() {
        let poly = generate_random_polynomial(5);
        assert_eq!(poly.degree, 5);
    }

    #[test]
    fn test_create_with_empty_coeffs() {
        // Creating a polynomial with an empty vector should result in a zero polynomial.
        let poly: Polynomial<7> = Polynomial::new(vec![]);
        
        assert!(poly.is_zero(), "A polynomial created with empty coefficients should be zero.");
        assert_eq!(poly.coefficients.len(), 0, "Zero polynomial should have no coefficients.");
        assert_eq!(poly.degree, -1, "Degree of zero polynomial should be -1 by convention.");
    }
    

    #[test]
    fn test_create_with_trailing_zeros() {
        let coeffs = vec![
            FieldElement::<7>::new(1),
            FieldElement::<7>::new(2),
            FieldElement::<7>::zero(),
            FieldElement::<7>::zero(),
        ];
        let poly = Polynomial::new(coeffs);
    
        assert_eq!(poly.degree, 1, "Polynomial degree should match the last nonzero coefficient.");
        assert_eq!(poly.coefficients.len(), 2, "Trailing zeros should be removed.");
        assert_eq!(poly.coefficients[0], FieldElement::<7>::new(1));
        assert_eq!(poly.coefficients[1], FieldElement::<7>::new(2));
    }
    
    #[test]
    fn test_is_zero() {
        let zero_poly = Polynomial::<7>::zero();
        assert!(zero_poly.is_zero(), "Zero polynomial should return true for is_zero().");
    
        let non_zero_poly = Polynomial::new(vec![FieldElement::<7>::new(0), FieldElement::<7>::new(1)]);
        assert!(!non_zero_poly.is_zero(), "Polynomial with nonzero terms should not be considered zero.");
    }
    
    #[test]
    fn test_leading_coefficient() {
        let poly = Polynomial::new(vec![FieldElement::<7>::new(2), FieldElement::<7>::new(5)]);
        assert_eq!(
            poly.leading_coefficient().unwrap(),
            FieldElement::<7>::new(5),
            "Leading coefficient should be the highest-degree term."
        );
    
        let zero_poly = Polynomial::<7>::zero();
        assert_eq!(
            zero_poly.leading_coefficient(),
            None,
            "Leading coefficient of zero polynomial should be None."
        );
    }
    

    #[test]
    fn test_add_zero_polynomial() {
        let p = Polynomial::new(vec![FieldElement::<7>::new(3), FieldElement::<7>::new(4)]);
        let zero_p = Polynomial::zero();
        let result = p.clone() + zero_p;
        assert_eq!(result, p);
    }

    #[test]
    fn test_sub_zero_polynomial() {
        let p = Polynomial::new(vec![FieldElement::<7>::new(3), FieldElement::<7>::new(4)]);
        let zero_p = Polynomial::zero();
        let result = p.clone() - zero_p;
    
        assert_eq!(result, p, "Subtracting zero polynomial should not change the polynomial.");
    }
    

    #[test]
    fn test_add_assign() {
        let mut p = Polynomial::new(vec![FieldElement::<7>::new(1), FieldElement::<7>::new(2)]);
        let q = Polynomial::new(vec![FieldElement::<7>::new(2), FieldElement::<7>::new(3)]);
        p += q;
    
        // (1 + 2x) + (2 + 3x) = 3 + 5x (mod 7)
        assert_eq!(p.coefficients[0], FieldElement::<7>::new(3));
        assert_eq!(p.coefficients[1], FieldElement::<7>::new(5));
    }
    

    #[test]
    fn test_sub_assign() {
        let mut p = Polynomial::new(vec![FieldElement::<7>::new(3), FieldElement::<7>::new(5)]);
        let q = Polynomial::new(vec![FieldElement::<7>::new(2), FieldElement::<7>::new(3)]);
        p -= q;
    
        // (3 + 5x) - (2 + 3x) = 1 + 2x (mod 7)
        assert_eq!(p.coefficients[0], FieldElement::<7>::new(1));
        assert_eq!(p.coefficients[1], FieldElement::<7>::new(2));
    }
    
    
#[test]
fn test_mul_by_zero_polynomial() {
    let p = Polynomial::new(vec![FieldElement::<7>::new(1), FieldElement::<7>::new(2)]);
    let zero_p = Polynomial::zero();
    let result = p * zero_p;

    assert!(result.is_zero(), "Multiplication by zero polynomial should result in a zero polynomial.");
}


#[test]
fn test_mul_assign_polynomial() {
    let mut p = Polynomial::new(vec![FieldElement::<7>::new(1), FieldElement::<7>::new(2)]);
    let q = Polynomial::new(vec![FieldElement::<7>::new(2), FieldElement::<7>::new(1)]);

    p *= q;

    // (1 + 2x) * (2 + x) = 2 + 5x + 2x^2 (mod 7)
    assert_eq!(p.coefficients.len(), 3);
    assert_eq!(p.coefficients[0], FieldElement::<7>::new(2));
    assert_eq!(p.coefficients[1], FieldElement::<7>::new(5));
    assert_eq!(p.coefficients[2], FieldElement::<7>::new(2));
}


#[test]
fn test_poly_scalar_division() {
    let coeffs = vec![FieldElement::<7>::new(2), FieldElement::<7>::new(4)];
    let mut poly = Polynomial::new(coeffs);
    let scalar = FieldElement::<7>::new(2);
    poly.scalar_div(scalar);

    // (2 + 4x) / 2 = (1 + 2x) (mod 7)
    assert_eq!(poly.coefficients[0], FieldElement::<7>::new(1));
    assert_eq!(poly.coefficients[1], FieldElement::<7>::new(2));
}

    #[test]
    #[should_panic]
    fn test_poly_scalar_div_by_zero() {
        let coeffs = vec![FieldElement::<7>::new(2), FieldElement::<7>::new(4)];
        let mut poly = Polynomial::new(coeffs);
        let zero = FieldElement::zero();
        poly.scalar_div(zero);
    }

#[test]
fn test_poly_div_rem_no_remainder() {
    let coeffs1 = vec![
        FieldElement::<7>::new(1),
        FieldElement::<7>::new(3),
        FieldElement::<7>::new(2)
    ];
    let coeffs2 = vec![
        FieldElement::<7>::new(1),
        FieldElement::<7>::new(1)
    ];
    let poly1 = Polynomial::new(coeffs1);
    let poly2 = Polynomial::new(coeffs2);
    let (q, r) = poly1.div_rem(&poly2);

    // Expected quotient: 2x + 1 (mod 7)
    assert_eq!(q.coefficients[0], FieldElement::<7>::new(1));
    assert_eq!(q.coefficients[1], FieldElement::<7>::new(2));
    assert!(r.is_zero(), "The remainder should be zero.");
}

#[test]
fn test_poly_rem_operator() {
    let coeffs1 = vec![
        FieldElement::<7>::new(2),
        FieldElement::<7>::new(5),
        FieldElement::<7>::new(3)
    ];
    let coeffs2 = vec![
        FieldElement::<7>::new(1),
        FieldElement::<7>::new(1)
    ];
    let poly1 = Polynomial::new(coeffs1);
    let poly2 = Polynomial::new(coeffs2);
    let remainder = poly1.clone() % poly2.clone();
    let (_, r) = poly1.div_rem(&poly2);

    assert_eq!(remainder, r, "Modulo operator should return the correct remainder.");
}

#[test]
fn test_poly_rem_assign() {
    let coeffs1 = vec![
        FieldElement::<7>::new(1),
        FieldElement::<7>::new(1),
        FieldElement::<7>::new(1)
    ];
    let coeffs2 = vec![
        FieldElement::<7>::new(1),
        FieldElement::<7>::new(1)
    ];
    let mut p = Polynomial::new(coeffs1);
    let q = Polynomial::new(coeffs2);

    p %= q.clone();
    let (_, r) = Polynomial::new(vec![
        FieldElement::<7>::new(1),
        FieldElement::<7>::new(1),
        FieldElement::<7>::new(1),
    ]).div_rem(&q);

    assert_eq!(p, r, "Polynomial %= operator should correctly compute remainder.");
}


    #[test]
    fn test_poly_div_rem_nontrivial() {
        let coeffs1 = vec![FieldElement::new(2), FieldElement::new(5), FieldElement::new(3)];
        let coeffs2 = vec![FieldElement::new(1), FieldElement::new(1)];
        let poly1 = Polynomial::new(coeffs1);
        let poly2 = Polynomial::new(coeffs2);
        let (q, r) = poly1.div_rem(&poly2);
        let reconstructed = (&poly2 * q.clone()) + r.clone();
        assert_eq!(reconstructed, poly1);
    }

    #[test]
    #[should_panic]
    fn test_poly_div_by_zero_polynomial() {
        let p = Polynomial::new(vec![FieldElement::<7>::new(1), FieldElement::<7>::new(2)]);
        let zero_p = Polynomial::zero();
        let _ = p.div_rem(&zero_p);
    }

    #[test]
    fn test_poly_div_assign_scalar() {
        let mut p = Polynomial::new(vec![FieldElement::<7>::new(2), FieldElement::<7>::new(4)]);
        p /= FieldElement::new(2);
        assert_eq!(p.coefficients[0], FieldElement::<7>::new(1));
        assert_eq!(p.coefficients[1], FieldElement::<7>::new(2));
    }




//     #[test]
//     fn test_neg() {
//         let p = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(2)]);
//         let neg_p = -p.clone();
//         let sum = p + neg_p;
//         assert!(sum.is_zero());
//     }

//     #[test]
//     fn test_partial_eq_diff_length() {
//         let p = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(2)]);
//         let q = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(2), FieldElement::new(3)]);
//         assert_ne!(p, q);
//     }

//     #[test]
//     fn test_partial_eq_diff_coeff() {
//         let p = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(2)]);
//         let q = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(3)]);
//         assert_ne!(p, q);
//     }

//     #[test]
//     fn test_compose_with_zero() {
//         let p = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(1)]);
//         let zero_poly = Polynomial::zero();
//         let result = p.compose(&zero_poly);
//         assert_eq!(result.degree, 0);
//         assert_eq!(result.coefficients[0], FieldElement::new(1));
//     }

//     #[test]
//     fn test_compose_with_constant() {
//         // p(x)= x + 2, c=5 => p(5)=7=>0 mod7 => constant poly 0.
//         let p = Polynomial::new(vec![FieldElement::new(2), FieldElement::new(1)]);
//         let constant_poly = Polynomial::new(vec![FieldElement::new(5)]);
//         let result = p.compose(&constant_poly);
//         assert_eq!(result.degree, 0);
//         assert_eq!(result.coefficients[0], FieldElement::zero());
//     }

//     #[test]
//     fn test_compose_additional() {
//         // p1(x)=1 + x, p2(x)=2 + 3x => p1(p2(x))=3+3x
//         let coeffs1 = vec![FieldElement::new(1), FieldElement::new(1)];
//         let coeffs2 = vec![FieldElement::new(2), FieldElement::new(3)];
//         let poly1 = Polynomial::new(coeffs1);
//         let poly2 = Polynomial::new(coeffs2);
//         let result = poly1.compose(&poly2);
//         assert_eq!(result.degree, 1);
//         assert_eq!(result.coefficients[0], FieldElement::new(3));
//         assert_eq!(result.coefficients[1], FieldElement::new(3));
//     }

//     // #[test]
//     // fn test_evaluate_random_points() {
//     //     let mut rng = rand::thread_rng();
//     //     for _ in 0..5 {
//     //         let degree = rng.gen_range(0..5);
//     //         let poly = generate_random_polynomial(degree);
//     //         let x_val = FieldElement::<7>::random_element(&[]);
//     //         let _ = poly.evaluate(x_val);
//     //     }
//     // }

//     #[test]
//     fn test_div_rem_random_polys() {
//         let mut rng = rand::thread_rng();
//         for _ in 0..5 {
//             let deg_a = rng.gen_range(0..5);
//             let deg_b = rng.gen_range(0..5);
//             let poly_a = generate_random_polynomial(deg_a);
//             let poly_b = generate_random_polynomial(deg_b);
//             if poly_b.is_zero() {
//                 continue;
//             }
//             let (q, r) = poly_a.div_rem(&poly_b);
//             if !r.is_zero() {
//                 assert!(r.degree < poly_b.degree);
//             }
//             let rebuilt = (&poly_b * q.clone()) + r.clone();
//             assert_eq!(rebuilt, poly_a);
//         }
//     }

//     #[test]
//     fn test_from_iter() {
//         let elems = vec![FieldElement::new(1), FieldElement::new(2), FieldElement::new(0)];
//         let poly: Polynomial<7> = elems.into_iter().collect();
//         assert_eq!(poly.degree, 1);
//         assert_eq!(poly.coefficients.len(), 2);
//         assert_eq!(poly.coefficients[0], FieldElement::new(1));
//         assert_eq!(poly.coefficients[1], FieldElement::new(2));
//     }
}
