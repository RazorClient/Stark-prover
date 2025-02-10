use crate::{
    fields::FieldElement,
    merkle::MerkleTree,
    channel::Channel,
};

/// Verifies the entire FRI proof generated by prove_fri.
pub fn verify_fri<const M: u64>(
    /* proof data, domain info, channel... */
) -> bool {
    // Reconstruct each round's merkle root,
    // check colinearity,
    // check final polynomial is degree < ...
    unimplemented!()
}
