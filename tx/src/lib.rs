//! Transaction cell abstraction for Axiom.
//!
//! A TransactionCell represents a deterministic, self-contained proposal
//! for a state transition with explicit read and write sets.

use axiom_types::{Hash, Slot};
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
    pub target: Hash,

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
    ) -> Self {
        Self {
            slot,
            read_set,
            write_set,
            call,
        }
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

// -------------------------------------------------------------------------------------------------------------------------- //
