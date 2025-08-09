use crate::models::{Human, Metric};
use crate::storage::Storage;
use regex::Regex;
use std::collections::HashMap;
use std::io;

/// Build an optional regex from pattern; empty string -> None.
pub fn build_name_regex(pattern: &str) -> io::Result<Option<Regex>> {
    if pattern.is_empty() {
        return Ok(None);
    }
    Regex::new(pattern)
        .map(Some)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

/// Return true if candidate contains all required labels.
pub fn labels_match(candidate: &Human, required: &[String]) -> bool {
    if required.is_empty() {
        return true;
    }
    let have = candidate.label.clone().unwrap_or_default();
    required.iter().all(|r| have.iter().any(|h| h == r))
}

/// Return true if candidate metrics meet or exceed each required threshold.
pub fn metrics_meet(candidate: &Human, minimums: &HashMap<String, u8>) -> bool {
    if minimums.is_empty() {
        return true;
    }
    let have: HashMap<String, u8> = candidate
        .metric
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|Metric { name, value }| (name, value))
        .collect();
    minimums.iter().all(|(k, min_v)| have.get(k).map_or(false, |v| v >= min_v))
}

/// Check if a human matches all query-derived filters.
pub fn human_matches(h: &Human, name_re: &Option<Regex>, required_labels: &[String], min_metrics: &HashMap<String, u8>) -> bool {
    if let Some(re) = name_re {
        if !re.is_match(&h.name) {
            return false;
        }
    }
    if !labels_match(h, required_labels) {
        return false;
    }
    if !metrics_meet(h, min_metrics) {
        return false;
    }
    true
}

/// Extract thresholds map from a query Human's metrics.
pub fn extract_min_metrics(query: &Human) -> HashMap<String, u8> {
    query
        .metric
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|Metric { name, value }| (name, value))
        .collect()
}

/// Search: load all humans and filter according to query.
pub fn run(storage: &Storage, query: &Human) -> io::Result<Vec<Human>> {
    let all = storage.load_all()?;
    let name_regex = build_name_regex(&query.name)?;
    let required_labels = query.label.clone().unwrap_or_default();
    let min_metrics = extract_min_metrics(query);

    let results = all
        .into_iter()
        .filter(|h| human_matches(h, &name_regex, &required_labels, &min_metrics))
        .collect();
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn mk_human(name: &str, labels: &[&str], metrics: &[(&str, u8)]) -> Human {
        Human {
            id: None,
            name: name.to_string(),
            phone: None,
            label: if labels.is_empty() {
                None
            } else {
                Some(labels.iter().map(|s| s.to_string()).collect())
            },
            metric: if metrics.is_empty() {
                None
            } else {
                Some(
                    metrics
                        .iter()
                        .map(|(n, v)| Metric { name: (*n).into(), value: *v })
                        .collect(),
                )
            },
        }
    }

    #[test]
    fn search_by_regex_labels_and_metrics() {
        let tmp = tempdir().unwrap();
        let storage = Storage::new(tmp.path().to_string_lossy().to_string());

        let a = mk_human("alice", &["eng", "oncall"], &[("speed", 10), ("height", 20)]);
        let b = mk_human("bob", &["sales"], &[("speed", 9)]);
        let c = mk_human("alina", &["eng"], &[("speed", 11), ("height", 20)]);

        storage.save(&a);
        storage.save(&b);
        storage.save(&c);

        // Query: name /ali.*/, labels must contain [eng], metrics speed >= 10
        let query = Human {
            id: None,
            name: String::from("^ali"),
            phone: None,
            label: Some(vec!["eng".into()]),
            metric: Some(vec![Metric { name: "speed".into(), value: 10 }]),
        };

        let results = run(&storage, &query).unwrap();
        let names: Vec<String> = results.into_iter().map(|h| h.name).collect();
        // alice (speed=10, eng,oncall) and alina (speed=11, eng) match; bob doesn't.
        assert_eq!(names, vec!["alice", "alina"]);
    }
}
