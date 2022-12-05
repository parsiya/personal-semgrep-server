// ----- START RuleSet

use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSet {
    pub name: String,
    pub rules: Vec<String>,
}

impl RuleSet {
    // create a RuleSet from a YAML string.
    pub fn from_yaml(yaml: String) -> semgrep_rs::Result<RuleSet> {
        match serde_yaml::from_str::<RuleSet>(&yaml) {
            Ok(rs) => Ok(rs),
            Err(e) => semgrep_rs::Error::wrap_string(e.to_string()),
        }
    }

    // create a RuleSet from a file.
    pub fn from_file(file: String) -> semgrep_rs::Result<RuleSet> {
        match semgrep_rs::utils::read_file_to_string(file.as_str()) {
            Err(e) => return semgrep_rs::Error::wrap_string(e.to_string()),
            Ok(str) => match serde_yaml::from_str::<RuleSet>(&str) {
                Err(e) => return semgrep_rs::Error::wrap_string(e.to_string()),
                Ok(rs) => Ok(rs),
            },
        }
    }

    // find all files with extensions in include but not in exclude in a path.
    pub fn find_all(
        path: String,
        include: Option<Vec<&str>>,
        exclude: Option<Vec<&str>>,
    ) -> Vec<String> {
        semgrep_rs::find(path, include, exclude)
    }

    pub fn find_all_simple(path: String) -> Vec<String> {
        semgrep_rs::find_simple(path)
    }

    // serialize the RuleSet as a YAML string.
    pub fn to_yaml(&self) -> semgrep_rs::Result<String> {
        match serde_yaml::to_string(&self) {
            Err(e) => semgrep_rs::Error::wrap_string(e.to_string()),
            Ok(yaml) => Ok(yaml),
        }
    }

    // write the ruleset to a YAML file.
    pub fn to_file(&self, path: String) -> io::Result<()> {
        match self.to_yaml() {
            Err(e) => Err::<(), io::Error>(io::Error::new(io::ErrorKind::InvalidData, e)),
            Ok(yaml) => semgrep_rs::utils::write_string_to_file(path, yaml),
        }
    }

    // // read all rulesets in a specific path, serialize them and return a vector.
    // pub fn from_path(path: String, include: Option<Vec<&str>>, exclude: Option<Vec<&str>>) -> Vec<RuleSet> {
    //     let ruleset_paths = semgrep_rs::find_rules(path, include, exclude);

    //     let mut rulesets: Vec<RuleSet> = Vec::new();

    //     for path in ruleset_paths {
    //         // deserialize each one
    //         let rs = match RuleSet::from
    //     }
    // }
}

// ----- END RuleSet
