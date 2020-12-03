use actix_web::{error, web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::str;
use tera::{Context, Tera};

async fn greet() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("Hello, world!"))
}

#[derive(Serialize, Deserialize)]
pub struct FormParams {
    input: String,
}

async fn forms(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("input", &"");
    let view = tmpl
        .render("forms.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

async fn simple_form(
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("input", &params.input);
    let view = tmpl
        .render("forms.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let templates = Tera::new("templates/**/*").unwrap();
    HttpServer::new(move || {
        App::new()
            .data(templates.clone())
            .route("/", web::get().to(greet))
            .route("/forms", web::get().to(forms))
            .route("/forms/simple", web::post().to(simple_form))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
