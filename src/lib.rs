//! # Flydent
//!
//! A Rust library for parsing aircraft registration callsigns and ICAO 24-bit identifiers.
//! This is a port of the Python flydenity library that identifies countries and organizations
//! from aircraft callsigns using ITU data.
//!
//! ## Features
//!
//! - Parse aircraft callsigns (e.g., "T6ABC" -> Afghanistan)
//! - Parse ICAO 24-bit identifiers (e.g., "700123" -> Afghanistan)
//! - Identify countries with ISO codes and organizations
//! - Compile-time CSV parsing for zero-runtime overhead
//! - No external CSV files required at runtime
//!
//! ## Usage
//!
//! ```rust
//! use flydent::{Parser, EntityResult};
//!
//! let parser = Parser::new();
//!
//! // Parse a callsign
//! if let Some(result) = parser.parse_simple("T6ABC") {
//!     match result {
//!         EntityResult::Country { nation, iso2, .. } => {
//!             println!("Country: {} ({})", nation, iso2);
//!         }
//!         EntityResult::Organization { name, .. } => {
//!             println!("Organization: {}", name);
//!         }
//!     }
//! }
//!
//! // Parse ICAO 24-bit identifier
//! if let Some(result) = parser.parse("700123", false, true) {
//!     println!("ICAO identifier parsed: {:?}", result);
//! }
//! ```

use regex::Regex;
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub enum EntityResult {
    Country {
        nation: String,
        description: String,
        iso2: String,
        iso3: String,
    },
    Organization {
        name: String,
        description: String,
    },
}

#[derive(Debug, Clone)]
struct EntityData {
    entity_result: EntityResult,
    priority: i32,
    callsigns: Vec<String>,
    regex: String,
    strict_regex: String,
    icao24bit_prefixes: Vec<String>,
}

fn parse_python_list(s: &str) -> Vec<String> {
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len()-1];
        if inner.is_empty() {
            Vec::new()
        } else {
            inner.split(", ")
                .map(|item| {
                    let item = item.trim();
                    if (item.starts_with('\'') && item.ends_with('\'')) || (item.starts_with('"') && item.ends_with('"')) {
                        item[1..item.len()-1].to_string()
                    } else {
                        item.to_string()
                    }
                })
                .collect()
        }
    } else {
        Vec::new()
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    // Escaped quote
                    current_field.push('"');
                    chars.next();
                } else {
                    in_quotes = !in_quotes;
                }
            }
            ',' if !in_quotes => {
                fields.push(current_field.trim().to_string());
                current_field.clear();
            }
            _ => current_field.push(ch),
        }
    }
    fields.push(current_field.trim().to_string());
    fields
}

macro_rules! build_data {
    () => {{
        let mut all_data = Vec::new();

        // Parse countries
        let countries_csv = include_str!("../data/processed_itu_countries_regex.csv");
        let mut lines = countries_csv.lines();
        let _header = lines.next().unwrap(); // Skip header

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            let fields = parse_csv_line(line);
            if fields.len() >= 10 {
                let nation = fields[0].clone();
                let description = fields[1].clone();
                let priority: i32 = fields[2].parse().unwrap_or(0);
                let iso_codes = parse_python_list(&fields[3]);
                let callsigns = parse_python_list(&fields[4]);
                let regex_str = fields[6].clone();
                let icao24bit_prefixes = parse_python_list(&fields[9]);

                let iso2 = iso_codes.get(0).cloned().unwrap_or_default();
                let iso3 = iso_codes.get(1).cloned().unwrap_or_default();

                let strict_regex_str = regex_str.replace("-{0,1}", "\\-").replace("{0,1}$", "$");

                all_data.push(EntityData {
                    entity_result: EntityResult::Country {
                        nation,
                        description,
                        iso2,
                        iso3,
                    },
                    priority,
                    callsigns,
                    regex: regex_str,
                    strict_regex: strict_regex_str,
                    icao24bit_prefixes,
                });
            }
        }

        // Parse organizations
        let orgs_csv = include_str!("../data/processed_itu_organizations_regex.csv");
        let mut lines = orgs_csv.lines();
        let _header = lines.next().unwrap(); // Skip header

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            let fields = parse_csv_line(line);
            if fields.len() >= 9 {
                let name = fields[0].clone();
                let description = fields[1].clone();
                let priority: i32 = fields[2].parse().unwrap_or(0);
                let callsigns = parse_python_list(&fields[3]);
                let regex_str = fields[5].clone();
                let icao24bit_prefixes = parse_python_list(&fields[8]);

                let strict_regex_str = regex_str.replace("-{0,1}", "\\-").replace("{0,1}$", "$");

                all_data.push(EntityData {
                    entity_result: EntityResult::Organization {
                        name,
                        description,
                    },
                    priority,
                    callsigns,
                    regex: regex_str,
                    strict_regex: strict_regex_str,
                    icao24bit_prefixes,
                });
            }
        }

        all_data
    }};
}

static DATA: Lazy<Vec<EntityData>> = Lazy::new(|| build_data!());

static CALLSIGNS_MAP: Lazy<HashMap<String, Vec<usize>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (i, data) in DATA.iter().enumerate() {
        for callsign in &data.callsigns {
            map.entry(callsign.clone()).or_insert_with(Vec::new).push(i);
        }
    }
    map
});

static ICAO_MAP: Lazy<HashMap<String, usize>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (i, data) in DATA.iter().enumerate() {
        for prefix in &data.icao24bit_prefixes {
            map.insert(prefix.clone(), i);
        }
    }
    map
});

static MIN_CALLSIGN_LEN: Lazy<usize> = Lazy::new(|| {
    CALLSIGNS_MAP.keys().map(|k| k.len()).min().unwrap_or(0)
});

