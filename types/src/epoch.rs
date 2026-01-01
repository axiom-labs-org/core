
/// Imports and dependencies
use core::fmt;

// -------------------------------------------------------------------------------------------------------------------------- //

//----------------------------------------------- Epoch -----------------------------------------------//

/// Logical epoch number.
///
/// An `Epoch` represents a higher-level grouping of slots and is used
/// for protocol-wide transitions such as:
/// - validator set changes
/// - reward calculation
/// - governance windows
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epoch(u64);

impl Epoch {
    /// Creates a new `Epoch` from a raw u64 value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying u64 representation.
    pub const fn value(&self) -> u64 {
        self.0
    }

    /// Returns the next epoch.
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Debug for Epoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Epoch({})", self.0)
    }
}

impl fmt::Display for Epoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// -------------------------------------------------------------------------------------------------------------------------- //
