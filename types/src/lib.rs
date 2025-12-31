//! Axiom core primitive types.
//! 
//! This module defines the core primitive types used throughout the Axiom system.
//! These types are intentionally kept minimal, deterministic and invariant-preserving

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

    /// Returns a zero address (all bytes set to zero).
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

//----------------------------------------------- ObjectId -----------------------------------------------//

/// Identifier for a state object.
///
/// An `ObjectId` uniquely identifies a piece of on-chain state such as:
/// - an account
/// - a smart contract
/// - an AMM pool
/// - a prediction market
///
/// ObjectIds are opaque identifiers with no embedded semantics.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjectId(Hash);

impl ObjectId {
    /// Creates a new `ObjectId` from a hash.
    ///
    /// The hash is typically derived deterministically from:
    /// - creator address
    /// - creation slot
    /// - object-specific salt
    pub const fn new(hash: Hash) -> Self {
        Self(hash)
    }

    /// Returns the underlying `Hash` representation.
    pub const fn as_hash(&self) -> &Hash {
        &self.0
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObjectId({:?})", self.0)
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}