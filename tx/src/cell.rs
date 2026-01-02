/// Imports and dependencies
use std::collections::BTreeMap;

use axiom_types::{Hash, ObjectId, Slot};
use axiom_state::{ReadSet};
use crate::TxError;

// -------------------------------------------------------------------------------------------------------------------------- //

//----------------------------------------------- Transaction Cell -----------------------------------------------//

/// Opaque call data describing execution intent.
///
/// The protocol does not interpret call data. It is passed to the
/// execution runtime, which enforces read/write restrictions.
#[derive(Clone, Debug)]
pub struct CallData {
    /// Target of the call (contract or object).
    pub target: ObjectId,

    /// Method selector or entrypoint identifier.
    pub selector: Vec<u8>,

    /// Encoded call payload.
    pub payload: Vec<u8>,
}

/// Intent describing which objects may be written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteIntent {
    Create,
    Modify,
    Delete,
}

/// A transaction cell.
///
/// This is the smallest schedulable execution unit in the Axiom protocol.
/// It declares *what may be touched*, not *what will be written*.
#[derive(Clone, Debug)]
pub struct TransactionCell {
    /// Slot in which the transaction is valid (context, not identity).
    slot: Slot,

    /// Read set of the transaction.
    read_set: ReadSet,

    /// Write intent set of the transaction.
    write_set: BTreeMap<ObjectId, WriteIntent>,

    /// Call data describing execution intent.
    call: CallData,
}

impl TransactionCell {
    /// Creates a new transaction cell.
    pub fn new(
        slot: Slot,
        read_set: ReadSet,
        write_set: BTreeMap<ObjectId, WriteIntent>,
        call: CallData,
    ) -> Result<Self, TxError> {
        // Enforce write ⊆ read
        for object_id in write_set.keys() {
            if !read_set.contains_key(object_id) {
                return Err(TxError::WriteWithoutRead {
                    object: *object_id,
                });
            }
        }

        Ok(Self {
            slot,
            read_set,
            write_set,
            call,
        })
    }

    /// Computes the deterministic identifier for this transaction cell.
    ///
    /// IMPORTANT:
    /// - Depends only on declared intent
    /// - Must NOT depend on execution output
    /// - Slot is intentionally excluded
    pub fn id(&self) -> Hash {
        let mut bytes = Vec::new();

        // 1️⃣ ReadSet (canonical order)
        for (id, version) in &self.read_set {
            bytes.extend_from_slice(id.as_hash().as_bytes());
            bytes.extend_from_slice(&version.to_be_bytes());
        }

        // 2️⃣ WriteSet (canonical order)
        for (id, intent) in &self.write_set {
            bytes.extend_from_slice(id.as_hash().as_bytes());
            bytes.push(*intent as u8);
        }

        // 3️⃣ CallData
        bytes.extend_from_slice(self.call.target.as_hash().as_bytes());
        bytes.extend_from_slice(&self.call.selector);
        bytes.extend_from_slice(&self.call.payload);

        Hash::new(blake3::hash(&bytes).into())
    }

    /// Returns the slot of the transaction cell.
    pub fn slot(&self) -> Slot {
        self.slot
    }

    /// Returns the read set of the transaction cell.
    pub fn read_set(&self) -> &ReadSet {
        &self.read_set
    }

    /// Returns the write intent set of the transaction cell.
    pub fn write_set(&self) -> &BTreeMap<ObjectId, WriteIntent> {
        &self.write_set
    }

    /// Returns the call data of the transaction cell.
    pub fn call(&self) -> &CallData {
        &self.call
    }
}

// -------------------------------------------------------------------------------------------------------------------------- //
