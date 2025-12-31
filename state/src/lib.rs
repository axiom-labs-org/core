//! Axiom state model.
//!
//! This crate defines the object-based state abstraction used by the Axiom
//! protocol. All on-chain state is represented as isolated, versioned objects.

// Using types defined in axiom-types
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

//----------------------------------------------- State Store -----------------------------------------------//

/// In-memory state store for Axiom.
///
/// This module provides a deterministic, version-aware state store backed
/// by an in-memory HashMap. It enforces object isolation, version monotonicity,
/// and atomic updates.

use std::collections::HashMap;

/// A write set containing proposed updates to state objects.
pub type WriteSet = HashMap<ObjectId, StateObject>;

/// A read set containing versions of state objects read during a transaction.
pub type ReadSet = HashMap<ObjectId, Version>;

/// In-Memory state store.
#[derive(Default)]
pub struct StateStore {
    objects: HashMap<ObjectId, StateObject>,
}

impl StateStore {
    /// Create a new empty state store.
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    /// Retrieve a state object by its identifier.
    pub fn get(&self, id: &ObjectId) -> Option<&StateObject> {
        self.objects.get(id)
    }

    /// Insert a new state object
    /// Fails if the object already exists
    pub fn insert(&mut self, object: StateObject) -> Result<(), StateError> {
        if self.objects.contains_key(&object.id()) {
            return Err(StateError::ObjectAlreadyExists);
        }
        self.objects.insert(object.id(), object);
        Ok(())
    }

    /// Apply a write set to the state store atomically.
    /// All updates must be valid; otherwise, no changes are made.
    pub fn apply(&mut self, read_set: &ReadSet, write_set: WriteSet) -> Result<(), StateError> {
        // Validate read set
        for (id, expected_version) in read_set {
            match self.objects.get(id) {
                Some(existing) => {
                    if existing.version() != *expected_version {
                        return Err(StateError::StaleRead {
                            expected: *expected_version,
                            found: existing.version(),
                        });
                    }
                }
                None => {
                    return Err(StateError::ObjectNotFound);
                }
            }
        }
        
        // Validate write set
        for (id, new_object) in &write_set {
            match self.objects.get(id) {
                Some(existing_object) => {
                    if new_object.version() != existing_object.version() + 1 {
                        return Err(StateError::InvalidVersion {
                            expected: existing_object.version() + 1,
                            found: new_object.version(),
                        });
                    }
                }
                None => {
                    // If the object doesn't exist, it's a new object
                    if new_object.version() != 0 {
                        return Err(StateError::InvalidVersion {
                            expected: 0,
                            found: new_object.version(),
                        });
                    }
                }
            }
        }

        // Apply all updates
        for (id, new_object) in write_set {
            self.objects.insert(id, new_object);
        }

        Ok(())
    }   
}

/// Errors returned by the state store.
#[derive(Debug)]
pub enum StateError {
    ObjectAlreadyExists,
    ObjectNotFound,
    StaleRead { expected: u64, found: u64 },
    InvalidVersion { expected: u64, found: u64 },
}

