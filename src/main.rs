// #[cfg(test)]
// mod tests;
use tera;
use std::path::{PathBuf};
use serde::Deserialize;
use actix_files as fs;
use actix_web::{
    HttpResponse, HttpServer,
    Result, App, web, middleware,
    error, http::header
};

mod mail;
mod blog;


#[derive(Deserialize)]
struct File {
    path: PathBuf
}


// Redirect to `/home`.
fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Found()
       .header(header::LOCATION, "/home")
       .finish())
}


// Render and serve templates.
// TODO Use global lazy static?
// TODO Implement actix friendly template errors.
fn page((tmpl, pg): (web::Data<tera::Tera>, web::Path<File>)) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
       .content_type("text/html")
       .body(
           tmpl.render(
               pg.path.with_extension("html").to_str().unwrap(),
               &tera::Context::new()
           ).map_err(|_| error::ErrorInternalServerError("Template error."))?
       ))
}


// Redirect asset requests to static file service.
// Backwards compatible with old site.
fn asset(file: web::Path<File>) -> Result<HttpResponse> {
    let fname = file.path
        .file_name().unwrap()
        .to_str().unwrap();
    Ok(HttpResponse::Found()
       .header(header::LOCATION, format!("/static/assets/{}", fname))
       .finish())
}


// Uses relative paths.
// TODO Use ini file.
fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        // Create template files from raw blog posts
        blog::update_blog();
        // Register all templates
        let tera = tera::compile_templates!("./src/templates/**/*");

        App::new()
            // Middleware
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            // Templates
            .data(tera)
            // Routes
            .route("/", web::get().to(index))
            .route("/{path}", web::get().to(page))
            .route("/s/{path:.*}", web::get().to(asset))
            // Services
            .configure(mail::mail_service)
            .service(fs::Files::new("/static", "./src/static").show_files_listing())
    })
        // .bind_uds("./site.sock")
        .bind("127.0.0.1:8080")?
        .workers(1)
        .run()
}
