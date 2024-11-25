use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;
use serde_json::Value;

pub fn render_template(template_path: &str, context: &HashMap<String, Value>) -> Result<String, Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();
    let template = fs::read_to_string(template_path)?;
    handlebars.register_template_string("email_template", template)?;
    print!("{:?} \n", context);
    let rendered = handlebars.render("email_template", &context)?;
    Ok(rendered)
}
