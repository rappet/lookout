use actix_web::{web, HttpRequest, HttpResponse, };
use actix_files::NamedFile;
use std::path::PathBuf;
use tera::{Context, Tera};
use futures::future::{join_all, ok as fut_ok, Future, Either};

use std::default::Default;
use std::net::IpAddr;
use std::process;
use std::str::from_utf8;

use crate::error::Result;

//?command=traceroute&target=rappet.de&rawoutput=true
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    target: Option<Target>,
    #[serde(default)]
    command: Command,
    #[serde(default)]
    rawoutput: bool,
}

impl Params {
    fn perform(self) -> Result<Option<String>> {
        //let output = &process::Command::new(cmd.name()).args(&[&ip.to_string()]).output()?.stdout;
        match (self.target, self.command) {
            (Some(Target::Ip(ip)), Command::Ping) => {
                let output = process::Command::new("ping").args(&["-c5", &ip.to_string()]).output()?.stdout;
                Ok(Some(String::from_utf8(output)?))
            },
            (Some(Target::Ip(ip)), Command::Traceroute) => {
                let output = process::Command::new("traceroute").args(&["-A", &ip.to_string()]).output()?.stdout;
                Ok(Some(String::from_utf8(output)?))
            },
            (Some(Target::Ip(ip)), Command::Mtr) => {
                let output = process::Command::new("mtr").args(&["-wenzc3", &ip.to_string()]).output()?.stdout;
                Ok(Some(String::from_utf8(output)?))
            },
            _ => Ok(None)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Ping,
    Traceroute,
    Mtr,
}

impl Command {
    fn name(self) -> &'static str {
        match self {
            Command::Ping => "ping",
            Command::Traceroute => "traceroute",
            Command::Mtr => "mtr"
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

fn perform_lg(templ: &Tera, params: &Params) -> Result<String> {
    let mut ctx = Context::new();
    ctx.insert("params", params);

    match params.clone() {
        Params { target: Some(Target::Ip(ip)), command: cmd, .. } => {
            ctx.insert("output", &if let Some(output) = params.clone().perform()? {
                output
            } else {
                String::from("unimplemented")
            });
            let s = templ.render("output-raw.html", &ctx)?;
            Ok(s)
        },
        _ => {
            let s = templ.render("index.html", &ctx)?;
            Ok(s)
        }
    }
}

pub fn index(
    templ: web::Data<Tera>,
    params: web::Query<Params>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    web::block(move || perform_lg(&templ, &params))
    .from_err()
    .and_then(|s| HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn raw(
    templ: web::Data<Tera>,
    params: web::Query<Params>,
) -> Result<HttpResponse> {
    let mut ctx = Context::new();
    ctx.insert("output", &format!("{:?}", params));
    ctx.insert("params", &params.0);
    let s = templ.render("output-raw.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn traceroute(
    templ: web::Data<Tera>,
    params: web::Query<Params>,
) -> Result<HttpResponse> {
    let mut ctx = Context::new();
    ctx.insert("output", &format!("{:?}", params));
    ctx.insert("params", &params.0);
    let s = templ.render("output-traceroute.html", &ctx)?;
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