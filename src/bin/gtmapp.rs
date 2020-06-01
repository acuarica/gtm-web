use gtm::{
    projects::Projects,
    services::{write_commits, write_workdir_status},
    NotesFilter,
};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use std::{error::Error, net::SocketAddr, thread, path::PathBuf};
use web_view::*;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    // let mut response = Response::new(Body::empty());
    let mut response = Response::builder()
        // .status(200)
        // .header("X-Custom-Foo", "Bar")
        .header("Access-Control-Allow-Origin", "*")
        // .body(json.into())
        .body(Body::empty())
        .unwrap();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::GET, "/v1/data/commits") => {
            let mut out = Vec::new();
            write_commits(&mut out, Projects::config()?.keys(), &NotesFilter::all())?;
            *response.body_mut() = Body::from(out);
        }
        (&Method::GET, "/v1/data/projects") => {
            let projects = Projects::config()?;
            let projects: Vec<&PathBuf> = projects.keys().collect();
            let json = serde_json::to_string(&projects).unwrap();
            *response.body_mut() = Body::from(json);
        }
        (&Method::GET, "/v1/data/status") => {
            let mut out = Vec::new();
            write_workdir_status(&mut out, Projects::config()?.keys());
            *response.body_mut() = Body::from(out);
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

#[tokio::main]
async fn serve(port: u16) {
    //-> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let make_service = make_service_fn(move |_conn| async move {
        Ok::<_, Box<dyn Error + Send + Sync>>(service_fn(handle))
    });
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    // Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let port = 3000;
    thread::spawn(move || {
        serve(port);
    });

    let html_content = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>gtm Dashboard</title>
</head>
</head>
</head>
<body class="bg-body text-primary">
  <script type="text/javascript">{script}</script>
  <script type="text/javascript">
    app('http://localhost:{port}');
  </script>
</body>
</html>
"#,
        script = include_str!("../../dist/app/app.js"),
        port = port,
    );

    web_view::builder()
        .title("gtm Dashboard")
        .content(Content::Html(html_content))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

    Ok(())
}
