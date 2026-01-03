use std::collections::{HashMap, BTreeMap};

use axiom_block::Block;
use axiom_ext_tx::{ExternalTransaction, Signature};
use axiom_tx::{TransactionCell, CallData};
use axiom_types::{Address, Slot, Epoch, ObjectId};
use axiom_types::Hash;

// -------------------------------------------------------------
// Helper: create a minimal valid TransactionCell
// -------------------------------------------------------------
fn make_cell(target: ObjectId) -> TransactionCell {
    TransactionCell::new(
        Slot::new(1),
        HashMap::new(),
        BTreeMap::new(),
        CallData {
            target,
            selector: vec![],
            payload: vec![],
        },
    ).unwrap()
}

// -------------------------------------------------------------
// Helper: create a minimal ExternalTransaction
// -------------------------------------------------------------
fn make_tx(signer: Address, nonce: u64, target: ObjectId, sig_byte: u8) -> ExternalTransaction {
    ExternalTransaction {
        signer,
        nonce,
        cells: vec![make_cell(target)],
        signature: Signature {
            bytes: vec![sig_byte; 64],
        },
    }
}

// -------------------------------------------------------------
// Test: Block hash is deterministic
// -------------------------------------------------------------
#[test]
fn block_hash_is_deterministic() {
    let signer = Address::new([1u8; 32]);
    let target = ObjectId::new(axiom_types::Hash::new([9u8; 32]));

    let txs = vec![
        make_tx(signer, 0, target, 1),
        make_tx(signer, 0, target, 2),
    ];

    let block1 = Block {
        slot: Slot::new(10),
        epoch: Epoch::new(1),
        transactions: txs.clone(),
        parent_hash: None,
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
    };

    let block2 = Block {
        slot: Slot::new(10),
        epoch: Epoch::new(1),
        transactions: txs,
        parent_hash: None,
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
    };

    let h1 = block1.hash();
    let h2 = block2.hash();

    assert_eq!(h1, h2, "same block data must hash identically");
}

// -------------------------------------------------------------
// Test: Block hash changes if transaction order changes
// -------------------------------------------------------------
#[test]
fn block_hash_changes_with_tx_order() {
    let signer = Address::new([2u8; 32]);
    let target = ObjectId::new(axiom_types::Hash::new([8u8; 32]));

    let tx1 = make_tx(signer, 0, target, 1);
    let tx2 = make_tx(signer, 0, target, 2);

    let block_a = Block {
        slot: Slot::new(5),
        epoch: Epoch::new(0),
        transactions: vec![tx1.clone(), tx2.clone()],
        parent_hash: None,
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
    };

    let block_b = Block {
        slot: Slot::new(5),
        epoch: Epoch::new(0),
        transactions: vec![tx2, tx1],
        parent_hash: None,
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
    };

    assert_ne!(
        block_a.hash(),
        block_b.hash(),
        "transaction order must affect block hash"
    );
}

// -------------------------------------------------------------
// Test: Block hash changes if signature changes
// -------------------------------------------------------------
#[test]
fn block_hash_changes_with_signature() {
    let signer = Address::new([3u8; 32]);
    let target = ObjectId::new(axiom_types::Hash::new([7u8; 32]));

    let tx_a = make_tx(signer, 0, target, 1);
    let tx_b = make_tx(signer, 0, target, 9); // different signature bytes

    let block_a = Block {
        slot: Slot::new(7),
        epoch: Epoch::new(2),
        transactions: vec![tx_a],
        parent_hash: None,
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
    };

    let block_b = Block {
        slot: Slot::new(7),
        epoch: Epoch::new(2),
        transactions: vec![tx_b],
        parent_hash: None,
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
    };

    assert_ne!(
        block_a.hash(),
        block_b.hash(),
        "signature must be committed to block hash"
    );
}

// -------------------------------------------------------------
// Test: Block hash changes if slot or epoch changes
// -------------------------------------------------------------
#[test]
fn block_hash_changes_with_context() {
    let signer = Address::new([4u8; 32]);
    let target = ObjectId::new(axiom_types::Hash::new([6u8; 32]));

    let tx = make_tx(signer, 0, target, 1);

    let block_slot_a = Block {
        slot: Slot::new(1),
        epoch: Epoch::new(0),
        transactions: vec![tx.clone()],
        parent_hash: None,
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
    };

    let block_slot_b = Block {
        slot: Slot::new(2),
        epoch: Epoch::new(0),
        transactions: vec![tx.clone()],
        parent_hash: None,
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
    };

    let block_epoch_c = Block {
        slot: Slot::new(1),
        epoch: Epoch::new(1),
        transactions: vec![tx],
        parent_hash: None,
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
    };

    assert_ne!(block_slot_a.hash(), block_slot_b.hash());
    assert_ne!(block_slot_a.hash(), block_epoch_c.hash());
}

// -------------------------------------------------------------
// Test: Canonical encoding is stable across calls
// -------------------------------------------------------------
#[test]
fn canonical_encoding_is_stable() {
    let signer = Address::new([5u8; 32]);
    let target = ObjectId::new(axiom_types::Hash::new([5u8; 32]));

    let tx = make_tx(signer, 0, target, 1);

    let block = Block {
        slot: Slot::new(42),
        epoch: Epoch::new(9),
        transactions: vec![tx],
        parent_hash: None,
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
    };

    let h1 = block.hash();
    let h2 = block.hash();

    assert_eq!(h1, h2, "repeated hashing must be stable");
}

// -------------------------------------------------------------
// Test: Block hash changes when state root changes
// -------------------------------------------------------------
#[test]
fn block_hash_changes_with_state_root() {
    let signer = Address::new([9u8; 32]);
    let target = ObjectId::new(Hash::new([1u8; 32]));

    let tx = make_tx(signer, 0, target, 1);

    let block_a = Block {
        parent_hash: None,
        slot: Slot::new(1),
        epoch: Epoch::new(0),
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
        transactions: vec![tx.clone()],
    };

    let block_b = Block {
        parent_hash: None,
        slot: Slot::new(1),
        epoch: Epoch::new(0),
        state_root: Hash::new([7u8; 32]),
        receipts_root: Hash::zero(),
        transactions: vec![tx],
    };

    assert_ne!(
        block_a.hash(),
        block_b.hash(),
        "changing state root must change block hash"
    );
}

// -------------------------------------------------------------
// Test: Block hash changes when receipts root changes
// -------------------------------------------------------------
#[test]
fn block_hash_changes_with_receipts_root() {
    let signer = Address::new([8u8; 32]);
    let target = ObjectId::new(Hash::new([4u8; 32]));

    let tx = make_tx(signer, 0, target, 1);

    let block_a = Block {
        parent_hash: None,
        slot: Slot::new(1),
        epoch: Epoch::new(0),
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
        transactions: vec![tx.clone()],
    };

    let block_b = Block {
        parent_hash: None,
        slot: Slot::new(1),
        epoch: Epoch::new(0),
        state_root: Hash::zero(),
        receipts_root: Hash::new([9u8; 32]),
        transactions: vec![tx],
    };

    assert_ne!(
        block_a.hash(),
        block_b.hash(),
        "changing receipts root must change block hash"
    );
}
