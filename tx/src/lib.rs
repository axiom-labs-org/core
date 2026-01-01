//! Transaction cell abstraction for Axiom.
//!
//! A TransactionCell represents a deterministic, self-contained proposal
//! for a state transition with explicit read and write sets.

pub mod transaction_cell;

pub use transaction_cell::{TransactionCell, CallData};
