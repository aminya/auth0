use crate::authentication::mfa::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestParameters {
    pub grant_type: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub mfa_token: String,
    pub oob_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_code: Option<String>,
}
