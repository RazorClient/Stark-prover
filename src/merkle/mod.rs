use crate::fields::FieldElement;
use rs_merkle::algorithms::Sha256;
use rs_merkle::{self, Hasher};

pub struct MerkleTree<const MODULUS: u64> {
    inner: rs_merkle::MerkleTree<rs_merkle::algorithms::Sha256>,
}

impl <const MODULUS: u64> MerkleTree<MODULUS> {
    pub fn new(data: Vec<FieldElement<MODULUS>>) -> Self {
        let hashed_data: Vec<[u8; 32]> = data
        .into_iter()
        .map(|d| {
            let bytes = d.value().to_be_bytes(); // big-endian
            Sha256::hash(&bytes)
        })
        .collect();
        let inner =
            rs_merkle::MerkleTree::<rs_merkle::algorithms::Sha256>::from_leaves(&hashed_data);

        MerkleTree { inner }
    }

    pub fn root(&self) -> String {
        self.inner.root_hex().unwrap()
    }
}

