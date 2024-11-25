use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;

pub fn render_template(template_path: &str, context: &HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();
    let template = fs::read_to_string(template_path)?;
    handlebars.register_template_string("email_template", template)?;

    let rendered = handlebars.render("email_template", &context)?;
    Ok(rendered)
}
