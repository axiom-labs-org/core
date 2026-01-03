//! Block-level execution data structures for Axiom.
//!
//! This crate defines block inputs and execution outputs.

pub mod block;
pub mod result;
pub mod execute;
pub mod encode;
pub mod hash;
pub mod receipts_root;

pub use block::Block;
pub use result::{TransactionResult, BlockExecutionResult};
pub use execute::execute_block;
pub use hash::block_hash;
pub use receipts_root::compute_receipts_root;
