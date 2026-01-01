
/// Imports and dependencies
use core::fmt;

//----------------------------------------------- Hash -----------------------------------------------//

/// Length (in bytes) of a cryptographic hash
pub const HASH_LENGTH: usize = 32;

/// Cryptographic hash.
///
/// A `Hash` represents the output of a cryptographic hashing function
/// and is used for:
/// - block identifiers
/// - transaction identifiers
/// - state roots
/// - Merkle roots
///
/// Hashes have no identity or ownership semantics
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Hash([u8; HASH_LENGTH]);

impl Hash {
    /// Creates a new `Hash` from raw bytes.
    ///
    /// This function assumes the bytes are the result of a
    /// cryptographic hashing function.
    pub const fn new(bytes: [u8; HASH_LENGTH]) -> Self {
        Self(bytes)
    }

    /// Returns the underlying byte representation.
    pub const fn as_bytes(&self) -> &[u8; HASH_LENGTH] {
        &self.0
    }

    /// Returns a zero hash (all bytes set to zero).
    ///
    /// This is useful for genesis values and placeholders.
    pub const fn zero() -> Self {
        Self([0u8; HASH_LENGTH])
    }   
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash(0x")?;
        for byte in &self.0[..4] {
            write!(f, "{:02x}", byte)?;
        }
        write!(f, "...)") // truncate for readability
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------------------------------- //
