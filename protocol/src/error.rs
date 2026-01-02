use axiom_execution_engine::ExecutionError;
use axiom_state_diff::CommitError;
use axiom_execution::PlanningError;
use axiom_state::NonceError;
use axiom_tx::TxError;

#[derive(Debug)]
pub enum ProtocolError {
    /// External transaction failed authorization
    NonceError(NonceError),

    /// Transaction intent was invalid
    TxError(TxError),

    /// Execution planning failed
    PlanningError(PlanningError),

    /// Execution engine failed
    ExecutionError(ExecutionError),

    /// State commit failed
    CommitError(CommitError),
}
