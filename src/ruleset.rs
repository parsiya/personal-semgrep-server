// ----- START RuleSet

use std::{fs, io};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSet {
    pub name: String,
    pub rules: Vec<String>
}

impl RuleSet {

    pub fn from_yaml(yaml: String) -> serde_yaml::Result<RuleSet> {
        Ok(serde_yaml::from_str::<RuleSet>(&yaml)?)
    }

    pub fn from_file(file: String) -> serde_yaml::Result<RuleSet> {
        // ZZZ add error handling.
        let yaml = read_file_to_string(file.as_str()).unwrap();
        Ok(serde_yaml::from_str::<RuleSet>(&yaml)?)
    }

    pub fn find_all(path: String, include: Option<Vec<&str>>, exclude: Option<Vec<&str>>) -> Vec<String> {
        let rulesets = semgrep_rs::find_rules(path, include, exclude);
        rulesets
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

// read a file and return a String.
pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}