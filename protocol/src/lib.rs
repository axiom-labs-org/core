//! Axiom protocol orchestration layer.
//!
//! This crate wires together authorization, execution, and state commits.

pub mod process;
pub mod error;

pub use process::process_external_transaction;
pub use error::ProtocolError;
