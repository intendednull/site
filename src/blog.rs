use pandoc;
use actix_web::{error, Result, web, HttpResponse};
use serde::Deserialize;


#[derive(Deserialize)]
struct BlogPost {
    title: String,
    version: Option<String>
}


pub fn blog_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog")
            .route("/{title}", web::get().to(blog))
            .route("/{title}/{version}", web::get().to(blog))
    );
}


fn blog(post: web::Path<BlogPost>) -> Result<HttpResponse> {


    Ok(HttpResponse::Ok().finish())
}
