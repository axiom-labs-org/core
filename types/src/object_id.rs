
/// Imports and dependencies
use core::fmt;
use crate::Hash;
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