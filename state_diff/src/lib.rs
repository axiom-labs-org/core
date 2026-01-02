//! Canonical state transition layer for Axiom.
//!
//! This crate defines StateDiff and atomic commit semantics.

pub mod diff;
pub mod commit;
pub mod error;

pub use diff::StateDiff;
pub use commit::commit_state_diff;
pub use error::CommitError;

