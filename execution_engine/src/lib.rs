//! Execution engine interface for Axiom.
//!
//! This crate defines the VM-agnostic execution boundary of the protocol.
//! It does NOT contain:
//! - authorization logic
//! - fee logic
//! - state mutation
//! - consensus logic
//!
//! Execution engines are replaceable; protocol rules are not.

pub mod context;
pub mod engine;
pub mod error;
pub mod reference;

pub use context::ExecutionContext;
pub use engine::{ExecutionEngine, ExecutionOutcome, StateView};
pub use error::ExecutionError;
pub use reference::ReferenceExecutionEngine;