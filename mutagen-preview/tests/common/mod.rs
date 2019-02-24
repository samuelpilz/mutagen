use lazy_static::lazy_static;
use mutagen_preview::MutagenRuntimeConfig;
use std::sync::RwLock;

mod features_error;

lazy_static! {
    static ref TEST_LOCK: RwLock<()> = RwLock::new(());
}

/// sets the global `mutation_id` correctly before running the test and runs tests sequentially.
///
/// The lock is required to ensure that set `mutation_id` is valid for the complete duration of the test case.
pub fn test_with_mutation_id<F: FnOnce() -> ()>(mutation_id: u32, testcase: F) {
    let lock_result = TEST_LOCK.write();
    MutagenRuntimeConfig::set_test_config(mutation_id);
    testcase();
    drop(lock_result);
}
