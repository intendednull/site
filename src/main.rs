use tera;
use actix_web::{web, middleware, error, App, HttpResponse, Error, HttpServer, Responder};


fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let r = tmpl.render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error."))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(r))
}


fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera = tera::compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*"));

        App::new()
            .data(tera)
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .workers(1)
        .run()
        .unwrap()
}
