use crate::fields::FieldElement;
use crate::polynomial::Polynomial;
use crate::{poly,fe,field};
// add ntt and pararell version latter



pub fn gen_polynomial_from_roots<const M: u64>(roots: &[FieldElement<M>]) -> Polynomial<M> {
    if roots.is_empty() {
        return Polynomial::zero(); 
    }

    // Start with p(x) = 1
    let mut p = poly![1];

    // Multiply p by (x - root) for each root
    for &root in roots {
        // Using the macro and your operator overloading
        p = p * poly![-root, FieldElement::one()];
    }

    p
}


/// Return the Lagrange basis polynomials [L0, L1, ..., L_{n-1}],
/// 
///    1) Let Vandermonde polynomial Z(x) = ∏ (x - x_j).
///    2) For each i, L_i(x) = [Z(x) / (x - x_i)] / denom_i,
///       where denom_i = ∏_{j != i} (x_i - x_j).
pub fn gen_lagrange_polynomials<const M: u64>(xs: &[FieldElement<M>]) -> Vec<Polynomial<M>> {
    let n = xs.len();
    if n == 0 {
        return vec![];
    }
    // 1)  Z(x) = ∏ (x - x_j).
    let Z = gen_polynomial_from_roots(xs);

    // 2) For each i, L_i(x) = (Z / (x - x_i)) * (1 / denom_i).
    let mut lagrange_vec = Vec::with_capacity(n);

    for i in 0..n {
        // Compute denom_i = ∏_{j != i} (x_i - x_j).
        let mut denom = FieldElement::one();
        for j in 0..n {
            if i == j { continue; }
            denom *= xs[i] - xs[j];
        }
        let denom_inv = denom.inverse(); 


        let divisor = gen_polynomial_from_roots(&[xs[i]]); // (x - x_i)
        let (mut li, rem) = Z.div_rem(&divisor);
        if !rem.is_zero(){
            panic!("Z(x) should be divisible by (x - x_i)");
        }
 
        li.scalar_mul(denom_inv);
        lagrange_vec.push(li);
    }

    lagrange_vec
}

/// Interpolate polynomial f of degree < n that satisfies
/// f(xs[i]) = ys[i] for i = 0..n-1.
///
/// Lagrange formula: f(x) = ∑ y_i * L_i(x).
pub fn interpolate_lagrange_polynomials<const M: u64>(
    xs: &[FieldElement<M>],
    ys: &[FieldElement<M>]
) -> Polynomial<M> {

    if xs.len() != ys.len() {
        panic!(
            "Mismatched x and y lengths: xs.len() = {}, ys.len() = {}",
            xs.len(),
            ys.len()
        );
    }
    let n = xs.len();
    if n == 0 {
        // Return zero polynomial if no points
        return Polynomial::zero();
    }
    // Compute the basis polynomials L_i(x).
    let l = gen_lagrange_polynomials(xs);

    // Sum up: f(x) = Σ (ys[i] * L[i](x)).
    let mut acc = Polynomial::zero();
    for i in 0..n {
        // clone L[i], multiply by ys[i].
        let mut term = l[i].clone();
        term.scalar_mul(ys[i]);
        // add to accumulator
        acc.add_assign(&term);
    }
    acc
}

    #[cfg(test)]
    mod test_interpol {
        use super::*;
        use itertools::Itertools; // for collect_vec()
        use rand::Rng;
        use std::collections::HashSet;
    
        #[test]
        fn test_gen_polynomial_from_roots() {

        let roots = vec![
            fe!(7, 1),
            fe!(7, 2),
            fe!(7, 3),
        ];
        let polynomial = gen_polynomial_from_roots(&roots);

        field!(F7,7);
        assert_eq!(polynomial.coefficients[0],F7::new(1));
        assert_eq!(polynomial.coefficients[1],F7::new(4));
        assert_eq!(polynomial.coefficients[2],F7::new(1));
        assert_eq!(polynomial.coefficients[3],F7::new(1));
        }
        #[test]
        fn test_gen_lagrange_poly() {
            // Create the x-values using the fe! macro.
            let xs = vec![
                fe!(7, 2),
                fe!(7, 3),
                fe!(7, 5),
            ];
    

        let lagrange_polys = gen_lagrange_polynomials(&xs);
        assert_eq!(lagrange_polys.len(), xs.len());

            for (i, &xi) in xs.iter().enumerate() {
                for (j, &xj) in xs.iter().enumerate() {
                    let eval = lagrange_polys[i].evaluate(xj);
                    if i == j {
                        // At its own node, the basis polynomial should evaluate to 1.
                        assert_eq!(
                            eval,
                            FieldElement::one(),
                            "Lagrange basis polynomial L_{} did not evaluate to 1 at its node (x = {:?})",
                            i,
                            xi
                        );
                    } else {
                        // At other nodes, the basis polynomial should evaluate to 0.
                        assert_eq!(
                            eval,
                            FieldElement::zero(),
                            "Lagrange basis polynomial L_{} did not evaluate to 0 at x = {:?}",
                            i,
                            xj
                        );
                    }
                }
    }}

}

