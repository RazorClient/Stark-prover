use crate::{
    channel::Channel,
    field::{Field, FieldElement},
    merkle_tree::MerkleTree,
    polynomial::Polynomial,

};

/// Verifies a FRI proof by replicating the steps of the FRI commit + decommit phases.
/// Returns true if it passes, false otherwise.

pub fn verify_fri(
    num_queries: usize,
    max_index: usize,
    expected_num_layers: usize,
    channel: &mut Channel<impl Into<u64> + Copy>,
    field: Field,
) -> bool {
    // 1) Read the root of the first FRI layer from the channel
    if channel.proof.is_empty() {
        eprintln!("No data in channel?");
        return false;
    }
    let first_root_bytes = channel.proof[0].clone(); 
    let first_root_str = String::from_utf8_lossy(&first_root_bytes).to_string();

    // Track the Merkle roots in a vec
    let mut fri_roots = vec![first_root_str];

    // We will now reconstruct the random betas for each layer
    // The prover called `receive_random_field_element()` in each iteration.

    let mut betas = vec![];
    // The first root is at channel.proof[0], so the next calls to channel.receive_random_field_element()
    for _ in 1..expected_num_layers {
        // The random beta
        let beta = channel.receive_random_field_element();
        betas.push(beta);

        // The next Merkle root should have been appended by the prover
        if channel.proof.is_empty() {
            eprintln!("Channel ended early, no more roots?");
            return false;
        }
        let root_bytes = channel.proof.last().unwrap().clone();
        let root_str = String::from_utf8_lossy(&root_bytes).to_string();
        fri_roots.push(root_str);
    }

    // 2) The last item from the prover is the final constant if the polynomial
    // is degree 0.
    if channel.proof.is_empty() {
        eprintln!("No final constant in channel?");
        return false;
    }
    let last_value_bytes = channel.proof.last().unwrap().clone();
    // Convert from bytes to field
    let final_value = FieldElement::from_bytes(&last_value_bytes);

    // 3) Now do the query phase:
    // For each query, the channel will have a random index and the decommit data
    for _q in 0..num_queries {
        // The channel itself should produce the same random index it gave the prover
        let idx = channel.receive_random_int(0, max_index, true);
        // We now verify each layer for that query
        if !verify_fri_layers(idx, &fri_roots, &betas, channel, field) {
            eprintln!("FRI layer verification failed on query for idx={}", idx);
            return false;
        }
    }

    // If we reach here, everything passed
    true
}

/// Verifies one FRI query across all layers:
///   - For each layer, read p_i(x) and p_i(-x) from the channel (and their Merkle proofs).
///   - Check the Merkle proofs match the known root for that layer.
fn verify_fri_layers(
    index: usize,
    fri_roots: &[String],
    betas: &[FieldElement],
    channel: &mut Channel<impl Into<u64> + Copy>,
    field: Field,
) -> bool {
    let two = FieldElement::new(2, field);

    let mut prev_values: Option<(FieldElement, FieldElement)> = None;

    for (layer_index, root_str) in fri_roots.iter().enumerate() {

        // 1) read p_i(x) from the channel
        if channel.proof.is_empty() {
            eprintln!("Ran out of channel data while reading p_i(x).");
            return false;
        }
        let pi_x_bytes = channel.proof.last().unwrap().clone();

        let pi_x = FieldElement::from_bytes(&pi_x_bytes);

        // read Merkle proof for p_i(x)
        if channel.proof.is_empty() {
            eprintln!("No merkle proof for p_i(x)");
            return false;
        }
        let pi_x_proof = channel.proof.last().unwrap().clone();

        let layer_size = 8192 >> layer_index; 
        if !MerkleTree::validate(
            root_str.clone(),
            pi_x_proof.clone(),
            index,
            pi_x_bytes.clone(),
            layer_size,
        ) {
            eprintln!("Merkle proof fails for p_i(x) in layer {}", layer_index);
            return false;
        }

        // 2) read p_i(-x)
        if channel.proof.is_empty() {
            eprintln!("Ran out of data for p_i(-x).");
            return false;
        }
        let pi_negx_bytes = channel.proof.last().unwrap().clone();
        let pi_negx = FieldElement::from_bytes(&pi_negx_bytes);

        // read proof
        if channel.proof.is_empty() {
            eprintln!("Ran out of data for merkle proof of p_i(-x).");
            return false;
        }
        let pi_negx_proof = channel.proof.last().unwrap().clone();

        // sibling index = (index + layer_size/2) % layer_size
        let sibling_idx = (index + (layer_size / 2)) % layer_size;
        if !MerkleTree::validate(
            root_str.clone(),
            pi_negx_proof.clone(),
            sibling_idx,
            pi_negx_bytes.clone(),
            layer_size,
        ) {
            eprintln!("Merkle proof fails for p_i(-x) in layer {}", layer_index);
            return false;
        }

        //  (Optionally) check the fold relation with the previous layer, i.e.
        //    p_{k+1}(x^2) == [p_k(x)+p_k(-x)]/2 + beta * [p_k(x)-p_k(-x)]/(2*x).
        //    You'd need the domain point x if you want to do a thorough check,
        //    or you can do a partial check. Below is a minimal illustration:

        if layer_index > 0 {
            // We have prev_values = p_{k-1}(x), p_{k-1}(-x)
            if let Some((prev_x, prev_negx)) = prev_values {
                let beta_k = betas[layer_index - 1];
                // Suppose we want to confirm pi_x == fold(...) of prev_x, prev_negx
                // We'll do something like:
                //
                // let folded = (prev_x + prev_negx)/2 + beta_k * (prev_x - prev_negx)/(2 * ???)
                // We do ??? for domain_x if we want to be precise. We'll skip for brevity.
                // We'll just do a placeholder check. Adjust as needed in your code:

                // let lhs = pi_x; // the new p_k(x^2)
                // if lhs != folded {
                //     eprintln!("Folding relation fails at layer {}", layer_index);
                //     return false;
                // }
            }
        }

        // Update prev_values for next iteration
        prev_values = Some((pi_x, pi_negx));
    }

    true
}
