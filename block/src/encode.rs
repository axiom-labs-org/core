
use axiom_ext_tx::ExternalTransaction;
use axiom_types::{Slot, Epoch, Hash};

/// Append a u64 as big-endian bytes.
fn push_u64(buf: &mut Vec<u8>, value: u64) {
    buf.extend_from_slice(&value.to_be_bytes());
}

/// Append a u32 as big-endian bytes.
fn push_u32(buf: &mut Vec<u8>, value: u32) {
    buf.extend_from_slice(&value.to_be_bytes());
}

/// Canonically encode a block.
pub fn encode_block(
    parent_hash: Option<Hash>,
    slot: Slot,
    epoch: Epoch,
    state_root: Hash,
    receipts_root: Hash,
    transactions: &[ExternalTransaction],
) -> Vec<u8> {
    let mut buf = Vec::new();

    // -------------------------------------------------------------
    // Domain separator (block)
    // -------------------------------------------------------------
    buf.extend_from_slice(b"Axiom::Block::v1");

    // -------------------------------------------------------------
    // Parent hash
    // -------------------------------------------------------------
    match parent_hash {
        Some(hash) => {
            buf.push(1); // presence flag
            buf.extend_from_slice(hash.as_bytes());
        }
        None => {
            buf.push(0); // genesis
        }
    }

    // -------------------------------------------------------------
    // Slot
    // -------------------------------------------------------------
    push_u64(&mut buf, slot.value());

    // -------------------------------------------------------------
    // Epoch
    // -------------------------------------------------------------
    push_u64(&mut buf, epoch.value());

    // -------------------------------------------------------------
    // State root
    // -------------------------------------------------------------
    buf.extend_from_slice(state_root.as_bytes());

    // -------------------------------------------------------------
    // Receipts root
    // -------------------------------------------------------------
    buf.extend_from_slice(receipts_root.as_bytes());

    // -------------------------------------------------------------
    // Transactions
    // -------------------------------------------------------------
    push_u32(&mut buf, transactions.len() as u32);

    for tx in transactions {
        encode_external_transaction(&mut buf, tx);
    }

    buf
}

/// Canonically encode an external transaction (for block hashing).
fn encode_external_transaction(buf: &mut Vec<u8>, tx: &ExternalTransaction) {
    // -------------------------------------------------------------
    // Domain separator (external tx)
    // -------------------------------------------------------------
    buf.extend_from_slice(b"Axiom::ExternalTx::v1");

    // -------------------------------------------------------------
    // Signer
    // -------------------------------------------------------------
    buf.extend_from_slice(tx.signer.as_bytes());

    // -------------------------------------------------------------
    // Nonce
    // -------------------------------------------------------------
    push_u64(buf, tx.nonce);

    // -------------------------------------------------------------
    // Cells (ordered by cell id)
    // -------------------------------------------------------------
    let mut cell_ids: Vec<_> = tx.cells.iter().map(|c| c.id()).collect();
    cell_ids.sort();

    push_u32(buf, cell_ids.len() as u32);

    for id in cell_ids {
        buf.extend_from_slice(id.as_bytes());
    }

    // -------------------------------------------------------------
    // Signature
    // -------------------------------------------------------------
    push_u32(buf, tx.signature.bytes.len() as u32);
    buf.extend_from_slice(&tx.signature.bytes);
}
