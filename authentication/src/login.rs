use crate::Api;
use reqwest::RequestBuilder;
pub use serde::{Deserialize, Serialize};

pub mod enterprise;
pub mod passive;
pub mod social;

pub trait Login {
    fn authorize<T: Serialize>(&self, request: T) -> RequestBuilder;
}

impl Login for Api {
    fn authorize<T: Serialize>(&self, request: T) -> RequestBuilder {
        let client = reqwest::Client::new();
        let endpoint = String::from("/authorize");
        let url = self.base_url.join(&endpoint).unwrap();
        client.get(url).query(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn enterprise_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
        let login = Api::init(base_url, authentication);
        let parameters = login::enterprise::RequestParamaters {
            response_type: String::from("some_awesome_response_type"),
            client_id: String::from("some_awesome_client_id"),
            connection: None,
            redirect_uri: String::from("some_awesome_redirect_uri"),
            state: None,
        };
        let request = login.authorize(parameters).build().unwrap();
        let test_url =
            String::from("https://your_domain/authorize?response_type=some_awesome_response_type&client_id=some_awesome_client_id&redirect_uri=some_awesome_redirect_uri");
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().is_empty(), true);
        assert_eq!(request.body().is_none(), true);
    }

    #[test]
    fn passive_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
        let login = Api::init(base_url, authentication);
        let parameters = login::passive::RequestParamaters {
            response_type: String::from("some_awesome_response_type"),
            client_id: String::from("some_awesome_client_id"),
            connection: None,
            redirect_uri: String::from("some_awesome_redirect_uri"),
            scope: None,
            state: Some(String::from("some_awesome_state")),
        };
        let request = login.authorize(parameters).build().unwrap();
        let test_url =
            String::from("https://your_domain/authorize?response_type=some_awesome_response_type&client_id=some_awesome_client_id&redirect_uri=some_awesome_redirect_uri&state=some_awesome_state");
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().is_empty(), true);
        assert_eq!(request.body().is_none(), true);
    }

    #[test]
    fn social_request() {
        let base_url = Url::parse("https://YOUR_DOMAIN").unwrap();
        let authentication = AuthenicationMethod::OAuth2Token(String::from("some_awesome_token"));
        let login = Api::init(base_url, authentication);
        let parameters = login::social::RequestParamaters {
            response_type: String::from("some_awesome_response_type"),
            client_id: String::from("some_awesome_client_id"),
            connection: None,
            redirect_uri: String::from("some_awesome_redirect_uri"),
            state: None,
            additional_parameters: None,
        };
        let request = login.authorize(parameters).build().unwrap();
        let test_url =
            String::from("https://your_domain/authorize?response_type=some_awesome_response_type&client_id=some_awesome_client_id&redirect_uri=some_awesome_redirect_uri");
        assert_eq!(request.method().as_str(), reqwest::Method::GET);
        assert_eq!(request.url().as_str(), test_url);
        assert_eq!(request.headers().is_empty(), true);
        assert_eq!(request.body().is_none(), true);
    }
}
