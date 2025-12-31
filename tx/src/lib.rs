//! Transaction cell abstraction for Axiom.
//!
//! A TransactionCell represents a deterministic, self-contained proposal
//! for a state transition with explicit read and write sets.

use std::collections::BTreeMap;

use axiom_types::{Hash, ObjectId};
use axiom_types::{Slot};
use axiom_state::{ReadSet, WriteSet};

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


/// A transaction cell.
///
/// This is the smallest schedulable execution unit in the Axiom protocol.
/// All state access is explicit and deterministic.
#[derive(Clone, Debug)]
pub struct TransactionCell {
    /// Slot in which the transaction is valid.
    slot: Slot,

    /// Read set of the transaction.
    read_set: ReadSet,

    /// Write set of the transaction.
    write_set: WriteSet,

    /// Call data describing execution intent.
    call: CallData,
}

impl TransactionCell {
    /// Creates a new transaction cell.
    pub fn new(
        slot: Slot,
        read_set: ReadSet,
        write_set: WriteSet,
        call: CallData,
    ) -> Result<Self, TxError> {
        // Enforce write ⊆ read
        for object_id in write_set.keys() {
            if !read_set.contains_key(object_id) {
                return Err(TxError::WriteWithoutRead {object: *object_id});
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
    pub fn id(&self) -> Hash {
        let mut bytes = Vec::new();

        // 1️⃣ Slot
        bytes.extend_from_slice(&self.slot.value().to_be_bytes());

        // 2️⃣ ReadSet (canonical order)
        let read: BTreeMap<_, _> = self.read_set.iter().collect();
        for (id, version) in read {
            bytes.extend_from_slice(id.as_hash().as_bytes());
            bytes.extend_from_slice(&version.to_be_bytes());
        }

        // 3️⃣ WriteSet (canonical order)
        let write: BTreeMap<_, _> = self.write_set.iter().collect();
        for (id, obj) in write {
            bytes.extend_from_slice(id.as_hash().as_bytes());
            bytes.extend_from_slice(&obj.version().to_be_bytes());
            bytes.extend_from_slice(obj.data());
        }

        // 4️⃣ CallData
        bytes.extend_from_slice(self.call.target.as_hash().as_bytes());
        bytes.extend_from_slice(&self.call.selector);
        bytes.extend_from_slice(&self.call.payload);

        // 5️⃣ Final hash
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

    /// Returns the write set of the transaction cell.
    pub fn write_set(&self) -> &WriteSet {
        &self.write_set
    }

    /// Returns the call data of the transaction cell.
    pub fn call(&self) -> &CallData {
        &self.call
    }
}

// Errors related to transaction cells.
#[derive(Debug)]
pub enum TxError {
    WriteWithoutRead {object: ObjectId},
}

// -------------------------------------------------------------------------------------------------------------------------- //
