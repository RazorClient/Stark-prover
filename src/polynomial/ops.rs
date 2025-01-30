use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, Neg};

use crate::FieldElement;


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

    // -------------- Division (Optional Schoolbook) ---------------
    //
    // If you need "in-place" division or remainder, you can implement
    // a function that modifies `self` = quotient, and returns a new remainder,
    // or vice versa. For now, we'll just do a standard function that returns
    // (quotient, remainder).
    //

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
            let ratio = lead_rem / den_lead;

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

impl<const M: u64> Mul for Polynomial<M> {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self.mul_assign(&rhs);
        self
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
            return Err("Polynomial division remainder is not zero".into());
        }
        q
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
impl<const M: u64> Eq for Polynomial<M> {}
