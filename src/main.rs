extern crate actix_web;
extern crate actix_files;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;
extern crate derive_more;
extern crate mime;

use actix_web::{web, get, post, middleware, App, HttpRequest, HttpResponse, HttpServer, Responder, Scope};
use actix_web::dev::RequestHead;
use actix_web::http::header::ContentType;
use actix_files::NamedFile;
use std::path::PathBuf;

use tera::{Context, Tera};
use std::sync::{Arc, Mutex};

mod error;

use error::{LookoutError, Result};

#[derive(Clone)]
struct State {
    templ: Arc<tera::Tera>,
}

impl State {
    fn new() -> State {
        State {
            templ: Arc::new(compile_templates!("templates/**/*"))
        }
    }
}

fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let state = State::new();

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .route("/css/{filename:.*}", web::get().to(css))
            .route("/js/{filename:.*}", web::get().to(js))
            .route("/webfonts/{filename:.*}", web::get().to(webfonts))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

fn index(
    state: web::Data<State>
) -> Result<HttpResponse> {
    let s = state.templ.render("index.html", &tera::Context::new())?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn css(req: HttpRequest) -> Result<NamedFile> {
    let mut path = PathBuf::from("./static/css");
    path.push(req.match_info().query("filename"));
    Ok(NamedFile::open(path)?)
}

fn js(req: HttpRequest) -> Result<NamedFile> {
    let mut path = PathBuf::from("./static/js");
    path.push(req.match_info().query("filename"));
    Ok(NamedFile::open(path)?)
}

fn webfonts(req: HttpRequest) -> Result<NamedFile> {
    let mut path = PathBuf::from("./static/webfonts");
    path.push(req.match_info().query("filename"));
    Ok(NamedFile::open(path)?)
}