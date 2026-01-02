
/// Imports and dependencies
use std::collections::BTreeMap;

use axiom_state::{ReadSet, StateObject, StateStore};
use axiom_tx::{TransactionCell, WriteIntent};
use axiom_ext_tx::PreparedExternalTransaction;
use axiom_types::ObjectId;
use crate::PlanningError;

// -------------------------------------------------------------------------------------------------------------------------- //

//----------------------------------------------- Execution Plan -----------------------------------------------//

/// A fully prepared, execution-ready plan derived from an
/// authorized external transaction.
///
/// An ExecutionPlan is:
/// - deterministic
/// - state-aware
/// - execution-agnostic
///
/// It does NOT execute code or mutate state.
#[derive(Debug)]
pub struct ExecutionPlan {
    /// All objects that must be read, with expected versions.
    ///
    /// This is the merged ReadSet of all transaction cells
    /// plus any protocol-required reads.
    pub read_set: ReadSet,

    /// All objects that execution is allowed to write to.
    ///
    /// This is derived from declared WriteIntents.
    pub write_intents: BTreeMap<ObjectId, WriteIntent>,

    /// Mandatory state updates that MUST be applied if execution succeeds.
    ///
    /// These writes are injected by the protocol itself
    /// (e.g. nonce updates, fee deductions).
    pub forced_writes: BTreeMap<ObjectId, StateObject>,

    /// Transaction cells to be executed, in declared order.
    pub cells: Vec<TransactionCell>,
}

/// Build an execution plan from a prepared external transaction.
///
/// This function:
/// - merges read sets
/// - merges write intents
/// - injects forced protocol writes (nonce)
///
/// It does NOT:
/// - execute code
/// - mutate state
/// - validate ownership
pub fn build_execution_plan(
    petx: PreparedExternalTransaction,
    state: &StateStore,
) -> Result<ExecutionPlan, PlanningError> {
    let mut merged_read_set: ReadSet = ReadSet::new();
    let mut merged_write_intents: BTreeMap<ObjectId, WriteIntent> = BTreeMap::new();
    let mut forced_writes: BTreeMap<ObjectId, StateObject> = BTreeMap::new();
    
    let signer = petx.tx.signer;
    
    // ---------------------------------------------------------------------
    // 1️⃣ Inject forced nonce write
    // ---------------------------------------------------------------------
    let (nonce_id, nonce_object) = petx.nonce_update;
    forced_writes.insert(nonce_id, nonce_object);

    // ---------------------------------------------------------------------
    // 2️⃣ Merge cell read sets
    // ---------------------------------------------------------------------
    for cell in &petx.tx.cells {
        for (object_id, version) in cell.read_set() {
            match merged_read_set.get(object_id) {
                Some(existing_version) => {
                    if existing_version != version {
                        return Err(PlanningError::ReadConflict {
                            object: *object_id,
                            expected: *existing_version,
                            found: *version,
                        });
                    }
                }
                None => {
                    merged_read_set.insert(*object_id, *version);
                }
            }
        }
    }

    // ---------------------------------------------------------------------
    // 3️⃣ Merge cell write intents
    // ---------------------------------------------------------------------
    for cell in &petx.tx.cells {
        for (object_id, intent) in cell.write_set() {
            match merged_write_intents.get(object_id) {
                Some(existing_intent) => {
                    if existing_intent != intent {
                        return Err(PlanningError::WriteIntentConflict {
                            object: *object_id,
                        });
                    }
                }
                None => {
                    merged_write_intents.insert(*object_id, *intent);
                }
            }
        }
    }


    // ---------------------------------------------------------------------
    // 4️⃣ OWNERSHIP VALIDATION (NEW)
    // ---------------------------------------------------------------------

    for (object_id, intent) in &merged_write_intents {
        match intent {
            WriteIntent::Create => {
                // Object must not already exist
                if state.get(object_id).is_some() {
                    return Err(PlanningError::WriteIntentConflict { object: *object_id });
                }
            }

            WriteIntent::Modify | WriteIntent::Delete => {
                let object = state.get(object_id).ok_or(
                    PlanningError::ObjectNotFound { object: *object_id },
                )?;

                if object.owner() != signer {
                    return Err(PlanningError::UnauthorizedWrite { 
                        object: *object_id, 
                        owner: object.owner(), 
                        signer 
                    });
                }
            }
        }
    }

    // ---------------------------------------------------------------------
    // 5 Build final execution plan
    // ---------------------------------------------------------------------
    Ok(ExecutionPlan {
        read_set: merged_read_set,
        write_intents: merged_write_intents,
        forced_writes,
        cells: petx.tx.cells,
    })
}

