use std::{env, collections::HashMap, io, fs};

use semgrep_rs::{utils::check_path_panic, RuleIndex, semgrep_rule::RuleFile};

use crate::ruleset::RuleSet;

mod ruleset;


// read a file and return a String.
fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn main() {

    let args: Vec<String> = env::args().collect();

    // this will panic if nothing is passed.
    let registry_path = &args[1];
    println!("Registry path: {}", registry_path);

    // check the path.
    check_path_panic(registry_path);

    // create the rule index.

    let rule_index: RuleIndex = RuleIndex::from_path(registry_path.to_string(), None, None);

    // second argument is the path to rulsets.
    let ruleset_path = &args[2];
    println!("Ruleset path: {}", ruleset_path);

    // find all rulesets
    let set_paths = RuleSet::find_all(ruleset_path.to_string(), None, None);
    // deserialize them

    let mut rulesets: HashMap<String, RuleFile> = HashMap::new();

    for p in set_paths {
        // read the file
        if let Ok(yaml) = read_file_to_string(p.as_str()) {
            // deserialize it.
            if let Ok(rs) = RuleSet::from_yaml(yaml) {
                // create a ruleset from rule IDs.
                let rf = rule_index.create_ruleset(rs.rules);
                // add it to the set.
                rulesets.insert(rs.name, rf);
            }
        }
    }

    // print all rulesets
    for key in rulesets.keys() {
        println!("ruleset name: {}", key);
        // get the rules IDs in the ruleset.
        for ru in rulesets[key].rules.clone() {
            println!("{}", ru.id);
        }
        println!("-----");
    }

    

    // now we can recursively read all files in the path.
    // let rule_files = read_rules(registry_path.to_string());

    // println!("{:?}", rule_files);

}

