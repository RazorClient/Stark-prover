use crate::field_element::FieldElement;
use crate::polynomial::Polynomial;
// add ntt and pararell version latter

pub fn gen_polynomial_from_roots<const M: u64>(roots: &[FieldElement<M>]) -> Polynomial<M> {
    if roots.is_empty() {
        return Polynomial::new(vec![]); 
    }

    // Start with the constant polynomial "1"
    let mut p = Polynomial::new(vec![FieldElement::one()]);

    // Multiply p by (x - root) for each root
    for &root in roots {
        // We'll build a new polynomial with one higher degree
        let mut new_coeffs = vec![FieldElement::zero(); p.coefficients.len() + 1];
        let neg_root = -root;

        // For each term a_i x^i in p(x), multiply by (x - root)
        for (i, &coeff) in p.coefficients.iter().enumerate() {
            new_coeffs[i] += coeff * neg_root;
            new_coeffs[i + 1] += coeff;
        }
        p = Polynomial::new(new_coeffs);
    }

    p
}

/// Return the Lagrange basis polynomials [L0, L1, ..., L_{n-1}],
/// where L_i(x) = ∏_{j != i} (x - x_j) / (x_i - x_j).
///
/// 
///    1) Let Z(x) = ∏ (x - x_j).
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
        assert!(rem.is_zero(), "Z(x) should be divisible by (x - x_i)");
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
    let n = xs.len();
    assert_eq!(n, ys.len(), "Mismatched x and y lengths");
    if n == 0 {
        // Return zero polynomial if no points
        return Polynomial::zero();
    }
    // Compute the basis polynomials L_i(x).
    let L = gen_lagrange_polynomials(xs);

    // Sum up: f(x) = Σ (ys[i] * L[i](x)).
    let mut acc = Polynomial::zero();
    for i in 0..n {
        // clone L[i], multiply by ys[i].
        let mut term = L[i].clone();
        term.scalar_mul(ys[i]);
        // add to accumulator
        acc.add_assign(&term);
    }
    acc
}
