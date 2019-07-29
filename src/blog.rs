use pandoc;
use std::path::Path;
use actix_web::{http::header, Result, web, HttpResponse};
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
    let fin = Path::new("./src/static/blog").join(&post.title).with_extension("org");
    let fout = fin.parent().unwrap()
        .join("html")
        .join(fin.file_name().unwrap())
        .with_extension("html");

    assert_eq!(fin, Path::new("./src/static/blog/intro.org"));
    assert_eq!(fout, Path::new("./src/static/blog/html/intro.html"));

    let mut pdoc = pandoc::new();
    pdoc.add_input(fin.to_str().unwrap());
    pdoc.set_output(
        pandoc::OutputKind::File(fout.to_str().unwrap().to_string())
    );
    pdoc.execute().unwrap();

    Ok(HttpResponse::Found()
       .header(
           header::LOCATION,
           fout
               .strip_prefix("./src/").unwrap()
               .to_str().unwrap())
       .finish()
    )
}
