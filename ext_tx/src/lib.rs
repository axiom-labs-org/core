//! Define external transactions

pub mod external_tx;
pub mod prepare;

pub use external_tx::{ExternalTransaction, Signature};
pub use prepare::{PreparedExternalTransaction, prepare_external_transaction};
