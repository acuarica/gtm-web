use bytes::buf::BufExt as _;
use hyper::Body;
use hyper::Client;
use hyper::{body, Request};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_json(url: hyper::Uri) -> Result<GitHubAccessToken> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method("POST")
        .uri(url)
        .header("Accept", "application/json")
        .body(Body::from(""))
        .expect("Request builder failed");

    let res = client.request(req).await?;
    let body = body::aggregate(res).await?;
    let token = serde_json::from_reader(body.reader())?;
    Ok(token)
}

#[derive(Deserialize, Debug)]
pub struct GitHubAccessToken {
    pub access_token: String,
    pub token_type: String,
}

#[cfg(test)]
mod tests {

    #[test]
    fn github() {

        // let url = format!("https://github.com/login/oauth/access_token?client_id={clientID}&client_secret={clientSecret}&code={requestToken}");
    }
}
