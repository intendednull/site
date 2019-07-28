use actix_web::{error, Result, web, HttpResponse};
use serde::Deserialize;
use lettre::sendmail::SendmailTransport;
use lettre::{SendableEmail, Envelope, EmailAddress, Transport};


#[derive(Deserialize)]
struct EmailForm {
    name: String,
    email: String,
    body: String
}


// Match a post to contact page.
pub fn mail_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/contact")
            .route(web::post().to(mail))
    );
}


// TODO Figure out why the email isn't being received.
fn mail((form, tmpl): (web::Form<EmailForm>, web::Data<tera::Tera>)) -> Result<HttpResponse> {
    let email = SendableEmail::new(
        Envelope::new(
            // From
            Some(EmailAddress::new(form.email.clone()).unwrap()),
            // To
            vec![EmailAddress::new("noah@coronasoftware.net".to_string()).unwrap()],
        ).unwrap(),
        // Subject
        format!("Contact from: {}", form.name.clone()),
        // Body
        form.body.as_bytes().to_vec()
    );
    let mut sender = SendmailTransport::new();
    let result = sender.send(email);

    // User feedback.
    // TODO Make this more general, usable by all services.
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
