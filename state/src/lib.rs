//! Axiom state model.
//!
//! This crate defines the object-based state abstraction used by the Axiom
//! protocol. All on-chain state is represented as isolated, versioned objects.

pub mod store;
pub mod object;
pub mod nonce;
pub mod error;
pub mod balance;

pub use store::{StateStore, ReadSet, WriteSet};
pub use object::{StateObject, Version};
pub use nonce::{nonce_object_id, validate_and_prepare_nonce_update};
pub use error::{StateError, NonceError};
pub use balance::{balance_object_id, decode_balance, encode_balance};



