#[cfg(test)]
mod tests;

use tera;
use std::path::{Path, PathBuf};
use actix_files as fs;
use actix_web::{
    HttpRequest, HttpResponse,
    Error, HttpServer,
    Responder, Result,
    web, middleware,
    error, App
};


fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let r = tmpl.render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error."))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(r))
}


fn resource(req: HttpRequest) -> Result<fs::NamedFile> {
    let file: PathBuf = req.match_info().query("filename").parse().unwrap();
    let path = Path::new("./src/static/images").join(file);

    Ok(fs::NamedFile::open(path)?)
}


fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        let tera = tera::compile_templates!("./src/templates/**/*");

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
