//! Define external transactions

/// Module for transaction-related types and utilities.
use axiom_tx::TransactionCell;
use axiom_types::{Address, Hash};
use axiom_state::{Version};

// -------------------------------------------------------------------------------------------------------------------------- //

//----------------------------------------------- External Transactions -----------------------------------------------//

/// Opaque transaction signature.
///
/// NOTE: Cryptographic verification is handled later.
/// For now, this is just a typed container.
#[derive(Clone, Debug)]
pub struct Signature {
    pub bytes: Vec<u8>,
}

/// External transaction submitted by users.
#[derive(Debug, Clone)]
pub struct ExternalTransaction {
    /// Address authorizing the transaction.
    pub signer: Address,

    /// Nonce to prevent replay attacks.
    pub nonce: Version,

    /// Cells involved in the transaction.
    pub cells: Vec<TransactionCell>,

    /// Signature verifying the transaction's authenticity.
    pub signature: Signature,
}

impl ExternalTransaction {
    /// Compute the hash of the transaction payload to be signed
    pub fn signing_hash(&self) -> Hash {
        let mut bytes = Vec::new();
        
        // Domain separator
        bytes.extend_from_slice(b"Axiom::ExternalTransaction::v1");

        // Signer
        bytes.extend_from_slice(self.signer.as_bytes());

        // Nonce
        bytes.extend_from_slice(&self.nonce.to_le_bytes());

        // Cells
        for cell in &self.cells {
            bytes.extend_from_slice(cell.id().as_bytes());
        }

        Hash::new(blake3::hash(&bytes).into())
    }
}

// -------------------------------------------------------------------------------------------------------------------------- //

