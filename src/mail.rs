use actix_web::{Result, web, HttpResponse};
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
        web::resource("/mail")
            .route(web::post().to(mail))
    );
}


fn mail(mut form: web::Form<EmailForm>) -> Result<HttpResponse> {
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
    assert!(result.is_ok());

    Ok(HttpResponse::Ok().finish())
}
