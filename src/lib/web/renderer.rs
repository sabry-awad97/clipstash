use serde::Serialize;
use std::path::PathBuf;
use thiserror::Error;

use super::ctx;

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("Failed to get handlebars template {template_name}: {source}")]
    GetTemplateError {
        template_name: String,
        source: handlebars::TemplateError,
    },
    #[error("Failed to render handlebars template {template_name}: {source}")]
    RenderError {
        template_name: String,
        source: handlebars::RenderError,
    },
}

pub struct Renderer<'r>(handlebars::Handlebars<'r>);

impl<'r> Renderer<'r> {
    pub fn new(template_dir: PathBuf) -> Result<Self, RendererError> {
        let mut handlebars = handlebars::Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", &template_dir)
            .map_err(|e| RendererError::GetTemplateError {
                template_name: String::from("unknown"),
                source: e,
            })?;

        Ok(Self(handlebars))
    }

    pub fn render<P>(&self, context: P, errors: &[&str]) -> String
    where
        P: ctx::PageContext + serde::Serialize + std::fmt::Debug,
    {
        let mut value = convert_to_value(&context);
        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into());
            value.insert("_title".into(), context.title().into());
            value.insert("_base".into(), context.parent().into());
        }

        let rendered = self.do_render(context.template_path(), &value);
        rendered.expect("Failed to render template")
    }

    fn do_render(
        &self,
        template_name: &str,
        context: &serde_json::Value,
    ) -> Result<String, RendererError> {
        let rendered =
            self.0
                .render(template_name, context)
                .map_err(|e| RendererError::RenderError {
                    template_name: template_name.to_string(),
                    source: e,
                })?;
        Ok(rendered)
    }
}

fn convert_to_value<S>(value: &S) -> serde_json::Value
where
    S: Serialize,
{
    serde_json::to_value(value).expect("Failed to convert to JSON value")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_new() {
        let template_dir = tempdir().unwrap();
        let template_file = template_dir.path().join("template.hbs");
        let mut file = File::create(&template_file).unwrap();
        file.write_all(b"Hello, {{name}}!").unwrap();

        let renderer = Renderer::new(template_dir.path().to_path_buf()).unwrap();
        assert!(renderer.0.get_template("template").is_some());
    }

    #[test]
    fn test_do_render() {
        let template_dir = tempdir().unwrap();
        let template_file = template_dir.path().join("template.hbs");
        let mut file = File::create(&template_file).unwrap();
        file.write_all(b"Hello, {{name}}!").unwrap();

        let renderer = Renderer::new(template_dir.path().to_path_buf()).unwrap();
        let result = renderer
            .do_render("template", &json!({"name": "World"}))
            .unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_convert_to_value() {
        #[derive(Serialize)]
        struct User {
            name: String,
            age: u32,
        }

        let user = User {
            name: "John".to_string(),
            age: 30,
        };

        let expected_value = json!({
            "name": "John",
            "age": 30,
        });

        let value = convert_to_value(&user);
        assert_eq!(value, expected_value);
    }
}
