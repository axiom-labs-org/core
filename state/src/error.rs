
/// Imports and dependencies
use axiom_types::{Address, ObjectId};

// -------------------------------------------------------------------------------------------------------------------------- //

/// Errors returned by the state store.
#[derive(Debug)]
pub enum StateError {
    ObjectAlreadyExists,
    ObjectNotFound,
    StaleRead { expected: u64, found: u64 },
    InvalidVersion { expected: u64, found: u64 },
}

// -------------------------------------------------------------------------------------------------------------------------- //

/// Errors that can occur during nonce validation.
#[derive(Debug)]
pub enum NonceError {
    NonceTooLow {
        expected: u64,
        found: u64,
    },
    NonceTooHigh {
        expected: u64,
        found: u64,
    },
    Unauthorized {
        owner: Address,
        caller: Address,
    },
    NonceObjectMissing {
        object: ObjectId,
    },
    InvalidNonce {
        expected: u64,
        got: u64,
    },
    DecodeError,
}
