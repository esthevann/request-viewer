use std::collections::HashMap;

use reqwest::{Client, IntoUrl, Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;


#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NewRequest {
    address: String,
    method: MethodType,
    body: String,
    body_type: BodyType,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Response {
    status: u16,
    body: String,
    headers: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, TS)]
#[ts(export)]
pub enum MethodType {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

impl From<&MethodType> for Method {
    fn from(method: &MethodType) -> Self {
        match method {
            MethodType::GET => Self::GET,
            MethodType::POST => Self::POST,
            MethodType::PUT => Self::PUT,
            MethodType::PATCH => Self::PATCH,
            MethodType::DELETE => Self::DELETE,
        }
    }
}

impl From<crate::prisma::Method> for MethodType  {
    fn from(method: crate::prisma::Method) -> Self {
        match method {
            crate::prisma::Method::Get => Self::GET,
            crate::prisma::Method::Post => Self::POST,
            crate::prisma::Method::Put => Self::PUT,
            crate::prisma::Method::Patch => Self::PATCH,
            crate::prisma::Method::Delete => Self::DELETE,
        }
    }
}

impl From<MethodType> for crate::prisma::Method {
    fn from(method: MethodType) -> Self {
        match method {
            MethodType::GET => Self::Get,
            MethodType::POST => Self::Post,
            MethodType::PUT => Self::Put,
            MethodType::PATCH => Self::Patch,
            MethodType::DELETE => Self::Delete,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]

enum BodyType {
    Json,
    Text,
    Bytes,
}

#[derive(Error, Serialize, Deserialize, Debug)]
enum RequestError {
    #[error("Error while sending request")]
    SendError
}

impl From<RequestError> for tauri::Error {
    fn from(_e: RequestError) -> Self {
        Self::FailedToSendMessage
    }
}

trait ClientExt {
    fn set_method<U>(self, method: &MethodType, url: U) -> RequestBuilder
    where
        U: IntoUrl;
}

trait RequestBuilderExt {
    fn set_content_type(self, body_type: &BodyType) -> RequestBuilder;
}

impl RequestBuilderExt for RequestBuilder {
    fn set_content_type(self, body_type: &BodyType) -> RequestBuilder {
        match body_type {
            BodyType::Json => self.header("Content-Type", "application/json"),
            BodyType::Text => self.header("Content-Type", "text/plain"),
            BodyType::Bytes => self.header("Content-Type", "application/octet-stream"),
        }
    }
}

impl ClientExt for Client {
    fn set_method<U>(self, method: &MethodType, url: U) -> RequestBuilder
    where
        U: IntoUrl,
    {
        self.request(method.into(), url)
    }
}


#[tauri::command]
pub async fn send_request(request: NewRequest) -> tauri::Result<Response> {
    let res = Client::new()
        .set_method(&request.method, request.address)
        .set_content_type(&request.body_type)
        .body(request.body)
        .send()
        .await;

    match res {
        Ok(res) => {
            let mut headers = HashMap::new();
            for (key, value) in res.headers()  {
                headers.insert(key.as_str().to_owned(), value.to_str().unwrap_or_default().to_owned());
            }
            Ok(Response {
                status: res.status().as_u16(),
                body: res.text().await.unwrap_or_default(),
                headers
            })
        }
        Err(e) => {
            println!("From rust: {e}");
            Err(RequestError::SendError.into())
        },
    }

}

