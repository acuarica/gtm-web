#![feature(async_closure)]

use git2::Repository;
use gtm::{clone::clone_repo, get_notes, NotesFilter};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use serde::ser::{SerializeSeq, Serializer};
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[cfg_attr(debug_assertions, structopt(version = env!("GTM_VERSION")))]
/// The gtm Dashboard server
///
/// Returns gtm time data for the specified services.
/// All data returned is in JSON format.
struct Args {
    /// Returns commits with gtm time data
    #[structopt(short, long)]
    addr: Option<String>,

    #[structopt(short, long)]
    port: Option<u16>,

    #[structopt(short, long)]
    datadir: String,
}

async fn handle(
    datadir: String,
    req: Request<Body>,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/v1/clone") => {
            let url = hyper::body::to_bytes(req).await?;
            let url: &str = std::str::from_utf8(&*url)?;
            println!("Cloning repository `{:?}` into {}", url, datadir);
            match clone_repo(url, datadir) {
                Ok(_repo) => *response.body_mut() = Body::from("Try POSTing data to /echo"),
                Err(err) => *response.body_mut() = Body::from(format!("Clone error: {:?}", err)),
            };
            // let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
            // let body = serde_json::from_slice(&req.into_body())?;
            // let url: String=  req.into_body().
        }
        (&Method::GET, "/v1/data/commits") => {
            let repo = Repository::open(format!("{}/{}", datadir, "git-clone-repo")).unwrap();
            let mut out = Vec::new();
            let mut ser = serde_json::Serializer::new(&mut out);
            let mut seq = ser.serialize_seq(None).unwrap();
            get_notes(
                |c| {
                    seq.serialize_element(&c.commit)
                        .expect("Could not serialize commit");
                },
                &repo,
                "sdfsdf",
                &NotesFilter::all(),
            )?;
            seq.end().expect("Could not end serialize commits");
            *response.body_mut() = Body::from(out);
        }
        (&Method::GET, "/v1/data/projects") => {
            let projects: Vec<&str> = vec!["test-project"];
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
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();
    let addr = match args.addr {
        None => Ipv4Addr::new(0, 0, 0, 0),
        Some(s) => s.parse::<Ipv4Addr>()?,
    };
    let port = args.port.unwrap_or(3000);

    println!("Binding to address: {:?}:{}", addr, port);
    println!("Datadir to clone into: {}", args.datadir);
    let addr = SocketAddr::from((addr, port));
    let datadir = args.datadir;
    let make_service = make_service_fn(move |_conn| {
        let datadir = datadir.clone();
        async move {
            Ok::<_, Box<dyn Error + Send + Sync>>(service_fn(move |req: Request<Body>| {
                handle(datadir.to_owned(), req)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
