// creating the index template.

use semgrep_rs::{Error, Result};
use serde::Serialize;
use tera::{Context, Tera};

//
#[derive(Serialize)]
pub(crate) struct TemplateInfo {
    rules: Vec<String>,
    policies: Vec<String>,
}

impl TemplateInfo {
    pub(crate) fn new(rules: Vec<String>, policies: Vec<String>) -> TemplateInfo {
        let mut rules = rules;
        rules.sort();

        let mut policies = policies;
        policies.sort();

        TemplateInfo { rules, policies }
    }

    pub(crate) fn render_index(&self) -> Result<String> {
        // let tera = match Tera::new("*.html") {
        //     Ok(t) => t,
        //     Err(e) => return Error::wrap_string(e.to_string()),
        // };

        let template = include_str!("data/index.html");
        let mut tera = Tera::default();
        // we shouldn't get any errors because the template is hardcoded.
        tera.add_raw_template("index", template)
            .map_err(|e| Error::new(e.to_string()))?;
        // serialize the template into a context.
        let ctx = Context::from_serialize(self).map_err(|e| Error::new(e.to_string()))?;
        // ctx.insert("info", self);
        // render the template and return.
        tera.render("index", &ctx)
            .map_err(|e| Error::new(e.to_string()))
    }
}
