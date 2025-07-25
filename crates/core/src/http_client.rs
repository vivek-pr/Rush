use reqwest::{Client, Response, StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<String, HttpClientError> {
        let resp = self.client.get(url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn delete(&self, url: &str) -> Result<String, HttpClientError> {
        let resp = self.client.delete(url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn post<T: Serialize + ?Sized>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<String, HttpClientError> {
        let resp = self.client.post(url).json(body).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn put<T: Serialize + ?Sized>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<String, HttpClientError> {
        let resp = self.client.put(url).json(body).send().await?;
        Self::handle_response(resp).await
    }

    async fn handle_response(resp: Response) -> Result<String, HttpClientError> {
        let status = resp.status();
        let text = resp.text().await?;
        if status.is_success() {
            Ok(text)
        } else {
            Err(HttpClientError::Status(status))
        }
    }
}

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("unexpected status: {0}")]
    Status(StatusCode),
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_get_ok() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/get");
            then.status(200).body("hello");
        });

        let client = HttpClient::new();
        let res = client.get(&server.url("/get")).await.unwrap();
        assert_eq!(res, "hello");
    }

    #[tokio::test]
    async fn test_post_ok() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(POST)
                .path("/post")
                .json_body_obj(&serde_json::json!({"k": "v"}));
            then.status(200).body("done");
        });

        let client = HttpClient::new();
        let body = serde_json::json!({"k": "v"});
        let res = client.post(&server.url("/post"), &body).await.unwrap();
        assert_eq!(res, "done");
    }

    #[tokio::test]
    async fn test_error_status() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/fail");
            then.status(500).body("error");
        });

        let client = HttpClient::new();
        let err = client.get(&server.url("/fail")).await.unwrap_err();
        match err {
            HttpClientError::Status(code) => assert_eq!(code, StatusCode::INTERNAL_SERVER_ERROR),
            _ => panic!("unexpected error"),
        }
    }
}
