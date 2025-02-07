// how to design the channel ?
// use merlin ??


use crate::fields::FieldElement; 
use alloy::{hex::{self}, primitives::U256};
use sha256;


/// - `proof`: stores all messages (Prover + “random challenges”).
/// - `compressed_proof`: stores a subset
/// - `state`: a rolling state  to generate pseudo-randomness.
#[derive(Debug, Clone)]
pub struct Channel<const MODULUS: u64> {
    /// All messages in raw bytes
    pub proof: Vec<Vec<u8>>,
    pub compressed_proof: Vec<Vec<u8>>,
    /// Current "randomness" state, stored as hex for naive hashing.
    pub state: String,
}

impl<const MODULUS: u64> Channel<MODULUS> {

    pub fn new() -> Self {
        Self {
            proof: Vec::new(),
            compressed_proof: Vec::new(),
            state: String::new()
        }
    }

    /// Simulates the Prover sending a message (raw bytes) into the channel.
    ///  1) We update `state` by hashing (old_state + hex-encoded bytes).
    ///  2) We store the raw bytes in both `proof` and `compressed_proof`.
    pub fn send(&mut self, message: &[u8]) {
        let old_state = self.state.clone();
        // Concatenate old_state + hex(message) 
        let concatenated = old_state + &hex::encode(message);
        self.state = sha256::digest(concatenated);

        // Record the raw bytes
        self.proof.push(message.to_vec());
        self.compressed_proof.push(message.to_vec());
    }


    pub fn receive_random_field_element(&mut self) -> FieldElement<MODULUS> {
        let num = self.receive_random_int(0, (MODULUS - 1) as usize, false);
        let field_elem = FieldElement::new(num as u64);

        //store the numeric value in the full proof (as bytes).
        self.proof.push(num.to_be_bytes().to_vec());

        field_elem
    }

    /// Emulates receiving a random integer in [min..max].
    pub fn receive_random_int(&mut self, min: usize, max: usize, show_in_proof: bool) -> usize {

        // Note that when the range is close to 2^256 this does not emit a uniform distribution,
        // even if sha256 is uniformly distributed.

        // Convert current state (hex) to U256.
        let state_u256 = U256::from_str_radix(&self.state, 16)
            .expect("Channel state is not valid hex");
    
        // Calculate range and convert it to U256.
        let range = (max - min) + 1;
        let range_u256 = U256::from(range);
    
        // Compute the pseudo-random integer in U256.
        let num = (state_u256 + U256::from(min)) % range_u256;
    
        // Update the channel's state with another hash.
        let old_state = self.state.clone();
        self.state = sha256::digest(old_state);

        if show_in_proof {
            self.proof.push((num.into_limbs()[0] as usize).to_be_bytes().to_vec());
        }
    
        // Convert the result from U256 by taking the first limb and casting it to usize.
        num.into_limbs()[0] as usize
    }
    

    /// Total size of all messages in `proof`.
    pub fn proof_size(&self) -> usize {
        self.proof.iter().map(|bytes| bytes.len()).sum()
    }

    /// Total size of all messages in `compressed_proof`.
    pub fn compressed_proof_size(&self) -> usize {
        self.compressed_proof.iter().map(|bytes| bytes.len()).sum()
    }
}
