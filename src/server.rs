// server API is defined here.

use std::io::Cursor;
use std::str::FromStr;

use log::error;
use semgrep_rs::{GenericRuleExt, PolicyIndex};
use tiny_http::{Header, Method, Request, Response};

use crate::GenericRuleIndex;

// localhost
const LOCALHOST: &str = "127.0.0.1:";

// represents the local HTTP server.
pub struct Server {
    address: String,
    server: Option<tiny_http::Server>,
    rule_index: GenericRuleIndex,
    policy_index: PolicyIndex,
}

impl Server {
    // creates a new server, note that server is not initialized and you must
    // call start first.
    pub fn new(address: String, rule_index: GenericRuleIndex, policy_index: PolicyIndex) -> Server {
        Server {
            address,
            server: None,
            rule_index,
            policy_index,
        }
    }

    // creates a new server that listens on localhost:port. The server is not
    // initialized and you must call start first.
    pub fn new_local(
        port: &str,
        rule_index: GenericRuleIndex,
        policy_index: PolicyIndex,
    ) -> Server {
        let mut addr: String = LOCALHOST.to_string();
        addr.push_str(port);
        Server::new(addr, rule_index, policy_index)
    }

    // start the server and populate the server field. We will panic if it
    // there's an error but that's expected. We want to know if server fails.
    pub fn start(mut self) {
        self.server = Some(tiny_http::Server::http(&self.address).unwrap());

        loop {
            let request = self
                .server
                .as_ref()
                .unwrap()
                .recv()
                .map_err(|e| error!("Error receiving request: {}", e.to_string()))
                .unwrap();

            let response = self.handle_request(&request);

            // ZZZ let's ignore the result for now.
            let _ = request.respond(response);
        }
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    fn handle_request(&self, request: &Request) -> Response<Cursor<Vec<u8>>> {
        // we only support GET right now.
        let method = request.method();
        if method != &Method::Get {
            self.not_found();
        }

        let path = request.url();

        // split the path by `/`.
        let segments: Vec<&str> = path.split('/').collect();

        // paths shorter than three segments are invalid.
        let path_length = segments.len();
        if path_length < 3 {
            return self.not_found();
        }

        // we will check the last three segments, we might start our server a few
        // paths down. `/path/to/c/r/ruleid` becomes `/c/r/ruleid`.

        // check for /c/
        match segments[path_length - 3] {
            "c" => match segments[path_length - 2] {
                // rule
                "r" => self.serve_rule(&segments[path_length - 1].to_string()),
                // policy
                "p" => self.serve_policy(&segments[path_length - 1].to_string()),
                // everything else
                _ => self.not_found(),
            },
            _ => self.not_found(),
        }
    }

    // serve_rule finds a rule in the rule index and returns it as a response.
    // Retrurns a 404 if it doesn't exist.
    fn serve_rule(&self, rule_id: &str) -> Response<Cursor<Vec<u8>>> {
        // get the rule from index.
        match self.rule_index.get_rule(rule_id) {
            // return any errors.
            None => Server::rule_not_found(format!("Rule {} not found.", rule_id)),
            // if the rule exists, create the rule YAML file and return it.
            Some(r) => match r.to_string() {
                // if there was an error in the YAML conversion.
                Err(e) => Server::rule_not_found(e.to_string()),
                Ok(y) => Server::ok_response(y),
            },
        }
    }

    // serve_rule finds a policy in the policy index and returns it as a response.
    // Retrurns a 404 if it doesn't exist.
    fn serve_policy(&self, policy_id: &str) -> Response<Cursor<Vec<u8>>> {
        // get the policy from the index.
        match self.policy_index.get_policy(policy_id) {
            None => Server::policy_not_found(format!("Policy {} not found.", policy_id)),
            // if the policy exists, return the content.
            Some(p) => Server::ok_response(p.get_content()),
        }
    }

    // not_found returns the 404 page with some examples.
    fn not_found(&self) -> Response<Cursor<Vec<u8>>> {
        let message = format!(
            "404 not found.\n\nRules: {}/c/r/{{ruleid}}\n\nPolicies: {}/c/p/{{policyname}}",
            self.address, self.address
        );

        Response::from_string::<String>(message)
            .with_status_code(404)
            .with_header(Header::from_str("Content-Type: text/plain").unwrap())
    }

    // rule_not_found returns a 404 when a rule is not found.
    fn rule_not_found(msg: String) -> Response<Cursor<Vec<u8>>> {
        let message = format!(
            r#"{}
Check if you're using the short rule ID format (e.g., rule_id_in_file)
or the complete format (e.g., path.to.rule.directory.rule_file_name.rule_id_in_file)"#,
            msg,
        );
        Response::from_string::<String>(message)
            .with_status_code(404)
            .with_header(Header::from_str("Content-Type: text/plain").unwrap())
    }

    // policy_not_found returns a 404 when a policy is not found.
    fn policy_not_found(message: String) -> Response<Cursor<Vec<u8>>> {
        Response::from_string::<String>(message)
            .with_status_code(404)
            .with_header(Header::from_str("Content-Type: text/plain").unwrap())
    }

    // ok_response returns a 200 response with the provided content.
    fn ok_response(content: String) -> Response<Cursor<Vec<u8>>> {
        Response::from_string(content)
            .with_header(Header::from_str("Content-Type: text/yaml").unwrap())
    }
}

// extract and return the path parameter from the request path.
fn get_path_parameter(path: &str, parameter: &str) -> Option<String> {
    let segments: Vec<&str> = path.split('/').collect();

    let index = segments.iter().position(|seg| seg == &parameter);

    match index {
        Some(i) => Some(segments[i].to_string()),
        None => None,
    }
}
