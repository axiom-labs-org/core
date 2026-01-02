//! Execution planning layer for Axiom.
//!
//! This crate defines how authorized external transactions are
//! transformed into deterministic execution plans.

pub mod plan;
pub mod error;

pub use plan::ExecutionPlan;
pub use error::PlanningError;