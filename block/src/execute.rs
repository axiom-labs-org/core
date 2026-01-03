use axiom_execution_engine::{ExecutionEngine, ExecutionContext};
use axiom_protocol::{process_external_transaction};
use axiom_state::{StateStore, compute_state_root};

use crate::{Block, BlockExecutionResult, TransactionResult, compute_receipts_root};

/// Execute a block against the given state.
///
/// Transactions are executed sequentially in block order.
/// Each transaction is atomic: failures do not affect state.
pub fn execute_block<E: ExecutionEngine>(
    state: &mut StateStore,
    block: &mut Block,
    engine: &E,
) -> BlockExecutionResult {
    let mut tx_results = Vec::with_capacity(block.transactions.len());
    let mut tx_hashes = Vec::with_capacity(block.transactions.len());

    let context = ExecutionContext {
        slot: block.slot,
        epoch: block.epoch,
    };

    for tx in &block.transactions {
        let tx_hash = tx.signing_hash();
        tx_hashes.push(tx_hash);
        
        let result = process_external_transaction(
            state,
            tx.clone(),
            engine,
            context,
        );

        match result {
            Ok(()) => {
                // For now, fee is fixed and known (e.g. 1)
                tx_results.push(TransactionResult::Success {
                    fee_charged: 1,
                });
            }
            Err(err) => {
                // State must be unchanged here (guaranteed by pipeline)
                tx_results.push(TransactionResult::Failure {
                    error: err,
                });
            }
        }
    }

    // 🔒 Commit execution results
    block.state_root = compute_state_root(state);
    block.receipts_root = compute_receipts_root(&tx_hashes, &tx_results);

    BlockExecutionResult {
        tx_results,
    }
}
