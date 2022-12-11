use std::env;

use semgrep_rs::{check_path_panic, GenericRuleIndex, PolicyIndex};

use log::info;

mod server;
use crate::server::Server;

mod diag;

fn main() {
    // setup logging.
    log4rs::init_file("logging.yaml", Default::default()).unwrap();

    let args: Vec<String> = env::args().collect();

    // this will panic if nothing is passed.
    let registry_path = &args[1];

    info!("Registry path: {}", registry_path);

    // check the path.
    check_path_panic(registry_path);

    // create a simple rule index.
    let generic_rule_index: GenericRuleIndex = GenericRuleIndex::from_path_simple(registry_path);

    // create a rule index with complete paths.
    // let generic_rule_index: GenericRuleIndex =
    //     GenericRuleIndex::from_path(registry_path.to_string(), None, None, true);

    info!("Processed these rules:");
    diag::print_vector(generic_rule_index.get_ids());

    // second argument is the path to policies.
    let policy_path = &args[2];
    info!("Policy path: {}", policy_path);

    // create a PolicyIndex.
    let policy_index = PolicyIndex::from_path_simple(policy_path, &generic_rule_index);

    info!("Policy index created.");
    let index = policy_index.get_index();
    info!("Loaded {} policies.", index.len());
    // print all policy names.
    for k in index.keys() {
        info!("{}", k);
    }

    let mut port = &"1234".to_string();
    // make the 3rd argument optional.
    if args.len() == 4 {
        port = &args[3];
    }
    // create an HTTP server and serve the files.
    let server = Server::new_local(port, generic_rule_index, policy_index);
    info!("Server started on: {}", &server.get_address());
    server.start();
}
