use actix_web::{error, Result, web, HttpResponse};
use serde::Deserialize;
use lettre::sendmail::SendmailTransport;
use lettre::{SendableEmail, Envelope, EmailAddress, Transport};


#[derive(Deserialize)]
struct EmailForm {
    name: String,
    email: Option<String>,
    body: String
}


pub fn mail_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/contact")
            .route(web::post().to(mail))
    );
}


fn mail((mut form, tmpl): (web::Form<EmailForm>, web::Data<tera::Tera>)) -> Result<HttpResponse> {
    let email = SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new(form.email.take().unwrap()).unwrap()),
            vec![EmailAddress::new("noah@coronasoftware.net".to_string()).unwrap()],
        ).unwrap(),
        form.name.clone(),
        form.body.as_bytes().to_vec()
    );
    let mut sender = SendmailTransport::new();
    let result = sender.send(email);

    let mut context = tera::Context::new();
    if result.is_ok() {
        context.insert("message", "Success!");
    } else {
        context.insert("error", "Oops, something went wrong..");
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(
        tmpl.render("contact.html", &context)
            .map_err(|_| error::ErrorInternalServerError("Template error."))?)
    )
}
