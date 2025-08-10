use crate::models::Human;
use crate::storage::Storage;
use polars::prelude::*;
use std::io::{self, Write};

fn humans_to_dataframe(humans: &[Human]) -> PolarsResult<DataFrame> {
    // Serialize each Human as a JSON object on its own line (JSON Lines / NDJSON)
    let mut buf = String::with_capacity(humans.len() * 128);
    for h in humans {
        let line = serde_json::to_string(h)
            .map_err(|e| PolarsError::ComputeError(format!("serde error: {}", e).into()))?;
        buf.push_str(&line);
        buf.push('\n');
    }

    let cursor = std::io::Cursor::new(buf);
    JsonReader::new(cursor).finish()
}

pub fn run(storage: &Storage, output: Option<&str>) -> io::Result<()> {
    let humans = storage.load_all()?;
    let df = humans_to_dataframe(&humans).map_err(to_io_err)?;

    // Write CSV either to file or stdout
    match output {
        Some(path) => {
            let mut file = std::fs::File::create(path)?;
            let mut writer = CsvWriter::new(&mut file);
            let mut df = df;
            writer.finish(&mut df).map_err(to_io_err)?;
        }
        None => {
            let mut out = io::stdout();
            let mut writer = CsvWriter::new(&mut out);
            let mut df = df;
            writer.finish(&mut df).map_err(to_io_err)?;
            out.flush()?;
        }
    }
    Ok(())
}

fn to_io_err<E: std::error::Error + Send + Sync + 'static>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Metric;
    use tempfile::tempdir;

    #[test]
    fn export_to_csv_stdout() {
        let tmp = tempdir().unwrap();
        let storage = Storage::new(tmp.path().to_string_lossy().to_string());

        let h = Human {
            id: Some("1".into()),
            name: "alice".into(),
            phone: Some("555".into()),
            description: Some("team lead".into()),
            label: Some(vec!["eng".into(), "oncall".into()]),
            metric: Some(vec![Metric { name: "speed".into(), value: 10 }]),
        };
        storage.save(&h);

        // Create a dataframe to ensure no panics in conversion
        let df = humans_to_dataframe(&storage.load_all().unwrap()).unwrap();
        assert!(df.get_column_names().contains(&"name".into()));
    }
}
