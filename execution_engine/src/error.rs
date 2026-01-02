use axiom_types::ObjectId;

#[derive(Debug)]
pub enum ExecutionError {
    /// Execution attempted to read an undeclared object.
    UnauthorizedRead { object: ObjectId },

    /// Execution attempted to write an undeclared object.
    UnauthorizedWrite { object: ObjectId },

    /// Execution logic failed deterministically.
    ExecutionFailed { reason: String },
}
