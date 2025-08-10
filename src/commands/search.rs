use crate::models::{Human, Metric};
use crate::storage::Storage;
use std::collections::HashMap;
use std::io;
use regex::Regex;

/// Normalize an optional pattern: None or empty -> None, else Some(pattern.clone()).
pub fn normalize_pattern(pat: &Option<String>) -> Option<String> {
    match pat.as_deref() {
        None | Some("") => None,
        Some(s) => Some(s.to_string()),
    }
}

/// Convert a simple wildcard pattern to an anchored regex string.
fn wildcard_to_regex(pattern: &str) -> String {
    let mut re = String::with_capacity(pattern.len() * 2 + 2);
    re.push('^');
    for ch in pattern.chars() {
        match ch {
            '*' => re.push_str(".*"),
            '?' => re.push('.'),
            _ => re.push_str(&regex::escape(&ch.to_string())),
        }
    }
    re.push('$');
    re
}

/// Simple wildcard matching supporting '*' (any sequence) and '?' (single char), implemented via regex.
pub fn wildcard_matches(pattern: &str, text: &str) -> bool {
    let re = wildcard_to_regex(pattern);
    match Regex::new(&re) {
        Ok(r) => r.is_match(text),
        Err(_) => false,
    }
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

/// Check if a human matches all query-derived filters (name wildcard, labels, metrics).
pub fn human_matches(h: &Human, name_pat: &Option<String>, required_labels: &[String], min_metrics: &HashMap<String, u8>) -> bool {
    if let Some(pat) = name_pat {
        if !wildcard_matches(pat, &h.name) {
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

/// Return true if description matches provided wildcard pattern (or if not provided).
pub fn description_matches(h: &Human, desc_pat: &Option<String>) -> bool {
    match desc_pat {
        None => true,
        Some(pat) => h
            .description
            .as_deref()
            .map(|d| wildcard_matches(pat, d))
            .unwrap_or(false),
    }
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
    let name_pat = if query.name.is_empty() { None } else { Some(query.name.clone()) };
    let desc_pat = normalize_pattern(&query.description);
    let required_labels = query.label.clone().unwrap_or_default();
    let min_metrics = extract_min_metrics(query);

    let results = all
        .into_iter()
        .filter(|h| human_matches(h, &name_pat, &required_labels, &min_metrics))
        .filter(|h| description_matches(h, &desc_pat))
        .collect();
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::models::humans::test_setup;

    #[test]
    fn search_by_wildcards_labels_and_metrics() {
        let tmp = tempdir().unwrap();
        let storage = Storage::new(tmp.path().to_string_lossy().to_string());
        for h in test_setup().into_iter().filter(|h| matches!(h.name.as_str(), "alice"|"bob"|"alina")) {
            storage.save(&h);
        }

        // Query: name ali*, labels must contain [eng], metrics speed >= 10
        let query = Human {
            id: None,
            name: String::from("ali*"),
            phone: None,
            description: None,
            label: Some(vec!["eng".into()]),
            metric: Some(vec![Metric { name: "speed".into(), value: 10 }]),
        };

        let results = run(&storage, &query).unwrap();
        let names: Vec<String> = results.into_iter().map(|h| h.name).collect();
        // alice (speed=10, eng,oncall) and alina (speed=11, eng) match; bob doesn't.
        assert_eq!(names, vec!["alice", "alina"]);
    }

    #[test]
    fn search_by_description_wildcard() {
        let tmp = tempdir().unwrap();
        let storage = Storage::new(tmp.path().to_string_lossy().to_string());
        for h in test_setup().into_iter().filter(|h| matches!(h.name.as_str(), "alice"|"alina")) {
            storage.save(&h);
        }

        let query = Human {
            id: None,
            name: String::from("*"),
            phone: None,
            description: Some("*lead".into()),
            label: None,
            metric: None,
        };

        let results = run(&storage, &query).unwrap();
        let names: Vec<String> = results.into_iter().map(|h| h.name).collect();
        assert_eq!(names, vec!["alice"]);
    }
}
