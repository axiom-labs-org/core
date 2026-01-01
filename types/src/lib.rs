//! Axiom core primitive types.
//! 
//! This module defines the core primitive types used throughout the Axiom system.
//! These types are intentionally kept minimal, deterministic and invariant-preserving

mod address;
mod hash;
mod slot;
mod epoch;
mod object_id;

pub use address::Address;
pub use hash::Hash;
pub use slot::Slot;
pub use epoch::Epoch;
pub use object_id::ObjectId;