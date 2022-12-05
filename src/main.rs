use std::{collections::HashMap, env};

use semgrep_rs::{check_path_panic, utils::read_file_to_string};
use semgrep_rs::{GenericRuleExt, GenericRuleFile, GenericRuleIndex};

use log::info;

mod ruleset;
use ruleset::RuleSet;

mod constants;
mod diag;

fn main() {
    log4rs::init_file("logging.yaml", Default::default()).unwrap();

    let args: Vec<String> = env::args().collect();

    // this will panic if nothing is passed.
    let registry_path = &args[1];

    info!("Registry path: {}", registry_path);

    // check the path.
    check_path_panic(registry_path);

    // create a simple rule index.
    let generic_rule_index: GenericRuleIndex =
        GenericRuleIndex::from_path_simple(registry_path.to_string());

    // create a rule index with complete paths.
    // let generic_rule_index: GenericRuleIndex =
    //     GenericRuleIndex::from_path(registry_path.to_string(), None, None, true);

    diag::print_vector(generic_rule_index.get_ids());

    // // second argument is the path to rulsets.
    // let ruleset_path = &args[2];
    // info!("Ruleset path: {}", ruleset_path);

    // // find all rulesets
    // let set_paths = RuleSet::find_all_simple(ruleset_path.to_string());

    // let mut rulesets: HashMap<String, GenericRuleFile> = HashMap::new();

    // // create an index of rulesets.
    // for p in set_paths {
    //     // read the file
    //     if let Ok(yaml) = read_file_to_string(p.as_str()) {
    //         // deserialize it.
    //         if let Ok(rs) = RuleSet::from_yaml(yaml) {
    //             // create a ruleset from rule IDs.
    //             let rf = generic_rule_index.create_ruleset(rs.rules);
    //             // add it to the set.
    //             rulesets.insert(rs.name, rf);
    //         }
    //     }
    // }

    // // print all rulesets
    // for key in rulesets.keys() {
    //     info!("ruleset name: {}", key);
    //     // get the rules IDs in the ruleset.
    //     for ru in rulesets[key].rules.clone() {
    //         info!("{}", ru.get_id() );
    //     }
    //     info!("-----");
    // }
    // create a ruleset file.
    // for (id, rule_file) in rulesets {
    //     // serialize to YAML and print to output.
    //     match rule_file.to_string() {
    //         Ok(s) => info!("{}", s),
    //         Err(e) => info!("{}", e.to_string()),
    //     }
    //     info!("\n")
    // }

    // // read the 3rd parameter to write the ruleset files to disk at that path
    // This can be a subcommand to create a ruleset from all the rules in a
    // specific path.
    // let target_path = &args[3];

    // // check the path.
    // check_path_panic(target_path);

    // // write the rulesets to disk.

    // for (id, rule_file) in rulesets {
    //     match rule_file.to_string() {
    //         Ok(s) => {
    //             let mut file_name: String = target_path.to_string();
    //             file_name.push_str(&id);
    //             file_name.push_str(".yaml");
    //             fs::write(file_name, s).expect("couldn't write file to disk");
    //         },
    //         Err(e) => info!("{}", e.to_string()),
    //     };
    // }

    // // create an HTTP server and serve the files.

    // use tiny_http::{Response, Server};

    // let port = &args[3];
    // let mut server_address = constants::LOCALHOST.to_string();
    // server_address.push_str(constants::SERVER_DELIM);
    // server_address.push_str(port.as_str());

    // let server = Server::http(server_address).unwrap();

    // for request in server.incoming_requests() {
    //     let url = request.url();

    //     // handle requests here.

    //     info!("method: {}, url: {}", request.method(), request.url());

    //     // request.u

    //     // info!(
    //     //     "received request! method: {:?}, url: {:?}, headers: {:?}",
    //     //     request.method(),
    //     //     request.url(),
    //     //     request.headers()
    //     // );

    //     let response = Response::from_string("hello world");
    //     _ = request.respond(response);
    // }
}
