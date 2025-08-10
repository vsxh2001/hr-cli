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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::humans::test_setup;
    use tempfile::tempdir;

    #[test]
    fn save_load_remove_roundtrip() {
        let tmp = tempdir().expect("tempdir");
        let storage = Storage::new(tmp.path().to_string_lossy().to_string());

    let human = test_setup().into_iter().find(|h| h.name == "Jane").unwrap();

        storage.save(&human);

        // load by name
        let loaded = storage.load("Jane").expect("load");
        assert_eq!(loaded.name, "Jane");
        assert_eq!(loaded.id.as_deref(), Some("123"));
        assert_eq!(loaded.phone.as_deref(), Some("555-0100"));
    assert_eq!(loaded.description.as_deref(), Some("A description"));
        assert_eq!(loaded.label.as_ref().unwrap().len(), 2);
        assert_eq!(loaded.metric.as_ref().unwrap().len(), 2);

        // load_all contains Jane
    let all = storage.load_all().expect("load_all");
    assert!(all.iter().any(|h| h.name == "Jane"));

        // remove and ensure gone
        storage.remove("Jane").expect("remove");
        let err = storage.load("Jane").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
