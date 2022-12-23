use std::fmt;

use clap::Parser;
use semgrep_rs::{check_path_panic, GenericRuleIndex, PolicyIndex};

use log::info;

mod server;
use server::Server;

mod template;

// TODO: add this usage to the readme then remove it from code.

// clap CLI struct.
#[derive(Parser, Debug)]
#[command(
    override_usage = "./private-semgrep-server -r path/to/rules/ [-p path/to/policies/] [-s 9090] [-q]"
)]
#[command(version = "0.1")]
#[command(about = "Starts a personal Semgrep server", long_about = None)]
struct Cli {
    /// Path to rules (required)
    #[arg(short, long)]
    rules: String,

    /// Path to policies (optional)
    #[arg(short, long)]
    policies: Option<String>,

    /// Server port (optional)
    #[arg(short, long = "port", default_value_t = 9090)]
    server_port: i32,

    /// Turn off logging (optional)
    #[arg(short, long, default_value_t = false)]
    quiet: bool,

    /// Use complete ruleIDs (optional)
    #[arg(short, long)]
    complete: bool,
}

fn main() {
    let args = Cli::parse();

    // setup logging if quiet is not set.
    match args.quiet {
        true => (),
        false => log4rs::init_file("logging.yaml", Default::default()).unwrap(),
    }

    // check the rules path and panic if it's not correct.
    info!("Rules path: {}", args.rules);
    // check the path and panic if it doesn't exist.
    check_path_panic(&args.rules);

    // create the rule index and panic if it didn't happen.
    let generic_rule_index: GenericRuleIndex =
        GenericRuleIndex::from_path(&args.rules, None, None, args.complete).unwrap();
    // log all the rules.
    info!("Created the rule index at {}", &args.rules);
    info!("Processed {} rules:", generic_rule_index.len());
    print_vector(generic_rule_index.get_ids());
    info!("----------");

    // read the policies if a path is provided.
    let policy_index: PolicyIndex = match &args.policies {
        // create the policy index from the policies in the  provided path.
        Some(p) => {
            info!("Policy path: {}", p);
            PolicyIndex::from_path_simple(&p, &generic_rule_index).unwrap()
        }
        // only create the all policy.
        None => {
            info!("No policy path provided, only creating the 'all' policy");
            PolicyIndex::empty(&generic_rule_index).unwrap()
        }
    };

    // log all the policies.
    info!("Loaded {} policies:", policy_index.len());
    print_vector(policy_index.get_ids());
    info!("----------");

    // create an HTTP server and serve the files.
    let server = Server::new_local(
        &args.server_port.to_string(),
        generic_rule_index,
        policy_index,
    );
    info!("Server started on: {}", &server.get_address());
    server.start();
}

// print_vector logs the contents of a vector.
pub(crate) fn print_vector<T: fmt::Debug>(vec: Vec<T>) {
    for item in vec {
        info!("{:?}", item);
    }
}
