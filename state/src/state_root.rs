use axiom_types::{Hash, ObjectId};
use crate::{StateStore, StateObject};

/// Compute the canonical state root of the entire state store.
///
/// Deterministic, order-independent, consensus-safe.
pub fn compute_state_root(state: &StateStore) -> Hash {
    let mut entries: Vec<(&ObjectId, &StateObject)> =
        state.objects_iter().collect();

    // Canonical ordering
    entries.sort_by_key(|(id, _)| *id);

    let mut bytes = Vec::new();

    // Domain separation
    bytes.extend_from_slice(b"Axiom::StateRoot::v1");

    for (id, object) in entries {
        // Object ID
        bytes.extend_from_slice(id.as_hash().as_bytes());

        // Version
        bytes.extend_from_slice(&object.version().to_be_bytes());

        // Object data hash
        let data_hash = blake3::hash(object.data());
        bytes.extend_from_slice(data_hash.as_bytes());
    }

    Hash::new(blake3::hash(&bytes).into())
}
