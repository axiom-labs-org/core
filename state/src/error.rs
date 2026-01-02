
/// Imports and dependencies

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
    InvalidNonce {
        expected: u64,
        got: u64,
    },
    DecodeError,
}
