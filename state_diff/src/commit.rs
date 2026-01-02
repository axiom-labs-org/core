use axiom_state::StateStore;
use crate::StateDiff;
use crate::CommitError;

/// Atomically apply a StateDiff to the StateStore.
pub fn commit_state_diff(
    state: &mut StateStore,
    diff: StateDiff,
) -> Result<(), CommitError> {
    // 1️⃣ Validate read set
    for (object_id, expected_version) in &diff.read_set {
        let object = state.get(object_id).ok_or(
            CommitError::ObjectNotFound { object: *object_id }
        )?;

        if object.version() != *expected_version {
            return Err(CommitError::StaleRead {
                object: *object_id,
                expected: *expected_version,
                found: object.version(),
            });
        }
    }

    // 2️⃣ Apply writes atomically
    for (_, object) in diff.writes {
        // copy BEFORE move
        let object_id = object.id(); 

        state.insert_or_update(object).map_err(|_| {
            CommitError::InvalidWrite {
                object: object_id,
            }
        })?;
    }

    Ok(())
}
