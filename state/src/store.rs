
/// Imports and dependencies
use axiom_types::{ObjectId};
use crate::StateObject;
use crate::Version;
use crate::StateError;

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

    /// Insert or update a state object.
    pub fn insert_or_update(&mut self, object: StateObject) -> Result<(), StateError> {
        self.objects.insert(object.id(), object);
        Ok(())
    }

    /// Iterate over all state objects (read-only).
    pub fn objects_iter(&self) -> impl Iterator<Item = (&ObjectId, &StateObject)> {
        self.objects.iter()
    }
    
}

// -------------------------------------------------------------------------------------------------------------------------- //
