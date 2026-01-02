
use axiom_types::{Address, ObjectId};

#[derive(Debug)]
pub enum PlanningError {
    /// Conflicting read versions for the same object.
    ReadConflict {
        object: ObjectId,
        expected: u64,
        found: u64,
    },

    /// Conflicting write intents for the same object.
    WriteIntentConflict {
        object: ObjectId,
    },

    // NEW: ownership
    ObjectNotFound {
        object: ObjectId,
    },

    UnauthorizedWrite {
        object: ObjectId,
        owner: Address,
        signer: Address,
    },
}