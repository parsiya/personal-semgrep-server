// ----- START RuleSet

use semgrep_rs::{find_simple, find, read_file_to_string};
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
        // ZZZ add error handling for reading the file.
        let yaml = read_file_to_string(file.as_str()).unwrap();
        Ok(serde_yaml::from_str::<RuleSet>(&yaml)?)
    }

    // find all yaml files in a path, assuming all rulesets end in .yaml or .yml.
    pub fn find_all(path: String, include: Option<Vec<&str>>, exclude: Option<Vec<&str>>) -> Vec<String> {
        let rulesets = find(path, include, exclude);
        rulesets
    }

    pub fn find_all_simple(path: String) -> Vec<String> {
        let rulesets = find_simple(path);
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

