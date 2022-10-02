use std::env;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

const RULE_EXTENSION: [&str; 2] = ["yml", "yaml"];

fn main() {

    let args: Vec<String> = env::args().collect();

    // this will panic if nothing is passed.
    let registry_path = &args[1];

    println!("Registry path: {}", registry_path);

    let reg_path = Path::new(registry_path);

    // check if path exists.
    if !Path::exists(reg_path) {
        // return with an error.
        panic!("{} does not exist.", registry_path);
    }

    // check if path is a directory. Technically, we could have just done this
    // check but we wouldn't know if the path existed vs. is not a directory.
    if !Path::is_dir(reg_path) {
        // return with an error.
        panic!("{} is not a directory.", registry_path);
    }

    // now we can recursively read all files in the path.
    let rule_files = read_rules(registry_path.to_string());

    println!("{:?}", rule_files);

}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn read_rules(path: String) -> Vec<String> {

    let mut results: Vec<String> = Vec::new();

    let walker = WalkDir::new(path).into_iter();

    // ignore errors and skip hidden files/directories
    for entry in walker.filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()) {

        let file_path = entry.path();

        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                if RULE_EXTENSION.contains(&ext_str) {
                    // println!("{:?} is a rule.", file_path.as_os_str());
                    results.push(file_path.to_string_lossy().as_ref().to_string());
                }
            }
        }
    }
    results

}

