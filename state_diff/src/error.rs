use axiom_types::ObjectId;

#[derive(Debug)]
pub enum CommitError {
    ObjectNotFound { object: ObjectId },
    StaleRead {
        object: ObjectId,
        expected: u64,
        found: u64,
    },
    InvalidWrite { object: ObjectId },
}
