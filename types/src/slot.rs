
/// Imports and dependencies
use core::fmt;

// -------------------------------------------------------------------------------------------------------------------------- //

//----------------------------------------------- Slot -----------------------------------------------//

/// Logical slot number.
///
/// A `Slot` represents a strictly increasing unit of logical time
/// used to order blocks and transactions.
///
/// Slots do NOT correspond to wall-clock time.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot(u64);

impl Slot {
    /// Creates a new `Slot` from a raw u64 value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying u64 representation.
    pub const fn value(&self) -> u64 {
        self.0
    }

    /// Returns the next slot.
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Debug for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Slot({})", self.0)
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// -------------------------------------------------------------------------------------------------------------------------- //
