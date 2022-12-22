use std::{env, process};

use semgrep_rs::{check_path_panic, GenericRuleIndex, PolicyIndex};

use log::{error, info};

mod server;
use crate::server::Server;

mod diag;

const USAGE: &str = "path/to/rules path/to/policies [1234]";

fn main() {
    // setup logging.
    log4rs::init_file("logging.yaml", Default::default()).unwrap();

    let args: Vec<String> = env::args().collect();

    // check the number of arguments.
    if args.len() < 1 {
        info!("{}", USAGE);
        error!("Provide at least one argument.");
        process::exit(1);
        // then return an exit code and leave.
    }

    let registry_path = &args[1];
    info!("Registry path: {}", registry_path);
    // check the path and panic if it doesn't exist.
    check_path_panic(registry_path);

    // create a simple rule index. panic if we coudldn't do it.
    let generic_rule_index = GenericRuleIndex::from_path_simple(registry_path).unwrap();

    // create a rule index with complete paths.
    // let generic_rule_index: GenericRuleIndex =
    //     GenericRuleIndex::from_path(registry_path.to_string(), None, None, true).unwrap();

    info!("Processed these rules:");
    diag::print_vector(generic_rule_index.get_ids());

    // second argument is the path to policies.
    let policy_path = &args[2];
    info!("Policy path: {}", policy_path);

    // create a PolicyIndex. we want to panic here because if the policy index
    // cannot be created, then the server cannot function.
    let policy_index = PolicyIndex::from_path_simple(policy_path, &generic_rule_index).unwrap();

    info!("Policy index created.");
    let index = policy_index.get_index();
    info!("Loaded {} policies.", index.len());
    // print all policy names.
    for k in index.keys() {
        info!("{}", k);
    }

    let mut port = &"1234".to_string();
    // make the 3rd argument optional, default value is 1234.
    if args.len() == 4 {
        port = &args[3];
    }
    // create an HTTP server and serve the files.
    let server = Server::new_local(port, generic_rule_index, policy_index);
    info!("Server started on: {}", &server.get_address());
    server.start();
}
