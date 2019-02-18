//! Mutator for int literals.

use crate::MutagenRuntimeConfig;

pub struct MutatorLitInt<T> {
    mutator_id: u32,
    original_lit: T,
}

impl MutatorLitInt<u32> {
    pub fn new(mutator_id: u32, original_lit: u32) -> Self {
        MutatorLitInt {
            mutator_id,
            original_lit,
        }
    }

    pub fn run_mutator(self, runtime: &MutagenRuntimeConfig) -> u32 {
        if runtime.mutation_id != self.mutator_id {
            self.original_lit
        } else {
            self.original_lit + 1
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::MutagenRuntimeConfig;

    #[test]
    pub fn mutator_lit_int_zero_inactive() {
        let mutator = MutatorLitInt::new(1, 0);
        let result = mutator.run_mutator(&MutagenRuntimeConfig::with_mutation_id(0));
        assert_eq!(result, 0)
    }
    #[test]
    pub fn mutator_lit_int_zero_active() {
        let mutator = MutatorLitInt::new(1, 0);
        let result = mutator.run_mutator(&MutagenRuntimeConfig::with_mutation_id(1));
        assert_eq!(result, 1)
    }
}
