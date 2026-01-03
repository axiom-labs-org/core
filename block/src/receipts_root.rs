use axiom_types::Hash;
use blake3;

use crate::TransactionResult;

/// Compute the canonical receipts root for a block.
///
/// Receipts are ordered exactly as transactions appear in the block.
pub fn compute_receipts_root(
    tx_hashes: &[Hash],
    results: &[TransactionResult],
) -> Hash {
    assert_eq!(
        tx_hashes.len(),
        results.len(),
        "tx hashes and results length mismatch"
    );

    let mut bytes = Vec::new();

    // Domain separation
    bytes.extend_from_slice(b"Axiom::ReceiptsRoot::v1");

    for (tx_hash, result) in tx_hashes.iter().zip(results.iter()) {
        // Transaction hash
        bytes.extend_from_slice(tx_hash.as_bytes());

        match result {
            TransactionResult::Success { fee_charged } => {
                bytes.push(1); // success flag
                bytes.extend_from_slice(&fee_charged.to_be_bytes());
            }
            TransactionResult::Failure { .. } => {
                bytes.push(0); // failure flag
                bytes.extend_from_slice(&0u64.to_be_bytes()); // no fee
            }
        }
    }

    Hash::new(blake3::hash(&bytes).into())
}
