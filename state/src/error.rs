
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
