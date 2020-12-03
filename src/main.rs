#[macro_use]
extern crate diesel;

use crate::schema::memos;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    sqlite::SqliteConnection,
};
use serde::{Deserialize, Serialize};
use std::str;
use tera::{Context, Tera};

pub mod models;
pub mod schema;

async fn greet() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("Hello, world!"))
}

#[derive(Serialize, Deserialize)]
pub struct FormParams {
    content: String,
}

async fn form(
    pool: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let memos = memos::table
        .load::<crate::models::Memo>(&conn)
        .expect("Error loading cards");
    ctx.insert("memos", &memos);
    let view = tmpl
        .render("form.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

async fn memo_form(
    pool: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let new_memo = crate::models::NewMemo {
        content: String::from(&params.content),
    };
    let conn = pool.get().expect("couldn't get db connection from pool");
    diesel::insert_into(memos::table)
        .values(&new_memo)
        .execute(&conn)
        .unwrap();
    let mut ctx = Context::new();
    let memos = memos::table
        .load::<crate::models::Memo>(&conn)
        .expect("Error loading cards");
    ctx.insert("memos", &memos);
    let view = tmpl
        .render("form.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let templates = Tera::new("templates/**/*").unwrap();

    let database_url = "database.sqlite3";
    let db_pool = r2d2::Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .expect("failed to create db connection pool");
    HttpServer::new(move || {
        App::new()
            .data(templates.clone())
            .data(db_pool.clone())
            .route("/", web::get().to(greet))
            .route("/form", web::get().to(form))
            .route("/form/memo", web::post().to(memo_form))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
