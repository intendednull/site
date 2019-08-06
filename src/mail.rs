use actix_web::{error, Result, web, HttpResponse};
use serde::Deserialize;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{Transport, SmtpClient};
use lettre_email::Email;
use super::{CONF, TERA};



#[derive(Deserialize)]
struct EmailForm {
    name: String,
    email: String,
    subject: Option<String>,
    body: String
}


/// Match a post to contact page.
pub fn mail_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/contact")
            .route(web::post().to(mail))
    );
}


/// Send contact form email
fn mail((mut form, tmpl, conf): (
        web::Form<EmailForm>,
        web::Data<&TERA>,
        web::Data<&CONF>)) -> Result<HttpResponse> {

    let smtp_conf = conf.section(Some("smtp")).unwrap();
    let email = Email::builder()
        .to(smtp_conf.get("mailto").unwrap().clone())
        .from((form.email.clone(), form.name.clone()))
        .subject(form.subject.take().unwrap_or(String::from("Contact Form")))
        .text(form.body.clone())
        .build().unwrap();

    let mut mailer = SmtpClient::new_simple(smtp_conf.get("server").unwrap()).unwrap()
        .authentication_mechanism(Mechanism::Login)
        .credentials(Credentials::new(
                smtp_conf.get("user").unwrap().clone(),
                smtp_conf.get("pass").unwrap().clone())
        )
        .transport();

    let result = mailer.send(email.into());
    mailer.close();
    // User feedback.
    // TODO Make this more general, usable by all services.
    let mut context = tera::Context::new();
    if result.is_ok() {
        context.insert("message", "Success!");
    } else {
        context.insert("error", "Oops, something went wrong..");
    }

    Ok(HttpResponse::Ok()
       .content_type("text/html")
       .body(tmpl.render("contact.html", &context)
             .map_err(|_| error::ErrorInternalServerError("Template error."))?
       ))
}
