use actix_web::{error, Result, web, HttpResponse};
use serde::Deserialize;


pub fn mail_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog")
            .route(web::post().to(mail))
    );
}
