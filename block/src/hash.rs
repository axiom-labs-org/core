use axiom_types::Hash;
use blake3;

use crate::encode::encode_block;
use crate::Block;

/// Compute the canonical hash of a block.
pub fn block_hash(block: &Block) -> Hash {
    let bytes = encode_block(
        block.parent_hash,
        block.slot,
        block.epoch,
        block.state_root,
        block.receipts_root,
        &block.transactions,
    );

    Hash::new(blake3::hash(&bytes).into())
}
