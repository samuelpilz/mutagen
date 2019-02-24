use lazy_static::lazy_static;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref GLOBAL_TRANSFORM_INFO: SharedTransformInfo = Default::default();
}

#[derive(Default)]
pub struct SharedTransformInfo(Arc<Mutex<MutagenTransformInfo>>);

const DEFAULT_MUTAGEN_DIR: &'static str = "target/mutagen";
const DEFAULT_MUTAGEN_FILENAME: &'static str = "mutations.txt";

/// Contains information about all mutations inserted into the code under test
///
/// This struct is used to collect the mutations during transformation. After running all transformers, this struct contains all mutators and their mutaitons inserted into the code
#[derive(Debug)]
pub struct MutagenTransformInfo {
    mutations: Vec<String>,
    mutagen_file: Option<File>,
}

impl Default for MutagenTransformInfo {
    fn default() -> Self {
        Self {
            mutations: vec![],
            mutagen_file: None,
        }
    }
}

impl MutagenTransformInfo {
    pub fn with_default_mutagen_file(&mut self) {
        // open file only once
        if let None = self.mutagen_file {
            let mutagen_dir = Path::new(DEFAULT_MUTAGEN_DIR);
            let mutagen_filepath = mutagen_dir.join(DEFAULT_MUTAGEN_FILENAME);
            if !mutagen_dir.exists() {
                create_dir_all(&mutagen_dir).unwrap();
            }
            let mutagen_file = File::create(mutagen_filepath.clone())
                .expect(&format!("unable to open file {:?}", mutagen_filepath));

            self.mutagen_file = Some(mutagen_file);
        }
    }

    /// add a mutation and return the id used for it, also writes the mutation to the global file.
    pub fn add_mutation(&mut self, mutation: String) -> u32 {
        let mut_id = 1 + self.mutations.len() as u32;

        // write the mutation if file was configured
        if let Some(mutagen_file) = &mut self.mutagen_file {
            let mut mutagen_file = BufWriter::new(mutagen_file);
            writeln!(mutagen_file, "{}: {}", mut_id, &mutation)
                .expect("unable to write to mutagen file");
            mutagen_file.flush().unwrap();
        }

        // add mutation to list
        self.mutations.push(mutation);

        // return next mutation id
        mut_id
    }
}

impl SharedTransformInfo {
    pub fn add_mutation(&self, mutation: String) -> u32 {
        self.0.lock().unwrap().add_mutation(mutation)
    }

    pub fn clone_shared(&self) -> Self {
        Self(Arc::clone(&self.0))
    }

    pub fn with_default_mutagen_file(&self) {
        self.0.lock().unwrap().with_default_mutagen_file()
    }
}

impl fmt::Display for MutagenTransformInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, mutation) in self.mutations.iter().enumerate() {
            write!(f, "{} - {}", i, mutation)?;
        }
        Ok(())
    }
}
