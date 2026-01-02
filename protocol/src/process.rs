use axiom_ext_tx::{ExternalTransaction, prepare_external_transaction};
use axiom_execution::{build_execution_plan};
use axiom_execution_engine::{
    ExecutionEngine,
    ExecutionContext,
};
use axiom_state::{StateStore};
use axiom_state_diff::{StateDiff, commit_state_diff};

use crate::error::ProtocolError;

/// Process a single external transaction against the current state.
///
/// This is the canonical state transition pipeline.
pub fn process_external_transaction<E: ExecutionEngine>(
    state: &mut StateStore,
    tx: ExternalTransaction,
    engine: &E,
    context: ExecutionContext,
) -> Result<(), ProtocolError> {
    // -------------------------------------------------------------
    // 1️⃣ Authorization (nonce)
    // -------------------------------------------------------------
    let prepared = prepare_external_transaction(tx, state)
        .map_err(ProtocolError::NonceError)?;

    // -------------------------------------------------------------
    // 2️⃣ Execution planning (ownership, fees, intents)
    // -------------------------------------------------------------
    let plan = build_execution_plan(prepared, state)
        .map_err(ProtocolError::PlanningError)?;

    // -------------------------------------------------------------
    // 3️⃣ Execute plan (VM / reference engine)
    // -------------------------------------------------------------
    let outcome = engine
        .execute(&plan, state, context)
        .map_err(ProtocolError::ExecutionError)?;

    // -------------------------------------------------------------
    // 4️⃣ Build StateDiff
    // -------------------------------------------------------------
    let mut writes = plan.forced_writes.clone();

    for (id, obj) in outcome.writes {
        if writes.contains_key(&id) {
            // Execution attempted to override protocol write
            return Err(ProtocolError::ExecutionError(
                axiom_execution_engine::ExecutionError::UnauthorizedWrite {
                    object: id,
                },
            ));
        }
        writes.insert(id, obj);
    }

    let diff = StateDiff {
        read_set: plan.read_set.clone(),
        writes,
    };

    // -------------------------------------------------------------
    // 5️⃣ Commit atomically
    // -------------------------------------------------------------
    commit_state_diff(state, diff)
        .map_err(ProtocolError::CommitError)?;

    Ok(())
}