static MAX_CALLSIGN_LEN: Lazy<usize> = Lazy::new(|| {
    CALLSIGNS_MAP.keys().map(|k| k.len()).max().unwrap_or(0)
});

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    fn parse_registration(&self, input: &str, strict: bool) -> Option<Vec<&EntityData>> {
        let mut datasets = Vec::new();

        for callsign_len in *MIN_CALLSIGN_LEN..=*MAX_CALLSIGN_LEN {
            if input.len() >= callsign_len {
                let prefix = &input[0..callsign_len];
                if let Some(indices) = CALLSIGNS_MAP.get(prefix) {
                    for &idx in indices {
                        datasets.push(&DATA[idx]);
                    }
                }
            }
        }

        if datasets.is_empty() {
            return None;
        }

        let mut matches_by_priority: HashMap<i32, Vec<&EntityData>> = HashMap::new();

        for data in datasets {
            let regex_str = if strict { &data.strict_regex } else { &data.regex };

            if let Ok(regex) = Regex::new(regex_str) {
                if regex.is_match(input) {
                    matches_by_priority.entry(data.priority).or_default().push(data);
                }
            }
        }

        if let Some(max_priority) = matches_by_priority.keys().max() {
            matches_by_priority.get(max_priority).cloned()
        } else {
            None
        }
    }

    fn parse_icao24bit(&self, input: &str, strict: bool) -> Option<Vec<&EntityData>> {
        if strict && !Regex::new(r"^[0-9A-F]{6}$").unwrap().is_match(input) {
            eprintln!("Warning: ICAO 24bit '{}' must be hexadecimal with length of 6 chars", input);
            return None;
        }

        let mut matches = Vec::new();

        for i in 0..input.len() {
            let prefix = &input[0..=i];
            if let Some(&idx) = ICAO_MAP.get(prefix) {
                matches.push(&DATA[idx]);
            }
        }

        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }

    pub fn parse(&self, input: &str, strict: bool, icao24bit: bool) -> Option<EntityResult> {
        if icao24bit {
            if let Some(matches) = self.parse_icao24bit(input, strict) {
                matches.first().map(|data| data.entity_result.clone())
            } else {
                None
            }
        } else if let Some(matches) = self.parse_registration(input, strict) {
            matches.first().map(|data| data.entity_result.clone())
        } else {
            None
        }
    }

    pub fn parse_simple(&self, input: &str) -> Option<EntityResult> {
        self.parse(input, false, false)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let _parser = Parser::new();
        assert!(*MIN_CALLSIGN_LEN > 0);
        assert!(*MAX_CALLSIGN_LEN >= *MIN_CALLSIGN_LEN);
        assert!(!DATA.is_empty());
        assert!(!CALLSIGNS_MAP.is_empty());
    }

    #[test]
    fn test_parse_csv_line() {
        let line = r#"Afghanistan,general,0,"['AF', 'AFG']","['T6', 'YA']",['AAA-ZZZ'],"^(T6|YA)(-{0,1}([A-Z]{3}|[A-Z0-9]{1,4})){0,1}$",700000,700FFF,['700']"#;
        let fields = parse_csv_line(line);
        assert_eq!(fields[0], "Afghanistan");
        assert_eq!(fields[3], "['AF', 'AFG']");
        assert_eq!(fields[4], "['T6', 'YA']");
    }

    #[test]
    fn test_parse_python_list() {
        let result = parse_python_list("['T6', 'YA']");
        assert_eq!(result, vec!["T6", "YA"]);

        let result = parse_python_list("['700']");
        assert_eq!(result, vec!["700"]);
    }

    #[test]
    fn test_parse_simple() {
        let parser = Parser::new();

        // Test with a known callsign prefix
        if let Some(result) = parser.parse_simple("T6ABC") {
            match result {
                EntityResult::Country { nation, .. } => {
                    assert_eq!(nation, "Afghanistan");
                }
                _ => panic!("Expected country result for T6ABC"),
            }
        } else {
            panic!("T6ABC should match Afghanistan");
        }
    }

    #[test]
    fn test_comprehensive_parsing() {
        let parser = Parser::new();

        // Test Afghanistan callsign T6ABC
        if let Some(result) = parser.parse("T6ABC", false, false) {
            match result {
                EntityResult::Country { nation, description, iso2, iso3 } => {
                    assert_eq!(nation, "Afghanistan");
                    assert_eq!(description, "general");
                    assert_eq!(iso2, "AF");
                    assert_eq!(iso3, "AFG");
                }
                _ => panic!("Expected country result for T6ABC"),
            }
        } else {
            panic!("T6ABC should match Afghanistan");
        }

        // Test organization callsign 4Y123
        if let Some(result) = parser.parse("4Y123", false, false) {
            match result {
                EntityResult::Organization { name, description } => {
                    assert_eq!(name, "International Civil Aviation Organization");
                    assert_eq!(description, "general");
                }
                _ => panic!("Expected organization result for 4Y123"),
            }
        } else {
            panic!("4Y123 should match ICAO");
        }

        // Test ICAO 24-bit identifier 700123
        if let Some(result) = parser.parse("700123", false, true) {
            match result {
                EntityResult::Country { nation, description, iso2, iso3 } => {
                    assert_eq!(nation, "Afghanistan");
                    assert_eq!(description, "general");
                    assert_eq!(iso2, "AF");
                    assert_eq!(iso3, "AFG");
                }
                _ => panic!("Expected country result for ICAO 700123"),
            }
        } else {
            panic!("ICAO 700123 should match Afghanistan");
        }

        // Test non-existent callsign should return None
        assert!(parser.parse("N123ABC", false, false).is_none());
    }
}
