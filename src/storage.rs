use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use serde_json::to_string_pretty;
use crate::models::Human;

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new(path: String) -> Storage {
        Storage {
            path: PathBuf::from(path),
        }
    }

    pub fn save(&self, human: &Human) {
        // Serialize the Human object to JSON
        let serialized = to_string_pretty(&human).expect("Failed to serialize Human");
        let file_path = self.path.join(format!("{}.json", human.name));

        // Write the serialized data to the file
        let mut file = File::create(&file_path).expect("Failed to create file");
        file.write_all(serialized.as_bytes())
            .expect("Failed to write to file");
    }

    pub fn load(&self, name: &str) -> io::Result<Human> {
        let file_path = self.path.join(format!("{}.json", name));
        let file = File::open(file_path)?;
        let human: Human = serde_json::from_reader(file)?;
        Ok(human)
    }
}
