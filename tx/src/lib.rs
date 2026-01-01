//! Transaction cell abstraction for Axiom.
//!
//! A TransactionCell represents a deterministic, self-contained proposal
//! for a state transition with explicit read and write sets.

pub mod cell;
pub mod error;

pub use cell::{TransactionCell, CallData};
pub use error::TxError;
