use std::collections::{HashMap, BTreeMap};

use axiom_block::{Block, execute_block, TransactionResult};
use axiom_execution_engine::{ReferenceExecutionEngine};
// use axiom_protocol::process_external_transaction;
use axiom_state::{StateStore, StateObject};
use axiom_state::nonce::nonce_object_id;
use axiom_state::balance::{balance_object_id, encode_balance, decode_balance};
use axiom_ext_tx::{ExternalTransaction, Signature};
use axiom_tx::{TransactionCell, CallData};
use axiom_types::{Address, Slot, Epoch, ObjectId};
use axiom_state::compute_state_root;
use axiom_types::Hash;
use axiom_block::compute_receipts_root;


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
// Test: Block executes all valid transactions
// -------------------------------------------------------------
#[test]
fn block_executes_all_valid_transactions() {
    let mut state = StateStore::new();
    let signer = Address::new([1u8; 32]);

    let balance_id = balance_object_id(signer);
    state.insert(StateObject::new(balance_id, signer, encode_balance(10))).unwrap();

    let txs = vec![
        ExternalTransaction {
            signer,
            nonce: 0,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        ExternalTransaction {
            signer,
            nonce: 0,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
    ];

    let mut block = Block {
        parent_hash: None,
        slot: Slot::new(1),
        epoch: Epoch::new(0),
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
        transactions: txs,
    };

    let engine = ReferenceExecutionEngine::default();
    let result = execute_block(&mut state, &mut block, &engine);

    let tx_hashes: Vec<_> = block
        .transactions
        .iter()
        .map(|tx| tx.signing_hash())
        .collect();

    let expected_receipts_root =
        compute_receipts_root(&tx_hashes, &result.tx_results);

    assert_eq!(block.receipts_root, expected_receipts_root);

    assert_eq!(result.tx_results.len(), 2);
    assert!(matches!(result.tx_results[0], TransactionResult::Success { .. }));
    assert!(matches!(result.tx_results[1], TransactionResult::Success { .. }));

    assert_eq!(decode_balance(state.get(&balance_id).unwrap()), 8);
    assert_eq!(state.get(&nonce_object_id(signer)).unwrap().version(), 1);

    // ✅ State root correctness
    let expected_root = compute_state_root(&state);
    assert_eq!(block.state_root, expected_root);
}

// -------------------------------------------------------------
// Test: Failed transaction does not affect later transactions
// -------------------------------------------------------------
#[test]
fn block_failure_does_not_affect_later_txs() {
    let mut state = StateStore::new();
    let signer = Address::new([2u8; 32]);

    let balance_id = balance_object_id(signer);
    state.insert(StateObject::new(balance_id, signer, encode_balance(10))).unwrap();

    let txs = vec![
        // tx #1 valid
        ExternalTransaction {
            signer,
            nonce: 0,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        // tx #2 invalid (bad nonce)
        ExternalTransaction {
            signer,
            nonce: 5, // ❌ invalid
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        // tx #3 valid again
        ExternalTransaction {
            signer,
            nonce: 0,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
    ];

    let mut block = Block {
        parent_hash: None,
        slot: Slot::new(1),
        epoch: Epoch::new(0),
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
        transactions: txs,
    };

    let engine = ReferenceExecutionEngine::default();
    let result = execute_block(&mut state, &mut block, &engine);

    let tx_hashes: Vec<_> = block
        .transactions
        .iter()
        .map(|tx| tx.signing_hash())
        .collect();

    let expected_receipts_root =
        compute_receipts_root(&tx_hashes, &result.tx_results);

    assert_eq!(block.receipts_root, expected_receipts_root);


    assert!(matches!(result.tx_results[0], TransactionResult::Success { .. }));
    assert!(matches!(result.tx_results[1], TransactionResult::Failure { .. }));
    assert!(matches!(result.tx_results[2], TransactionResult::Success { .. }));

    // Only two successful txs should charge fees
    assert_eq!(decode_balance(state.get(&balance_id).unwrap()), 8);
    
    // ✅ State root correctness
    let expected_root = compute_state_root(&state);
    assert_eq!(block.state_root, expected_root);
}

// -------------------------------------------------------------
// Test: Block with all invalid transactions causes no state change
// -------------------------------------------------------------
#[test]
fn block_all_invalid_txs_no_state_change() {
    let mut state = StateStore::new();
    let signer = Address::new([3u8; 32]);

    let balance_id = balance_object_id(signer);
    state.insert(StateObject::new(balance_id, signer, encode_balance(5))).unwrap();

    let txs = vec![
        ExternalTransaction {
            signer,
            nonce: 1,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        ExternalTransaction {
            signer,
            nonce: 2,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
    ];

    let mut block = Block {
        parent_hash: None,
        slot: Slot::new(1),
        epoch: Epoch::new(0),
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
        transactions: txs,
    };

    let engine = ReferenceExecutionEngine::default();
    let result = execute_block(&mut state, &mut block, &engine);

    let tx_hashes: Vec<_> = block
        .transactions
        .iter()
        .map(|tx| tx.signing_hash())
        .collect();

    let expected_receipts_root =
        compute_receipts_root(&tx_hashes, &result.tx_results);

    assert_eq!(block.receipts_root, expected_receipts_root);


    assert!(result.tx_results.iter().all(|r| matches!(r, TransactionResult::Failure { .. })));

    // State unchanged
    assert_eq!(decode_balance(state.get(&balance_id).unwrap()), 5);
    assert!(state.get(&nonce_object_id(signer)).is_none());

    // ✅ State root correctness
    let expected_root = compute_state_root(&state);
    assert_eq!(block.state_root, expected_root);
}

// -------------------------------------------------------------
// Test: Block execution is deterministic
// -------------------------------------------------------------
#[test]
fn block_execution_is_deterministic() {
    let signer = Address::new([4u8; 32]);

    let mut state1 = StateStore::new();
    let mut state2 = StateStore::new();

    let balance_id = balance_object_id(signer);
    let obj = StateObject::new(balance_id, signer, encode_balance(10));

    state1.insert(obj.clone()).unwrap();
    state2.insert(obj).unwrap();

    let txs = vec![
        ExternalTransaction {
            signer,
            nonce: 0,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        ExternalTransaction {
            signer,
            nonce: 0,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
    ];

    let mut block = Block {
        parent_hash: None,
        slot: Slot::new(1),
        epoch: Epoch::new(0),
        state_root: Hash::zero(),
        receipts_root: Hash::zero(),
        transactions: txs,
    };

    let engine = ReferenceExecutionEngine::default();

    let r1 = execute_block(&mut state1, &mut block, &engine);
    let r2 = execute_block(&mut state2, &mut block, &engine);

    assert_eq!(r1.tx_results.len(), r2.tx_results.len());
    assert_eq!(
        decode_balance(state1.get(&balance_id).unwrap()),
        decode_balance(state2.get(&balance_id).unwrap())
    );
}
