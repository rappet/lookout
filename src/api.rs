use actix_web::{web, HttpRequest, HttpResponse, };
use actix_files::NamedFile;
use std::path::PathBuf;
use tera::{Context, Tera};

use std::default::Default;
use std::net::IpAddr;

use crate::error::Result;

//?command=traceroute&target=rappet.de&rawoutput=true
#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    target: Option<Target>,
    #[serde(default)]
    command: Command,
    #[serde(default)]
    rawoutput: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Ping,
    Traceroute,
    Mtr,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Target {
    Ip(IpAddr),
    Host(String),
}



impl Default for Command {
    fn default() -> Command {
        Command::Traceroute
    }
}

pub fn index(
    templ: web::Data<Tera>,
    params: web::Query<Params>,
) -> Result<HttpResponse> {
    let mut ctx = Context::new();
    ctx.insert("output", &format!("{:?}", params));
    ctx.insert("params", &params.0);
    let s = templ.render("output-raw.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn raw(
    templ: web::Data<Tera>,
) -> Result<HttpResponse> {
    let s = templ.render("output-raw.html", &Context::new())?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn traceroute(
    templ: web::Data<Tera>,
) -> Result<HttpResponse> {
    let s = templ.render("output-template.html", &Context::new())?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn css(req: HttpRequest) -> Result<NamedFile> {
    let mut path = PathBuf::from("./static/css");
    path.push(req.match_info().query("filename"));
    Ok(NamedFile::open(path)?)
}

pub fn js(req: HttpRequest) -> Result<NamedFile> {
    let mut path = PathBuf::from("./static/js");
    path.push(req.match_info().query("filename"));
    Ok(NamedFile::open(path)?)
}

pub fn webfonts(req: HttpRequest) -> Result<NamedFile> {
    let mut path = PathBuf::from("./static/webfonts");
    path.push(req.match_info().query("filename"));
    Ok(NamedFile::open(path)?)
}