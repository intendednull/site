#[cfg(test)]
mod tests;

use tera;
use std::env;
use log;
use std::path::{Path, PathBuf};
use actix_files as fs;
use actix_web::{
    HttpRequest, HttpResponse,
    Error, HttpServer,
    web, middleware,
    error, App,
    Responder, Result
};


fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let r = tmpl.render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error."))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(r))
}


fn resource(req: HttpRequest) -> Result<fs::NamedFile> {
    let file: PathBuf = req.match_info().query("filename").parse().unwrap();
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/static/images"))
        .join(file);

    Ok(fs::NamedFile::open(path)?)
}


fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        // let root_dir = );
        let tera = tera::compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*"));

        App::new()
            .data(tera)
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/s/{filename:.*}", web::get().to(resource))
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .workers(1)
        .run()
        .unwrap()
}
