use tera;
use ini::Ini;
use lazy_static::lazy_static;
use dotenv::dotenv;
use actix_files as fs;
use std::path::{PathBuf};
use serde::Deserialize;
use actix_web::{
    HttpResponse, HttpServer,
    Result, App, web, middleware,
    http::header
};

mod mail;
mod blog;
mod template;


#[derive(Deserialize)]
struct File {
    path: Option<PathBuf>
}

lazy_static! {
    pub static ref CONF: Ini = Ini::load_from_file("conf.ini").unwrap();
}


/// Serve pages.
fn page(pg: web::Path<File>) -> Result<HttpResponse> {
    let template = match &pg.path {
        Some(p) => p.with_extension("html").to_str().unwrap().to_owned(),
        None => "home.html".to_owned(),
    };
    template::render(&template, &tera::Context::new())
}


/// Redirect asset requests to static file service.
/// Backwards compatible with old site.
fn asset(file: web::Path<File>) -> Result<HttpResponse> {
    let fname = file.path.as_ref().unwrap()
        .file_name().unwrap().to_str().unwrap();
    Ok(HttpResponse::Found()
       .header(header::LOCATION, format!("/static/assets/{}", fname))
       .finish())
}


/// Start the server.
fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    blog::update_blog();

    HttpServer::new(|| {
        App::new()
            // Middleware
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            // Routes
            .route("/favicon.ico", web::to(
                || HttpResponse::Found()
                    .header(header::LOCATION, "/static/assets/favicon.ico")
                    .finish()))
            .route("/s/{path:.*}", web::get().to(asset)) // Alias for /static/assets/
            .route("/{path}", web::get().to(page))
            .route("/", web::get().to(page))
            // Services
            .service(fs::Files::new("/static", "static").show_files_listing())
            .configure(mail::mail_service)
    })
        // .bind_uds("./site.sock")?
        .bind("127.0.0.1:8080")?
        .workers(1)
        .run()
}
