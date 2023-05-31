use std::path::PathBuf;
use thiserror::Error;

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
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
