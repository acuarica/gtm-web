use bytes::buf::BufExt as _;
use hyper::Body;
use hyper::Client;
use hyper::{body, Request, Uri};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::{str::from_utf8, io::Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn github_repos(token: &str) -> Result<Vec<GitHubRepo>> {
    fetch_json2(
        Request::builder()
            .method("GET")
            .uri("https://api.github.com/user/repos".parse::<Uri>()?)
            .header("User-Agent", "gtm Dashboard serve")
            .header("Accept", "application/json")
            .header("Authorization", format!("token {}", token))
            .body(Body::from(""))
            .expect("Request builder failed"),
    ).await
}

pub async fn fetch_json<T: DeserializeOwned>(url: hyper::Uri, method: &str) -> Result<T> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(method)
        .uri(url)
        .header("Accept", "application/json")
        .body(Body::from(""))
        .expect("Request builder failed");

    let res = client.request(req).await?;
    let body = body::aggregate(res).await?;
    let token = serde_json::from_reader(body.reader())?;
    Ok(token)
}

pub async fn fetch_json2<T: DeserializeOwned>(req: Request<Body>) -> Result<T> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.request(req).await?;
    let body = body::aggregate(res).await?;

    let mut buffer = Vec::new();
    body.reader().read_to_end(&mut buffer)?;

    println!("{:?}", from_utf8(buffer.as_slice()));
    let object = serde_json::from_reader(buffer.as_slice())?;

    // let object = serde_json::from_reader(body.reader())?;
    Ok(object)
}

#[derive(Deserialize, Debug)]
pub struct GitHubAccessToken {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubRepo {
    pub id: u64,
    pub node_id: String,
    pub full_name: String,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub ssh_url: String,
    pub clone_url: String,
}

pub trait QueryString {
    fn parse_query(&self) -> HashMap<String, String>;
}

impl<T> QueryString for Request<T> {
    fn parse_query(&self) -> HashMap<String, String> {
        self.uri()
            .query()
            .map(|v| {
                url::form_urlencoded::parse(v.as_bytes())
                    .into_owned()
                    .collect()
            })
            .unwrap_or_else(HashMap::new)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn github() {

        // let url = format!("https://github.com/login/oauth/access_token?client_id={clientID}&client_secret={clientSecret}&code={requestToken}");
    }
}
