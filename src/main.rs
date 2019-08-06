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
    error, http::header
};

mod mail;
mod blog;


#[derive(Deserialize)]
struct File {
    path: PathBuf
}

lazy_static! {
    pub static ref TERA: tera::Tera = tera::compile_templates!("src/templates/**/*");
    pub static ref CONF: Ini = Ini::load_from_file("conf.ini").unwrap();
}


/// Render and serve templates.
fn page((tmpl, pg): (web::Data<&TERA>, web::Path<File>)) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
       .content_type("text/html")
       .body(
           tmpl.render(
               pg.path.with_extension("html").to_str().unwrap(),
               &tera::Context::new()
           ).map_err(|_| error::ErrorInternalServerError("Template error."))?
       ))
}


/// Redirect asset requests to static file service.
/// Backwards compatible with old site.
fn asset(file: web::Path<File>) -> Result<HttpResponse> {
    let fname = file.path
        .file_name().unwrap()
        .to_str().unwrap();
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
            .data(&TERA)
            .data(&CONF)
            // Routes
            .route("/", web::to(
                || HttpResponse::Found()
                    .header(header::LOCATION, "/home")
                    .finish()))
            .route("/favicon.ico", web::to(
                || HttpResponse::Found()
                    .header(header::LOCATION, "/static/assets/favicon.ico")
                    .finish()))
            // Alias for /static/assets/
            .route("/s/{path:.*}", web::get().to(asset))
            .route("/{path}", web::get().to(page))
            // Services
            .service(fs::Files::new("/static", "static").show_files_listing())
            .configure(mail::mail_service)
    })
        // .bind_uds("./site.sock")?
        .bind("127.0.0.1:8080")?
        .workers(1)
        .run()
}
