#![feature(async_closure)]

use git2::Repository;
use gtm::{
    clone::{clone_repo, url_path},
    get_notes,
    oauth2::{fetch_json, github_repos, GitHubAccessToken, GitHubRepo, QueryString},
    NotesFilter,
};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode, Uri,
};
use log::*;
use serde::ser::{SerializeSeq, Serializer};
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
};
use structopt::StructOpt;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(StructOpt, Clone)]
#[cfg_attr(debug_assertions, structopt(version = env!("GTM_VERSION")))]
/// The gtm Dashboard server
///
/// Returns gtm time data for the specified services.
/// All data returned is in JSON format.
struct Args {
    /// Returns commits with gtm time data
    #[structopt(short, long)]
    addr: String,

    #[structopt(short, long)]
    port: u16,

    #[structopt(short, long)]
    datadir: String,

    #[structopt(short, long)]
    rootdir: Option<String>,
}

async fn dispatch(
    args: Args,
    req: Request<Body>,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/v1/login/github") => {
            let params = req.parse_query();
            let client_id = "c0d72815c0f4d616112a";
            let client_secret = "0f42ddfea8ce3818a88c5affaf74a097c7235c1c";
            if let Some(code) = params.get("code") {
                info!("Getting code {} to authenticate", code);
                let url = format!("https://github.com/login/oauth/access_token?client_id={client_id}&client_secret={client_secret}&code={code}",
                    client_id=client_id,
                    client_secret=client_secret,
                    code=code
                );
                let uri: Uri = url.parse()?;
                trace!("URL to authenticate {:?}", uri);
                let token: GitHubAccessToken = fetch_json(uri, "POST").await?;
                return Ok(Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header("Location", format!("/?access_token={}", token.access_token))
                    .body(Body::empty())
                    .unwrap());
            } else {
                info!("invalid code, aborting");
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }
        (&Method::POST, "/v1/clone") => {
            let params = req.parse_query();
            if let Some(token) = params.get("access_token") {
                debug!("Fetching repos for token {}", token);
                let repos: Vec<GitHubRepo> = github_repos(token).await?;
                for repo in repos {
                    let url = repo.clone_url;
                    debug!("Cloning repository `{:?}` into {}", url, args.datadir);
                    match clone_repo(&url, &args.datadir) {
                        Ok(_repo) => *response.body_mut() = Body::from("Try POSTing data to /echo"),
                        Err(err) => {
                            *response.body_mut() = Body::from(format!("Clone error: {:?}", err))
                        }
                    };
                }
            } else {
                warn!("Access token not found to fetch projects");
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }
        (&Method::GET, "/v1/data/commits") => {
            debug!("serving commits");
            let params = req.parse_query();
            if let Some(token) = params.get("access_token") {
                debug!("Fetching repos for token {}", token);
                let repos: Vec<GitHubRepo> = github_repos(token).await?;
                let mut out = Vec::new();
                let mut ser = serde_json::Serializer::new(&mut out);
                let mut seq = ser.serialize_seq(None).unwrap();
                for ghrepo in repos {
                    let url_path = url_path(&ghrepo.clone_url);
                    if let Ok(repo) = Repository::open(format!("{}/{}", args.datadir, url_path)) {
                        if let Err(_err) = get_notes(
                            |c| {
                                seq.serialize_element(&c.commit)
                                    .expect("Could not serialize commit");
                            },
                            &repo,
                            &ghrepo.full_name,
                            &NotesFilter::all(),
                        ) {
                            debug!("No gtm-data for repo {}", ghrepo.full_name);
                        }
                    } else {
                        warn!("Could not open repo {}", ghrepo.full_name);
                    }
                }
                seq.end().expect("Could not end serialize commits");
                *response.body_mut() = Body::from(out);
            } else {
                warn!("Access token not found to fetch projects");
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }
        (&Method::GET, "/v1/data/projects") => {
            let params = req.parse_query();
            if let Some(token) = params.get("access_token") {
                debug!("Fetch repos for token {}", token);
                let projects: Vec<GitHubRepo> = github_repos(token).await?;
                let projects: Vec<&String> = projects.iter().map(|r| &r.full_name).collect();
                let json = serde_json::to_string(&projects).unwrap();
                *response.body_mut() = Body::from(json);
            } else {
                warn!("Access token not found to fetch projects");
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }
        (&Method::GET, _) => {
            let mut filename = req.uri().path();
            if filename == "/" {
                filename = "/index.html";
            }
            if let Some(rootdir) = args.rootdir {
                if let Ok(file) = File::open(format!("{}{}", rootdir, filename)).await {
                    let stream = FramedRead::new(file, BytesCodec::new());
                    let body = Body::wrap_stream(stream);
                    return Ok(Response::new(body));
                }
            }
            warn!("Static file `{}` not found", filename);
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
        _ => {
            info!("not found");
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

async fn handle(
    args: Args,
    req: Request<Body>,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    trace!("{} Request {:?}", req.method(), req.uri());
    match dispatch(args, req).await {
        Err(err) => {
            error!("Error: {:?}", err);
            Err(err)
        }
        ok => ok,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // pretty_env_logger::init();
    env_logger::init();

    let args = Args::from_args();
    let addr = args.addr.parse::<Ipv4Addr>()?;

    info!("Binding to address: {:?}:{}", addr, args.port);
    info!("Data dir to clone into: {}", args.datadir);
    info!(
        "Root dir to serve static files from: `{}`",
        match &args.rootdir {
            None => "None",
            Some(dir) => dir,
        }
    );
    let addr = SocketAddr::from((addr, args.port));
    let make_service = make_service_fn(move |_conn| {
        let args = args.clone();
        async move {
            Ok::<_, Box<dyn Error + Send + Sync>>(service_fn(move |req: Request<Body>| {
                handle(args.clone(), req)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    Ok(())
}
