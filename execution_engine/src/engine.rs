use std::collections::BTreeMap;

use axiom_types::ObjectId;
use axiom_state::StateObject;
use axiom_execution::ExecutionPlan;

use crate::context::ExecutionContext;
use crate::error::ExecutionError;

/// Read-only view into protocol state.
///
/// Execution engines MUST NOT mutate state directly.
/// All mutations must be returned as write proposals.
pub trait StateView {
    /// Fetch a state object by ID.
    ///
    /// Implementations MUST return:
    /// - `Some(&StateObject)` if the object exists
    /// - `None` if the object does not exist
    fn get_object(&self, id: &ObjectId) -> Option<&StateObject>;
}

/// Result of executing an execution plan.
///
/// These writes are:
/// - produced by execution logic
/// - merged with forced protocol writes
/// - validated again before commit
#[derive(Debug)]
pub struct ExecutionOutcome {
    /// Objects written by execution logic.
    ///
    /// These MUST:
    /// - correspond to declared write intents
    /// - increment versions correctly
    pub writes: BTreeMap<ObjectId, StateObject>,
}

/// VM-agnostic execution engine interface.
///
/// The execution engine:
/// - executes validated plans
/// - is deterministic
/// - is sandboxed
/// - has no side effects
///
/// The protocol enforces all invariants around it.
pub trait ExecutionEngine {
    /// Execute a validated execution plan.
    ///
    /// # Guarantees
    /// - No state mutation occurs here
    /// - No undeclared reads or writes are allowed
    /// - Deterministic for identical inputs
    ///
    /// # Errors
    /// Any error MUST be deterministic and consensus-safe.
    fn execute(
        &self,
        plan: &ExecutionPlan,
        state: &dyn StateView,
        context: ExecutionContext,
    ) -> Result<ExecutionOutcome, ExecutionError>;
}
