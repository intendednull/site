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


// TODO Use ini file.
/// **WARNING** relative pathing
fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        blog::update_blog();
        let tera = tera::compile_templates!("./src/templates/**/*");

        App::new()
            // Middleware
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(tera)
            // Routes
            .route("/", web::to(
                || HttpResponse::Found()
                    .header(header::LOCATION, "/home")
                    .finish()))
            .route("/favicon.ico", web::to(
                || HttpResponse::Found()
                    .header(header::LOCATION, "/static/assets/favicon.ico")
                    .finish()))
            .route("/s/{path:.*}", web::get().to(asset))
            .route("/{path}", web::get().to(page))
            // Services
            .service(fs::Files::new("/static", "./src/static").show_files_listing())
            .configure(mail::mail_service)
    })
        // .bind_uds("./site.sock")?
        .bind("127.0.0.1:8080")?
        .workers(1)
        .run()
}
