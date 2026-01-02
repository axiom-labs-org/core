use axiom_types::{Slot, Epoch};

/// Immutable execution context provided by the protocol.
#[derive(Debug, Clone, Copy)]
pub struct ExecutionContext {
    pub slot: Slot,
    pub epoch: Epoch,
}
