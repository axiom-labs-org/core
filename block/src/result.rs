use axiom_protocol::ProtocolError;

/// Result of executing a single transaction within a block.
#[derive(Debug)]
pub enum TransactionResult {
    /// Transaction executed successfully.
    Success {
        /// Fee charged for the transaction.
        fee_charged: u64,
    },

    /// Transaction failed and caused no state changes.
    Failure {
        /// Reason for failure.
        error: ProtocolError,
    },
}

/// Result of executing an entire block.
#[derive(Debug)]
pub struct BlockExecutionResult {
    /// Per-transaction execution results in block order.
    pub tx_results: Vec<TransactionResult>,
}
