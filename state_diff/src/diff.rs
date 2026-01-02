use std::collections::BTreeMap;

use axiom_types::ObjectId;
use axiom_state::{StateObject, ReadSet};

/// Canonical state transition produced by execution.
#[derive(Debug)]
pub struct StateDiff {
    /// Objects that were read, with expected versions.
    ///
    /// Used for optimistic concurrency control.
    pub read_set: ReadSet,

    /// Objects that will be written.
    ///
    /// Includes:
    /// - execution writes
    /// - forced protocol writes (nonce, fees)
    pub writes: BTreeMap<ObjectId, StateObject>,
}
