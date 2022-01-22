use std::path::Path;
use std::error::Error;

use rocket::serde::Serialize;
use rocket::trace::{info_span,error,error_span};

use crate::engine::Engine;

pub use crate::tera::{Context, Tera};


impl Engine for Tera {
    const EXT: &'static str = "tera";

    fn init<'a>(templates: impl Iterator<Item = (&'a str, &'a Path)>) -> Option<Self> {
        // Create the Tera instance.
        let mut tera = Tera::default();
        let ext = [".html.tera", ".htm.tera", ".xml.tera", ".html", ".htm", ".xml"];
        tera.autoescape_on(ext.to_vec());

        // Collect into a tuple of (name, path) for Tera. If we register one at
        // a time, it will complain about unregistered base templates.
        let files = templates.map(|(name, path)| (path, Some(name)));

        // Finally try to tell Tera about all of the templates.
        if let Err(e) = tera.add_template_files(files) {
            let span = error_span!("Failed to initialize Tera templating.");
            let _e = span.enter();
            let mut error = Some(&e as &dyn Error);
            while let Some(err) = error {
                error!(error = %err);
                error = err.source();
            }

            None
        } else {
            Some(tera)
        }
    }

    fn render<C: Serialize>(&self, name: &str, context: C) -> Option<String> {
        let span = info_span!("Rendering Tera template", template = %name);
        let _e = span.enter();
        if self.get_template(name).is_err() {
            error!("Tera template '{}' does not exist.", name);
            return None;
        };

        let tera_ctx = Context::from_serialize(context)
            .map_err(|e| error!("Tera context error: {}.", e))
            .ok()?;

        match Tera::render(self, name, &tera_ctx) {
            Ok(string) => Some(string),
            Err(e) => {
                let span = error_span!("Error rendering Tera template", template = %name);
                let _e = span.enter();
                let mut error = Some(&e as &dyn Error);
                while let Some(err) = error {
                    error!(error = %err);
                    error = err.source();
                }

                None
            }
        }
    }
}
