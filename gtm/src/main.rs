#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde_derive;
extern crate serde_json;

use git2::*;
use gtm::fetch_projects;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    command: Option<String>,
}

async fn gtm_web_service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    use hyper::*;
    let mut response = Response::builder()
        .status(200)
        .header("X-Custom-Foo", "Bar")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::empty())
        .unwrap();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/data/commits") => {
            let repo = Repository::open("/Users/luigi/work/home").unwrap();
            let notes = gtm::get_notes(&repo).unwrap();
            let json = serde_json::to_string(&notes).unwrap();
            *response.body_mut() = Body::from(json);
        }
        (&Method::GET, "/data/projects") => {
            let projects = fetch_projects();
            let json = serde_json::to_string(&projects).unwrap();
            *response.body_mut() = Body::from(json);
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}

#[tokio::main]
async fn serve() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(gtm_web_service)) });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn main() -> Result<(), git2::Error> {
    let args = Args::from_args();

    match args.command {
        Some(command) => println!("{}", command),
        _ => println!("{}", "nada"),
    }

    serve();
    // let repo = Repository::open("tests/cases/repo")?;

    // let notes = gtm::get_notes(&repo)?;
    // println!("{}", serde_json::to_string(&notes).unwrap());
    // for n in ns {
    //     println!("{:?}", n)
    // }

    Ok(())
}
