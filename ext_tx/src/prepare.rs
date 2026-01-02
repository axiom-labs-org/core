
/// Imports and dependencies
use axiom_state::{
    StateStore,
    StateObject,
    validate_and_prepare_nonce_update,
    NonceError,
};

use axiom_types::ObjectId;

use crate::external_tx::ExternalTransaction;

// -------------------------------------------------------------------------------------------------------------------------- //

//----------------------------------------------- Prepare External Transactions -----------------------------------------------//

/// An external transaction that has passed authorization checks
/// and is ready for execution planning.
#[derive(Debug)]
pub struct PreparedExternalTransaction {
    /// Original user-submitted transaction
    pub tx: ExternalTransaction,

    /// Nonce state update that must be applied atomically
    pub nonce_update: (ObjectId, StateObject),
}

/// Validate an external transaction and prepare it for execution.
///
/// This performs authorization checks only:
/// - nonce validation
///
/// It does NOT:
/// - verify signatures (yet)
/// - execute cells
/// - mutate state
pub fn prepare_external_transaction(
    tx: ExternalTransaction,
    state: &StateStore,
) -> Result<PreparedExternalTransaction, NonceError> {
    let nonce_update = validate_and_prepare_nonce_update(
        tx.signer,
        tx.nonce,
        state,
    )?;

    Ok(PreparedExternalTransaction {
        tx,
        nonce_update,
    })
}

// -------------------------------------------------------------------------------------------------------------------------- //
