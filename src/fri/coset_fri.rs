//! in the FRI protocol, shifting a subgroup by an offset `g`.

use crate::fields::element::FieldElement;

/// Defines a Coset-FRI configuration for domain generation:
///    D = { offset * omega^i | i = 0..domain_size-1 }
/// where omega has order = domain_size (typically 2^k).
#[derive(Clone, Debug)]
pub struct CosetFri<const M: u64> {
    /// The coset offset `g`, not in <omega>.
    pub offset: FieldElement<M>,

    /// The subgroup generator of order `domain_size`.
    pub omega: FieldElement<M>,

    /// The size of the initial domain, e.g. 2^k.
    pub domain_size: usize,
}

impl<const M: u64> CosetFri<M> {
    /// Creates a new CosetFri instance.
    pub fn new(offset: FieldElement<M>, omega: FieldElement<M>, domain_size: usize) -> Self {
        Self {
            offset,
            omega,
            domain_size,
        }
    }

    /// Generates the initial coset domain:
    ///      D = { offset * (omega^i) : i in [0..domain_size) }
    pub fn generate_coset_domain(&self) -> Vec<FieldElement<M>> {
        (0..self.domain_size)
            .map(|i| self.offset * self.omega.pow(i as u64))
            .collect()
    }

    /// Squares each element of the current domain to build the next domain
    /// of half the length. The typical "FRI folding" step will use half of the domain.
    pub fn next_coset_domain(
        &self,
        current_domain: &[FieldElement<M>]
    ) -> Vec<FieldElement<M>> {
        // Note: some FRI implementations only take the first half of current_domain
        // for the next round. This is a design choice. Here we show the "square all" step.
        current_domain
            .iter()
            .map(|d| d.square())
            .collect()
    }
}

