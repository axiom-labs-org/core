/// Imports and dependencies
use axiom_types::{Address, ObjectId};

// -------------------------------------------------------------------------------------------------------------------------- //

/// Errors related to transaction cells.
#[derive(Debug)]
pub enum TxError {
    WriteWithoutRead {object: ObjectId},
    UnauthorizedWrite {
        object: ObjectId,
        owner: Address,
        caller: Address,
    },
    ObjectNotFound {object: ObjectId},
}

// -------------------------------------------------------------------------------------------------------------------------- //

