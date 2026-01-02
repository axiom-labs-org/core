use axiom_types::{Address, ObjectId, Hash};
use crate::{StateObject};

const BALANCE_DOMAIN: &[u8] = b"axiom::balance";

pub fn balance_object_id(address: Address) -> ObjectId {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(BALANCE_DOMAIN);
    bytes.extend_from_slice(address.as_bytes());

    let hash = Hash::new(blake3::hash(&bytes).into());
    ObjectId::new(hash)
}

/// Decode balance from state object data
pub fn decode_balance(obj: &StateObject) -> u64 {
    let bytes = obj.data();
    u64::from_le_bytes(bytes.try_into().unwrap())
}

/// Encode balance into state object data
pub fn encode_balance(balance: u64) -> Vec<u8> {
    balance.to_le_bytes().to_vec()
}
