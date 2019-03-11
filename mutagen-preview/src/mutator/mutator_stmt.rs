//! Mutator for statements.

use crate::MutagenRuntimeConfig;

pub struct MutatorStmt {
    mutator_id: u32,
}

impl MutatorStmt {
    pub fn new(mutator_id: u32) -> Self {
        Self { mutator_id }
    }

    pub fn run_mutator(self, runtime: &MutagenRuntimeConfig) -> bool {
        return runtime.mutation_id != self.mutator_id;
    }
}

#[cfg(test)]
mod tests {

    mod mutate_test {

        use crate::mutate;
        use ::mutagen_preview::MutagenRuntimeConfig;


        #[mutate(conf(local), only(stmt))]
        fn add_one() -> u32 {
            let mut x = 1;
            x += 1;
            x
        }
        #[test]
        fn add_one_inactive() {
            MutagenRuntimeConfig::test_with_mutation_id(0, || {
                assert_eq!(add_one(), 2);
            })
        }
        #[test]
        fn add_one_active() {
            MutagenRuntimeConfig::test_with_mutation_id(1, || {
                assert_eq!(add_one(), 1);
            })
        }

        #[mutate(conf(local), only(stmt))]
        fn push_one() -> Vec<u32> {
            let mut v = Vec::new();
            v.push(1);
            v
        }
        #[test]
        fn push_one_inactive() {
            MutagenRuntimeConfig::test_with_mutation_id(0, || {
                assert_eq!(push_one(), vec![1]);
            })
        }
        #[test]
        fn push_one_active() {
            MutagenRuntimeConfig::test_with_mutation_id(1, || {
                assert_eq!(push_one(), vec![]);
            })
        }

        #[mutate(conf(local), only(stmt))]
        fn vec_macro() -> Vec<&'static str> {
            vec!["1"]
        }
        #[test]
        fn vec_macro_inactive() {
            MutagenRuntimeConfig::test_with_mutation_id(0, || {
                assert_eq!(vec_macro(), vec!["1"]);
            })
        }
        #[test]
        fn vec_macro_active() {
            MutagenRuntimeConfig::test_with_mutation_id(1, || {
                assert_eq!(vec_macro(), vec!["1"]);
            })
        }

    }
}
