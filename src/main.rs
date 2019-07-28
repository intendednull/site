#[cfg(test)]
mod tests;

use tera;
use std::io;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use actix_files as fs;
use actix_web::{
    HttpRequest, HttpResponse,
    Error, HttpServer,
    Responder, Result,
    web, middleware,
    error, App,
    http::header
};

mod mail;

#[derive(Deserialize)]
struct File {
    path: PathBuf
}


// Redirect to `/home`
fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Found()
       .header(header::LOCATION, "/home")
       .finish())
}


// The main distributer for page get requests.
fn page((tmpl, pg): (web::Data<tera::Tera>, web::Path<File>)) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        tmpl.render(
            pg.path.with_extension("html").to_str().unwrap(),
            &tera::Context::new()
        ).map_err(|_| error::ErrorInternalServerError("Template error."))?
    ))
}


// Redirect asset requests to the static file service.
// This is assigned to the url `/s/{file}` in order to maintain documents that
// link to the assets of my old website.
fn asset(file: web::Path<File>) -> Result<HttpResponse> {
    let name = file.path.file_name().unwrap().to_str().unwrap();
    Ok(HttpResponse::Found()
       .header(header::LOCATION, format!("/static/assets/{}", name))
       .finish())
}


fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        let tera = tera::compile_templates!("./src/templates/**/*");

        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(tera)
            .route("/", web::get().to(index))
            .route("/{path}", web::get().to(page))
            .configure(mail::mail_service)
            .service(fs::Files::new("/static", "./src/static").show_files_listing())
            .route("/s/{path:.*}", web::get().to(asset))
    })
        // .bind_uds("./site.sock")
        .bind("127.0.0.1:8080")
        .unwrap()
        .workers(1)
        .run()
        .unwrap()
}
