
/// Imports and dependecies
use axiom_types::{Address, ObjectId, Hash};
use crate::{StateObject, StateStore, Version, NonceError};
// -------------------------------------------------------------------------------------------------------------------------- //

//----------------------------------------------- Nonce -----------------------------------------------//

/// Reserved domain for nonce objects.
/// This defines STATE LAYOUT, not a type.
const NONCE_DOMAIN: &[u8] = b"axiom::nonce";

/// Deterministically derive the nonce ObjectId for an address.
///
/// There is exactly ONE nonce object per address.
pub fn nonce_object_id(address: Address) -> ObjectId {
    let mut bytes = Vec::new();

    // domain separation (protocol rule)
    bytes.extend_from_slice(NONCE_DOMAIN);

    // bind to address
    bytes.extend_from_slice(address.as_bytes());

    let hash = Hash::new(blake3::hash(&bytes).into());
    ObjectId::new(hash)
}

/// Validate nonce and prepare nonce object update.
///
/// Nonce is enforced via StateObject versioning.
pub fn validate_and_prepare_nonce_update(
    signer: Address,
    provided_nonce: Version,
    state: &StateStore,
) -> Result<(ObjectId, StateObject), NonceError> {
    let nonce_id = nonce_object_id(signer);

    match state.get(&nonce_id) {
        Some(existing) => {
            // nonce must equal current version
            if provided_nonce != existing.version() {
                return Err(NonceError::InvalidNonce {
                    expected: existing.version(),
                    got: provided_nonce,
                });
            }

            // prepare updated nonce object with version+1
            let updated = existing.next_version();
            Ok((nonce_id, updated))
        }
        None => {
            // First transaction: nonce must be 0
            if provided_nonce != 0 {
                return Err(NonceError::InvalidNonce {
                    expected: 0,
                    got: provided_nonce,
                });
            }

            let obj = StateObject::new(
                nonce_id,
                signer,
                Vec::new(), // no data needed
            );

            Ok((nonce_id, obj))
        }
    }
}