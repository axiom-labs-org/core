use axiom_ext_tx::ExternalTransaction;
use axiom_types::{Slot, Epoch, Hash};

use crate::hash::block_hash;

/// A block is an ordered batch of external transactions.
///
/// A block is an execution and consensus object.
#[derive(Debug, Clone)]
pub struct Block {
    /// Hash of the parent block (None for genesis)
    pub parent_hash: Option<Hash>,

    /// Slot in which this block is executed
    pub slot: Slot,

    /// Epoch context for protocol transitions
    pub epoch: Epoch,

    /// State root AFTER executing this block
    pub state_root: Hash,

    /// Receipts root committing tx results
    pub receipts_root: Hash,

    /// Ordered list of transactions
    pub transactions: Vec<ExternalTransaction>,
}

impl Block {
    /// Compute the canonical hash of this block.
    pub fn hash(&self) -> Hash {
        block_hash(self)
    }
}
