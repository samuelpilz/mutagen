//! Mutator for int literals.

use crate::MutagenRuntimeConfig;

pub struct MutatorStmt {
    mutator_id: u32,
}

impl MutatorStmt {
    pub fn new(mutator_id: u32) -> Self {
        MutatorStmt { mutator_id }
    }

    pub fn run_mutator(self, runtime: &MutagenRuntimeConfig) -> bool {
        return runtime.mutation_id != self.mutator_id;
    }
}
