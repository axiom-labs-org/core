
/// Imports and dependencies
use axiom_types::{Address, ObjectId, Hash};
// -------------------------------------------------------------------------------------------------------------------------- //

//----------------------------------------------- Nonce -----------------------------------------------//

/// Domain prefix used for deriving nonce object IDs.
///
/// This ensures nonce objects live in a reserved, collision-free
/// namespace inside global state.
const NONCE_DOMAIN: &[u8] = b"axiom::nonce";

/// Derives the deterministic nonce ObjectId for a given address.
///
/// There is exactly one nonce object per address.
/// This function must always return the same ObjectId
/// for the same address across all nodes.
pub fn nonce_object_id(address: Address) -> ObjectId {
    let mut bytes = Vec::new();

    // Domain separation
    bytes.extend_from_slice(NONCE_DOMAIN);
    
    // Bind nonce to a specific address
    bytes.extend_from_slice(address.as_bytes());

    // Hash derivation material into a fixed-size ID
    let hash = Hash::new(blake3::hash(&bytes).into());
    ObjectId::new(hash)

}

// -------------------------------------------------------------------------------------------------------------------------- //
