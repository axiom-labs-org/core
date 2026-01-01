
/// Imports and dependencies
use core::fmt;

// -------------------------------------------------------------------------------------------------------------------------- //

//----------------------------------------------- Account Address -----------------------------------------------//

/// Length (in bytes) of a Axiom address
pub const ADDRESS_LENGTH: usize = 32;

/// Axiom address.
///
/// An `Address` is a fixed-size, immutable identifier used for:
/// - user accounts
/// - smart contracts
/// - protocol-owned objects
///
/// Addresses have no inherent meaning beyond identity.

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address([u8; ADDRESS_LENGTH]);

impl Address {
    /// Creates a new `Address` from raw bytes.
    ///
    /// This function does not perform semantic validation.
    /// It is the caller's responsibility to ensure correctness.
    pub const fn new(bytes: [u8; ADDRESS_LENGTH]) -> Self {
        Self(bytes)
    }

    /// Returns the underlying byte representation.
    pub const fn as_bytes(&self) -> &[u8; ADDRESS_LENGTH] {
        &self.0
    }

    /// Returns the zero address (all bytes set to zero).
    ///
    /// ⚠️ PROTOCOL NOTE:
    /// - The zero address is NOT a valid user.
    /// - It MUST NOT be used for authorization or ownership.
    /// - It is intended only as a sentinel or placeholder value.
    /// - Any semantic meaning must be defined explicitly at higher layers.
    pub const fn zero() -> Self {
        Self([0u8; ADDRESS_LENGTH])
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address(0x")?;
        for byte in &self.0[..4] {
            write!(f, "{:02x}", byte)?;
        }
        write!(f, "...)") // truncate for readability
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------------------------------- //
