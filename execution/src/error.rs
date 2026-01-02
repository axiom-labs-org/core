
use axiom_types::ObjectId;

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
}