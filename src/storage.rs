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
        let path_buf = PathBuf::from(&path);
        if !path_buf.exists() {
            std::fs::create_dir_all(&path_buf).expect("Failed to create directory");
        }
        Storage {
            path: path_buf,
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

    pub fn load_all(&self) -> io::Result<Vec<Human>> {
        let mut humans = Vec::new();
        for entry in std::fs::read_dir(&self.path)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let human: Human = serde_json::from_reader(File::open(entry.path())?)?;
                humans.push(human);
            }
        }
        Ok(humans)
    }

    pub fn remove(&self, name: &str) -> io::Result<()> {
        let file_path = self.path.join(format!("{}.json", name));
        std::fs::remove_file(file_path)?;
        Ok(())
    }
}
