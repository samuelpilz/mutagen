use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;

lazy_static! {
    static ref RUNTIME_CONFIG: Mutex<Option<MutagenRuntimeConfig>> = Mutex::new(None);
}

#[derive(Copy, Clone)]
pub struct MutagenRuntimeConfig {
    pub mutation_id: u32,
}

impl MutagenRuntimeConfig {
    /// access the currently active runtime-config based on the environment variable `MUATION_ID`
    pub fn get_default() -> Self {
        let mut lock_guard = RUNTIME_CONFIG.lock().unwrap();
        match &*lock_guard {
            None => {
                // runtime config not initialized -> set default config based on env-var
                let env_config = MutagenRuntimeConfig::from_env();
                *lock_guard = Some(env_config);
                env_config
            }
            Some(config) => *config,
        }
    }

    fn from_env() -> Self {
        let mutation_id = env::var("MUTATION_ID")
            .map(|s| s.parse().unwrap_or(0))
            .unwrap_or(0);
        MutagenRuntimeConfig { mutation_id }
    }

    #[cfg(any(test, feature = "self-test"))]
    pub fn with_mutation_id(mutation_id: u32) -> Self {
        MutagenRuntimeConfig { mutation_id }
    }

    #[cfg(any(test, feature = "self-test"))]
    pub fn set_test_config(mutation_id: u32) {
        *RUNTIME_CONFIG.lock().unwrap() = Some(MutagenRuntimeConfig::with_mutation_id(mutation_id));
    }

    #[cfg(any(test, feature = "self-test"))]
    pub fn clear_test_config() {
        *RUNTIME_CONFIG.lock().unwrap() = None;
    }
}
