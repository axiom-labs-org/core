
/// Imports and dependencies
use axiom_types::{Address, ObjectId};

// -------------------------------------------------------------------------------------------------------------------------- //

//----------------------------------------------- State Object -----------------------------------------------//

/// Version number of a state object.
pub type Version = u64;

/// A state object.
///
/// A `StateObject` represents the smallest unit of mutable on-chain state.
/// Objects are isolated, versioned, and owned by a single address.
#[derive(Clone, Debug)]
pub struct StateObject {
    id: ObjectId,
    owner: Address,
    version: Version,
    data: Vec<u8>,
}

impl StateObject {
    /// Create new state object
    /// New state object is always created with version 0
    pub fn new(id: ObjectId, owner: Address, data: Vec<u8>) -> Self {
        Self {
            id,
            owner,
            version: 0,
            data,
        }
    }

    // Return the object identifier
    pub fn id(&self) -> ObjectId {
        self.id
    }

    // Return the owner address
    pub fn owner(&self) -> Address {
        self.owner
    }

    // Return the version number
    pub fn version(&self) -> Version {
        self.version
    }

    // Produce a new version of the object with incremented version number
    pub fn next_version(&self) -> Self {
        Self {
            id: self.id,
            owner: self.owner,
            version: self.version + 1,
            data: self.data.clone(),
        }
    }

    // Return a reference to the object data
    pub fn data(&self) -> &[u8] {
        &self.data  
    }

    // Produce a new version of the object with updated data
    // The original object remains unchanged
    pub fn with_data(self, new_data: Vec<u8>) -> Self {
        Self {
            id: self.id,
            owner: self.owner,
            version: self.version + 1,
            data: new_data,
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------- //
