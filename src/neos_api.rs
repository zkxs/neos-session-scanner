use bytes::Buf;
use hyper::{Body, Client, StatusCode, Uri};
use hyper_tls::HttpsConnector;

use crate::neos_dto::{ErrorResponse, Session};

#[derive(Debug)]
pub enum SessionResponse {
    Session(Session),
    Error(ErrorResponse),
}

pub async fn lookup_session(session_id: &str) -> Result<SessionResponse, String> {
    let uri: Uri = format!("{}{}", "https://www.neosvr-api.com/api/sessions/", session_id)
        .parse()
        .expect("Could not parse Neos session API URI");
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    match client.get(uri).await {
        Ok(r) => deserialize_session(r).await,
        Err(e) => Err(e.to_string()),
    }
}

async fn deserialize_session(response: hyper::Response<Body>) -> Result<SessionResponse, String> {
    let status = response.status();
    let body = hyper::body::aggregate(response).await
        .map_err(|e| format!("error aggregating session response body: {:?}", e))?;
    match status {
        StatusCode::OK => {
            serde_json::from_reader(body.reader())
                .map_err(|e| format!("error parsing session response body: {:?}", e))
                .map(|b| SessionResponse::Session(b))
        },
        _ => {
            serde_json::from_reader(body.reader())
                .map_err(|e| format!("error parsing error response body: {:?}", e))
                .map(|b| SessionResponse::Error(b))
        },
    }
}
