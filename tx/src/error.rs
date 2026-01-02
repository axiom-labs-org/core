/// Imports and dependencies
use axiom_types::{ObjectId};

// -------------------------------------------------------------------------------------------------------------------------- //

/// Errors related to transaction cells.
#[derive(Debug)]
pub enum TxError {
    WriteWithoutRead {object: ObjectId},
    ObjectNotFound {object: ObjectId},
}

// -------------------------------------------------------------------------------------------------------------------------- //

