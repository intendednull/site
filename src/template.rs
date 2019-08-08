use tera;
use serde::Serialize;
use lazy_static::lazy_static;
use actix_web::{HttpResponse, Result, error};

use super::blog;

lazy_static! {
    pub static ref TERA: tera::Tera = {
        let mut _tera = tera::compile_templates!("src/templates/**/*");
        blog::configure_tera(&mut _tera);
        _tera
    };
}

#[derive(Serialize)]
pub struct Message {
    level: String,
    message: String
}

impl Message {
    pub fn new(level: &str, message: &str) -> Self {
        Self { level: level.to_owned(), message: message.to_owned() }
    }
    pub fn info(message: &str) -> Self { Self::new("info", message) }
    pub fn success(message: &str) -> Self { Self::new("success", message) }
    pub fn warn(message: &str) -> Self { Self::new("warning", message) }
    pub fn error(message: &str) -> Self { Self::new("danger", message) }
}


/// Render templates.
pub fn render<T: Serialize>(fp: &str, context: &T) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
       .content_type("text/html")
       .body(TERA.render(&fp, context)
             .map_err(|e| error::ErrorInternalServerError(format!("Template error: {:?}", e)))?
       ))
}
