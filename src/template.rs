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
        TemplateInfo { rules, policies }
    }

    pub(crate) fn render_index(&self) -> Result<String> {
        // let tera = match Tera::new("*.html") {
        //     Ok(t) => t,
        //     Err(e) => return Error::wrap_string(e.to_string()),
        // };

        let tera = Tera::new("*.html").map_err(|e| Error::new(e.to_string()))?;

        let mut ctx = Context::from_serialize(self).map_err(|e| Error::new(e.to_string()))?;
        ctx.insert("info", self);

        tera.render("index.html", &ctx)
            .map_err(|e| Error::new(e.to_string()))
    }
}
