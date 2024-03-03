use crate::authentication::Api;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleRequestParameters {
    #[serde(skip)]
    pub access_token: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestParameters {
    pub access_token: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub permission_name: String,
    pub resource_server_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePermissionsRequestParameters {
    #[serde(skip)]
    pub access_token: String,
    #[serde(skip)]
    pub role_id: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleUserRequestParameters {
    #[serde(skip)]
    pub access_token: String,
    #[serde(skip)]
    pub role_id: String,
    pub users: Vec<String>,
}

pub trait Roles {
    fn create_role(&self, params: CreateRoleRequestParameters) -> RequestBuilder;
    fn read_role(&self, params: RequestParameters) -> RequestBuilder;
    fn update_role(&self, params: RequestParameters) -> RequestBuilder;
    fn delete_role(&self, params: RequestParameters) -> RequestBuilder;

    fn create_role_permissions(&self, params: ChangePermissionsRequestParameters)
        -> RequestBuilder;
    fn read_role_permissions(&self, params: RequestParameters) -> RequestBuilder;
    fn delete_role_permissions(&self, params: ChangePermissionsRequestParameters)
        -> RequestBuilder;

    fn create_role_users(&self, params: CreateRoleUserRequestParameters) -> RequestBuilder;
    fn read_role_users(&self, params: RequestParameters) -> RequestBuilder;
}

impl Roles for Api {
    fn create_role(&self, params: CreateRoleRequestParameters) -> RequestBuilder {
        let url = self.base_url.join("/api/v2/roles/").unwrap();
        let headers = create_headers(&params.access_token);
        self.client.post(url).headers(headers).json(&params)
    }

    fn read_role(&self, params: RequestParameters) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/roles/{}", params.role_id))
            .unwrap();
        let headers = create_headers(&params.access_token);
        self.client.get(url).headers(headers)
    }

    fn update_role(&self, params: RequestParameters) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/roles/{}", params.role_id))
            .unwrap();
        let headers = create_headers(&params.access_token);
        self.client.patch(url).headers(headers)
    }

    fn delete_role(&self, params: RequestParameters) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/roles/{}", params.role_id))
            .unwrap();
        let headers = create_headers(&params.access_token);
        self.client.delete(url).headers(headers)
    }

    fn create_role_permissions(
        &self,
        params: ChangePermissionsRequestParameters,
    ) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/roles/{}/permissions", params.role_id))
            .unwrap();
        let headers = create_headers(&params.access_token);
        self.client.post(url).headers(headers).json(&params)
    }

    fn read_role_permissions(&self, params: RequestParameters) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/roles/{}/permissions", params.role_id))
            .unwrap();
        let headers = create_headers(&params.access_token);
        self.client.get(url).headers(headers)
    }

    fn delete_role_permissions(
        &self,
        params: ChangePermissionsRequestParameters,
    ) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/roles/{}/permissions", params.role_id))
            .unwrap();
        let headers = create_headers(&params.access_token);
        self.client.delete(url).headers(headers).json(&params)
    }

    fn create_role_users(&self, params: CreateRoleUserRequestParameters) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/roles/{}/users", params.role_id))
            .unwrap();
        let headers = create_headers(&params.access_token);
        self.client.post(url).headers(headers).json(&params)
    }

    fn read_role_users(&self, params: RequestParameters) -> RequestBuilder {
        let url = self
            .base_url
            .join(&format!("/api/v2/roles/{}/users", params.role_id))
            .unwrap();
        let headers = create_headers(&params.access_token);
        self.client.get(url).headers(headers)
    }
}

fn create_headers(access_token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        HeaderValue::from_static("application/json"),
    );

    let auth_value = format!("Bearer {}", access_token);
    headers.insert(
        reqwest::header::AUTHORIZATION,
        HeaderValue::from_str(&auth_value).unwrap(),
    );

    headers
}
