use crate::fields::FieldElement;
use crate::polynomial::Polynomial;
use crate::{poly,fe,field};

/// - Each layer's evaluations
/// - Each layer's Merkle tree
/// - The final polynomial 
#[derive(Clone)]
pub struct FRIProof {
    pub fri_layers: Vec<Vec<FieldElement>>, 
    pub fri_merkles: Vec<MerkleTree>,
    pub final_poly: Polynomial, // The final constant or low-degree poly
}

///  produce the next domain by squaring.
/// For a domain [d_0, d_1, ..., d_{n-1}], the “folded” domain
/// is [d_0^2, d_1^2, ..., d_{(n/2)-1}^2].
fn next_fri_domain(domain: &[FieldElement]) -> Vec<FieldElement> {
    let half = domain.len() / 2;
    domain[..half]
        .iter()
        .map(|&x| x.pow(2u64))
        .collect()
}


/// If your FRI definition is the standard
/// \[p_{i+1}(x) = \frac{p_i(x) + p_i(-x)}{2} + \beta * \frac{p_i(x) - p_i(-x)}{2x}\],
///
///     next_poly(x) = even_part(x) + beta * odd_part(x)

fn next_fri_polynomial(poly: &Polynomial, beta: FieldElement) -> Polynomial {
    let odd_coeffs: Vec<FieldElement> = poly
        .coefficients
        .iter()
        .skip(1)
        .step_by(2)
        .copied()
        .collect(); // a1, a3, a5,...
    let even_coeffs: Vec<FieldElement> = poly
        .coefficients
        .iter()
        .step_by(2)
        .copied()
        .collect(); // a0, a2, a4,...

    let odd_poly = Polynomial::new(odd_coeffs) * beta;
    let even_poly = Polynomial::new(even_coeffs);
    odd_poly + even_poly
}

/// Single FRI “fold” step: produce next polynomial, next domain, and next layer of evaluations.
fn next_fri_layer(
    current_poly: &Polynomial,
    current_domain: &[FieldElement],
    beta: FieldElement,
) -> (Polynomial, Vec<FieldElement>, Vec<FieldElement>) {
    let folded_poly = next_fri_polynomial(current_poly, beta);
    let folded_domain = next_fri_domain(current_domain);
    let folded_evals = folded_domain
        .iter()
        .map(|&x| folded_poly.evaluate(x))
        .collect::<Vec<_>>();
    (folded_poly, folded_domain, folded_evals)
}

/// The main “FRI commit” phase:
/// 1. Evaluate  polynomial on the domain, build Merkle tree, send root.
/// 2. Repeatedly fold with random betas
/// 3. Send the final constant (or low-degree polynomial) to the verifier.
/// 4. Return all data as `FRIProof`.
pub fn fri_commit(
    mut poly: Polynomial,
    mut domain: Vec<FieldElement>,
    channel: &mut Channel,
) -> FRIProof {

    let mut evals = domain.iter().map(|&x| poly.evaluate(x)).collect::<Vec<_>>();
    let mut merkle = MerkleTree::new(&evals);

    //store each layer's evals + Merkle tree
    let mut fri_layers = vec![evals];
    let mut fri_merkles = vec![merkle];

    // Send the root of the first layer
    channel.send(fri_merkles[0].root().to_vec());

    // While the polynomial is still more than degree 0...
    while poly.degree >= 1 {
        // Get random beta from the verifier
        let beta = channel.receive_random_field_element();

        // Fold polynomial + domain
        let (new_poly, new_domain, new_evals) = next_fri_layer(&poly, &domain, beta);

        // Build next Merkle
        let new_merkle = MerkleTree::new(&new_evals);

        // Send the new Merkle root
        channel.send(new_merkle.root().to_vec());

        fri_layers.push(new_evals);
        fri_merkles.push(new_merkle);
        poly = new_poly;
        domain = new_domain;
    }

    // Send that constant final to the verifier
    let final_value = if poly.is_zero() {
        FieldElement::zero()
    } else {
        poly.coefficients[0]
    };
    channel.send(final_value.to_bytes());

    // Return the entire FRI proof (all layers + trees + final poly)
    FRIProof {
        fri_layers,
        fri_merkles,
        final_poly: poly,
    }
}

/* ============================================
   Decommitment of FRI Queries
   ============================================
   After the verifier picks random indices in
   [0..(size of first layer)], we provide for each:

   1) The value fri_layers[i][index] and its Merkle proof
   2) The "sibling" value fri_layers[i][index + length/2] and its proof
   3) This repeats for each FRI layer i (because the domain halves each time).
   4) On the last layer, there's typically only 1 value left (the final constant).
*/

/// Decommit all FRI layers for a single query index.
pub fn decommit_fri_layers(
    index: usize,
    fri_layers: &[Vec<FieldElement>],
    fri_merkles: &[MerkleTree],
    channel: &mut Channel,
) {

    for (layer_evals, merkle_tree) in fri_layers.iter().zip(fri_merkles) {
        let length = layer_evals.len();
        // If length == 1, it’s the final constant—just send that or skip it
        if length == 1 {
            channel.send(layer_evals[0].to_bytes());
        }

        // The actual index in this layer:
        let idx = index % length;
        let sibling_idx = (idx + length / 2) % length;

        // Send the element
        channel.send(layer_evals[idx].to_bytes());
        let path = merkle_tree.get_authentication_path(idx);
        channel.send(path);

        // Send the sibling
        channel.send(layer_evals[sibling_idx].to_bytes());
        let sibling_path = merkle_tree.get_authentication_path(sibling_idx);
        channel.send(sibling_path);
    }
}


pub fn decommit_fri(
    num_queries: usize,
    max_index: usize,
    fri_layers: &[Vec<FieldElement>],
    fri_merkles: &[MerkleTree],
    channel: &mut Channel,
) {
    for _ in 0..num_queries {
        let idx = channel.receive_random_int(0, max_index, /* show_in_proof= */ true);
        decommit_fri_layers(idx, fri_layers, fri_merkles, channel);
    }
}

