use actix_web::{Result, web, HttpResponse};
use serde::Deserialize;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{Transport, SmtpClient};
use lettre_email::Email;
use super::{CONF, template};



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
fn mail(mut form: web::Form<EmailForm>) -> Result<HttpResponse> {
    let smtp_conf = CONF.section(Some("smtp")).unwrap();
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

    let context = result.ok().map_or_else(
        || template::Message::error("Oops, something went wrong.."),
        |_| template::Message::success("Success!"));

    template::render("contact.html", &context)
}
