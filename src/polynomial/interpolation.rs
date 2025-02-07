use crate::fields::FieldElement;
use crate::polynomial::Polynomial;
use crate::{poly,fe,field};
use rayon::prelude::*;
// add ntt version latter



pub fn gen_polynomial_from_roots<const M: u64>(roots: &[FieldElement<M>]) -> Polynomial<M> {
    if roots.is_empty() {
        return Polynomial::zero(); 
    }

    // Start with p(x) = 1
    let mut p = poly![1];

    // Multiply p by (x - root) for each root
    for &root in roots {
        p = p * poly![-root, FieldElement::one()];
    }

    p
}
// /// nlogn but slower lmao 
// pub fn polynomial_from_roots<const M: u64>(roots: &[FieldElement<M>]) -> Polynomial<M> {
//     if roots.is_empty() {
//         return Polynomial::zero();
//     }
//     if roots.len() == 1 {
//         return poly![-roots[0], FieldElement::one()];
//     }

//     let mid = roots.len() / 2;
//     let left = polynomial_from_roots(&roots[..mid]);
//     let right = polynomial_from_roots(&roots[mid..]);

//     left * right
// }


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

pub fn gen_lagrange_polynomials_parallel<const M: u64>(roots: &[FieldElement<M>]) -> Vec<Polynomial<M>> {
    let n = roots.len();
    if n == 0 {
        return vec![];
    }
    // 1)  Z(x) = ∏ (x - x_j).
    // 1)  Z(x) = ∏ (x - x_j).
    let Z = gen_polynomial_from_roots(roots);
        // Step 2: For each i, compute L_i(x) in parallel
        (0..n)
        .into_par_iter() 
        .map(|i| {
            // Compute denom_i
            let mut denom = FieldElement::one();
            for j in 0..n {
                if i != j {
                    denom *= roots[i] - roots[j];
                }
            }
            let denom_inv = denom.inverse();

            // Divide Z by (x - x_i)
            let divisor = gen_polynomial_from_roots(&[roots[i]]);
            let (mut li, rem) = Z.div_rem(&divisor);
            if !rem.is_zero() {
                panic!("Z(x) should be divisible by (x - x_i)");
            }
            li.scalar_mul(denom_inv);

            li
        })
        .collect()



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
    // use the pararell version 
    
    let l = gen_lagrange_polynomials_parallel(xs);

    // Sum up: f(x) = Σ (ys[i] * L[i](x)).
    let mut acc = Polynomial::zero();
    for i in 0..n {
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
         // for collect_vec()
        
        

        pub fn generate_random_polynomial<const M: u64>(degree: usize) -> Polynomial<M> {
            let mut coeffs = Vec::with_capacity(degree + 1);
            for _ in 0..=degree {
                coeffs.push(FieldElement::<M>::random());
            }
            Polynomial::new(coeffs)
        }
    
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

//    #[test]
//     fn test_poly_interpolation() {
//         let mut rng = rand::thread_rng();
//         for _ in 0..10 {
//             // Random degree up to 100
//             let degree = rng.gen_range(0..100);

//             // Use your helper function to get a random polynomial of that degree.
//             let p = generate_random_polynomial::<7>(degree);

//             // Collect at least degree+1 distinct random x-values.
//             let mut x_values_set = HashSet::new();
//             while x_values_set.len() < degree + 1 {
//                 // Replace this call with the valid method in your code for random field elements:
//                 x_values_set.insert(FieldElement::<7>::random());
//             }
//             let x_values: Vec<_> = x_values_set.into_iter().collect_vec();

//             // Evaluate p on these x-values.
//             let y_values: Vec<_> = x_values
//                 .iter()
//                 .map(|&x| p.evaluate(x))
//                 .collect_vec();

//             // Interpolate the polynomial from the points (x_values, y_values).
//             let interpolated_p = interpolate_lagrange_polynomials(&x_values, &y_values);

//             assert_eq!(
//                 p, interpolated_p,
//                 "Polynomial interpolation failed for a randomly generated polynomial of degree {}",
//                 degree
//             );
//         }
//     }

#[test]
fn test_gen_lagrange_poly2() {
    let x = vec![
        fe!(7, 2),
        fe!(7, 3),
        fe!(7, 5),fe!(7,6)
    ];

    let lagrange_polynomials = gen_lagrange_polynomials(&x);
    assert_eq!(lagrange_polynomials.len(), 4);

    for (i, &xi) in x.iter().enumerate() {
        for (j, &xj) in x.iter().enumerate() {
            let eval = lagrange_polynomials[i].evaluate(xj);
            if i == j {
                // Should be 1 at its own node
                assert_eq!(eval, FieldElement::one());
            } else {
                // Should be 0 at others
                assert_eq!(eval, FieldElement::zero());
            }
        }
    }
    
}

#[test]
    fn test_interpolate_lagrange_polynomials() {
        let x = vec![
            fe!(7, 2),
            fe!(7, 3),
            fe!(7, 5)
        ];

        let y = vec![
            fe!(7, 1),
            fe!(7, 2),
            fe!(7, 3)
        ];
        let result = interpolate_lagrange_polynomials(&x, &y);

        // Expected polynomial in GF(7) has coefficients: constant term = 5,
        // x term = 3, x^2 term = 1.
        assert_eq!(result.coefficients[0],fe!(7, 5));
        assert_eq!(result.coefficients[1],fe!(7, 3));
        assert_eq!(result.coefficients[2],fe!(7, 1));
    }

    #[test]
    fn test_gen_lagrange_poly_parallel_small() {
        // Some distinct x-values in Field7
        let x = vec![
            fe!(7, 2),
            fe!(7, 3),
            fe!(7, 5),
            fe!(7, 6),
        ];


        let lagrange_polys = gen_lagrange_polynomials_parallel(&x);


        assert_eq!(lagrange_polys.len(), x.len());

        // Check that each L_i(x_j) == 1 if i == j, else 0
        for (i, &xi) in x.iter().enumerate() {
            for (j, &xj) in x.iter().enumerate() {
                let eval = lagrange_polys[i].evaluate(xj);
                if i == j {
                    assert_eq!(
                        eval,
                        FieldElement::one(),
                        "Basis polynomial L_{} did not evaluate to 1 at x = {:?}",
                        i,
                        xj
                    );
                } else {
                    assert_eq!(
                        eval,
                        FieldElement::zero(),
                        "Basis polynomial L_{} did not evaluate to 0 at x = {:?}",
                        i,
                        xj
                    );
                }
            }
        }
    }

    #[test]
    fn test_gen_lagrange_poly_parallel_random() {

        let x = vec![
            fe!(7, 1),
            fe!(7, 2),
            fe!(7, 3),
            fe!(7, 4),
            fe!(7, 6),
        ];

        let lagrange_polys = gen_lagrange_polynomials_parallel(&x);
        assert_eq!(lagrange_polys.len(), x.len());

        // Same correctness check: L_i(x_j) == δ_{ij}
        for (i, &xi) in x.iter().enumerate() {
            for (j, &xj) in x.iter().enumerate() {
                let eval = lagrange_polys[i].evaluate(xj);
                if i == j {
                    assert_eq!(eval, FieldElement::one());
                } else {
                    assert_eq!(eval, FieldElement::zero());
                }
            }
        }
    }

}

