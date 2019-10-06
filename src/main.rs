extern crate actix_web;
extern crate actix_files;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;
extern crate derive_more;

use actix_web::{web, middleware, App, HttpServer};

mod error;
mod api;

use error::{Result};

#[derive(Clone)]
struct State {
}

impl State {
    fn new() -> State {
        State {}
    }
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let state = State::new();

    HttpServer::new(move || {
        let templ = compile_templates!("templates/**/*");
        App::new()
            .data(templ)
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .route("/css/{filename:.*}", web::get().to(api::css))
            .route("/js/{filename:.*}", web::get().to(api::js))
            .route("/webfonts/{filename:.*}", web::get().to(api::webfonts))
            .service(web::resource("/").route(web::get().to(api::index)))
            .service(web::resource("/raw").route(web::get().to(api::raw)))
            .service(web::resource("/traceroute").route(web::get().to(api::traceroute)))
    })
    .bind("[::]:8080")?
    .run()?;
    Ok(())
}