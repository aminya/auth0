use crate::authentication::Api;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestParameters {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestParameters {
    pub access_token: String,
    pub user_id: String,
}

pub trait Users {
    fn create_user(&self, request: CreateRequestParameters) -> RequestBuilder;
    fn read_user(&self, request: RequestParameters) -> RequestBuilder;
    fn update_user(&self, request: RequestParameters) -> RequestBuilder;
    fn delete_user(&self, request: RequestParameters) -> RequestBuilder;
}

impl Users for Api {
    fn create_user(&self, request: CreateRequestParameters) -> RequestBuilder {
        let url = self.base_url.join("/api/v2/users/").unwrap();
        let headers = create_headers(&request.access_token);
        self.client.post(url).headers(headers)
    }

    fn read_user(&self, request: RequestParameters) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/users/{}", request.user_id))
            .unwrap();
        let headers = create_headers(&request.access_token);
        self.client.get(url).headers(headers)
    }

    fn update_user(&self, request: RequestParameters) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/users/{}", request.user_id))
            .unwrap();
        let headers = create_headers(&request.access_token);
        self.client.patch(url).headers(headers)
    }

    fn delete_user(&self, request: RequestParameters) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/users/{}", request.user_id))
            .unwrap();
        let headers = create_headers(&request.access_token);
        self.client.delete(url).headers(headers)
    }
}

fn create_headers(access_token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let auth_value = format!("Bearer {}", access_token);
    headers.insert(
        reqwest::header::AUTHORIZATION,
        HeaderValue::from_str(&auth_value).unwrap(),
    );
    headers
}
