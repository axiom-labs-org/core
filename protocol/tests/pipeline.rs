use std::collections::{HashMap, BTreeMap};

use axiom_protocol::process_external_transaction;
use axiom_execution_engine::{ReferenceExecutionEngine, ExecutionContext};
use axiom_state::{StateStore, StateObject};
use axiom_state::nonce::nonce_object_id;
use axiom_state::balance::{balance_object_id, encode_balance, decode_balance};
use axiom_ext_tx::{ExternalTransaction, Signature};
use axiom_tx::{TransactionCell, CallData};
use axiom_types::{Address, Slot, Epoch, ObjectId};

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
// Test: Valid transaction succeeds and mutates state correctly
// -------------------------------------------------------------
#[test]
fn pipeline_valid_transaction_succeeds() {
    let mut state = StateStore::new();
    let signer = Address::new([1u8; 32]);

    let balance_id = balance_object_id(signer);
    state.insert(StateObject::new(balance_id, signer, encode_balance(10))).unwrap();

    let tx = ExternalTransaction {
        signer,
        nonce: 0,
        cells: vec![make_cell(balance_id)],
        signature: Signature { bytes: vec![] },
    };

    let engine = ReferenceExecutionEngine::default();
    let ctx = ExecutionContext { slot: Slot::new(1), epoch: Epoch::new(0) };

    process_external_transaction(&mut state, tx, &engine, ctx).unwrap();

    assert_eq!(decode_balance(state.get(&balance_id).unwrap()), 9);
    assert!(state.get(&nonce_object_id(signer)).is_some());
}

// -------------------------------------------------------------
// Test: Transaction with invalid nonce is rejected
// -------------------------------------------------------------
#[test]
fn pipeline_rejects_invalid_nonce() {
    let mut state = StateStore::new();
    let signer = Address::new([2u8; 32]);

    let balance_id = balance_object_id(signer);
    state.insert(StateObject::new(balance_id, signer, encode_balance(10))).unwrap();

    let tx = ExternalTransaction {
        signer,
        nonce: 1, // ❌ invalid
        cells: vec![make_cell(balance_id)],
        signature: Signature { bytes: vec![] },
    };

    let engine = ReferenceExecutionEngine::default();
    let ctx = ExecutionContext { slot: Slot::new(1), epoch: Epoch::new(0) };

    assert!(process_external_transaction(&mut state, tx, &engine, ctx).is_err());

    assert!(state.get(&nonce_object_id(signer)).is_none());
    assert_eq!(decode_balance(state.get(&balance_id).unwrap()), 10);
}

// -------------------------------------------------------------
// Test: Transaction with stale nonce is rejected
// -------------------------------------------------------------
#[test]
fn pipeline_rejects_stale_nonce() {
    let mut state = StateStore::new();
    let signer = Address::new([3u8; 32]);

    let balance_id = balance_object_id(signer);
    state.insert(StateObject::new(balance_id, signer, encode_balance(10))).unwrap();

    let engine = ReferenceExecutionEngine::default();
    let ctx = ExecutionContext { slot: Slot::new(1), epoch: Epoch::new(0) };

    // tx #1 (nonce = 0) → creates nonce object
    process_external_transaction(
        &mut state,
        ExternalTransaction {
            signer,
            nonce: 0,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        &engine,
        ctx,
    ).unwrap();

    // tx #2 (nonce = 0) → increments nonce to 1
    process_external_transaction(
        &mut state,
        ExternalTransaction {
            signer,
            nonce: 0,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        &engine,
        ctx,
    ).unwrap();

    // tx #3 with stale nonce = 0 ❌
    let result = process_external_transaction(
        &mut state,
        ExternalTransaction {
            signer,
            nonce: 0, // stale
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        &engine,
        ctx,
    );

    assert!(result.is_err());
}


// -------------------------------------------------------------
// Test: Sequential transactions increment nonce correctly
// -------------------------------------------------------------
#[test]
fn pipeline_sequential_transactions_increment_nonce() {
    let mut state = StateStore::new();
    let signer = Address::new([4u8; 32]);

    let balance_id = balance_object_id(signer);
    state.insert(StateObject::new(balance_id, signer, encode_balance(10))).unwrap();

    let engine = ReferenceExecutionEngine::default();
    let ctx = ExecutionContext { slot: Slot::new(1), epoch: Epoch::new(0) };

    // tx #1
    process_external_transaction(
        &mut state,
        ExternalTransaction {
            signer,
            nonce: 0,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        &engine,
        ctx,
    ).unwrap();

    // tx #2
    process_external_transaction(
        &mut state,
        ExternalTransaction {
            signer,
            nonce: 0,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        &engine,
        ctx,
    ).unwrap();

    // tx #3
    process_external_transaction(
        &mut state,
        ExternalTransaction {
            signer,
            nonce: 1,
            cells: vec![make_cell(balance_id)],
            signature: Signature { bytes: vec![] },
        },
        &engine,
        ctx,
    ).unwrap();

    let nonce_obj = state.get(&nonce_object_id(signer)).unwrap();
    assert_eq!(nonce_obj.version(), 2);

    let balance = decode_balance(state.get(&balance_id).unwrap());
    assert_eq!(balance, 7);
}


// -------------------------------------------------------------
// Test: Transaction with insufficient balance is rejected
// -------------------------------------------------------------
#[test]
fn pipeline_rejects_insufficient_balance() {
    let mut state = StateStore::new();
    let signer = Address::new([5u8; 32]);

    let balance_id = balance_object_id(signer);
    state.insert(StateObject::new(balance_id, signer, encode_balance(0))).unwrap();

    let tx = ExternalTransaction {
        signer,
        nonce: 0,
        cells: vec![make_cell(balance_id)],
        signature: Signature { bytes: vec![] },
    };

    let engine = ReferenceExecutionEngine::default();
    let ctx = ExecutionContext { slot: Slot::new(1), epoch: Epoch::new(0) };

    assert!(process_external_transaction(&mut state, tx, &engine, ctx).is_err());

    assert!(state.get(&nonce_object_id(signer)).is_none());
    assert_eq!(decode_balance(state.get(&balance_id).unwrap()), 0);
}

// -------------------------------------------------------------
// Test: Transaction failure leaves state unchanged
// -------------------------------------------------------------
#[test]
fn pipeline_failure_is_atomic() {
    let mut state = StateStore::new();
    let signer = Address::new([6u8; 32]);

    let balance_id = balance_object_id(signer);
    state.insert(StateObject::new(balance_id, signer, encode_balance(5))).unwrap();

    // Snapshot observable state
    let balance_before = decode_balance(state.get(&balance_id).unwrap());
    let nonce_id = nonce_object_id(signer);
    let nonce_exists_before = state.get(&nonce_id).is_some();

    let tx = ExternalTransaction {
        signer,
        nonce: 1, // invalid
        cells: vec![make_cell(balance_id)],
        signature: Signature { bytes: vec![] },
    };

    let engine = ReferenceExecutionEngine::default();
    let ctx = ExecutionContext { slot: Slot::new(1), epoch: Epoch::new(0) };

    let _ = process_external_transaction(&mut state, tx, &engine, ctx);

    // Assert state unchanged
    let balance_after = decode_balance(state.get(&balance_id).unwrap());
    let nonce_exists_after = state.get(&nonce_id).is_some();

    assert_eq!(balance_before, balance_after);
    assert_eq!(nonce_exists_before, nonce_exists_after);
}
