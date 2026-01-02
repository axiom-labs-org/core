use std::collections::BTreeMap;

use axiom_execution::ExecutionPlan;
// use axiom_state::StateObject;
// use axiom_types::ObjectId;

use crate::{
    ExecutionContext,
    ExecutionEngine,
    ExecutionOutcome,
    ExecutionError,
    StateView,
};

/// Reference execution engine.
///
/// This engine performs NO computation.
/// It only validates that execution respects declared constraints.
#[derive(Debug, Default)]
pub struct ReferenceExecutionEngine;

impl ExecutionEngine for ReferenceExecutionEngine {
    fn execute(
        &self,
        plan: &ExecutionPlan,
        state: &dyn StateView,
        _context: ExecutionContext,
    ) -> Result<ExecutionOutcome, ExecutionError> {
        // -------------------------------------------------------------
        // 1️⃣ Validate all declared reads are accessible
        // -------------------------------------------------------------
        for (object_id, _) in &plan.read_set {
            if state.get_object(object_id).is_none() {
                return Err(ExecutionError::UnauthorizedRead {
                    object: *object_id,
                });
            }
        }

        // -------------------------------------------------------------
        // 2️⃣ Validate write intents reference existing objects if needed
        // -------------------------------------------------------------
        for (object_id, intent) in &plan.write_intents {
            match intent {
                // Create: object must NOT exist yet
                axiom_tx::WriteIntent::Create => {
                    if state.get_object(object_id).is_some() {
                        return Err(ExecutionError::UnauthorizedWrite {
                            object: *object_id,
                        });
                    }
                }

                // Modify/Delete: object MUST exist
                axiom_tx::WriteIntent::Modify | axiom_tx::WriteIntent::Delete => {
                    if state.get_object(object_id).is_none() {
                        return Err(ExecutionError::UnauthorizedWrite {
                            object: *object_id,
                        });
                    }
                }
            }
        }

        // -------------------------------------------------------------
        // 3️⃣ Produce empty execution writes
        // -------------------------------------------------------------
        Ok(ExecutionOutcome {
            writes: BTreeMap::new(),
        })
    }
}
