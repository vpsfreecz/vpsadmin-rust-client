use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "access_token_lifetime")]
    pub access_token_lifetime: Option<String>,
    #[serde(default, rename = "access_token_seconds")]
    pub access_token_seconds: Option<i64>,
    #[serde(default, rename = "allow_single_sign_on")]
    pub allow_single_sign_on: Option<bool>,
    #[serde(default, rename = "authorization_start_requires_user_action")]
    pub authorization_start_requires_user_action: Option<bool>,
    #[serde(default, rename = "authorization_start_uri")]
    pub authorization_start_uri: Option<String>,
    #[serde(default, rename = "client_id")]
    pub client_id: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "is_default")]
    pub is_default: Option<bool>,
    #[serde(default, rename = "issue_refresh_token")]
    pub issue_refresh_token: Option<bool>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "redirect_uri")]
    pub redirect_uri: Option<String>,
    #[serde(default, rename = "refresh_token_seconds")]
    pub refresh_token_seconds: Option<i64>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "oauth2_client/show",
    method: runtime::Method::Get,
    path: "v7.0/oauth2_clients/{oauth2_client_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("oauth2_client"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "oauth2_client",
        allow_empty_response: false,
    }),
    meta_namespace: Some("_meta"),
    global_meta: &[
        runtime::ParameterSpec {
            name: "includes",
            kind: runtime::ParameterKind::Custom,
            required: false,
            nullable: false,
            protected: false,
        },
        runtime::ParameterSpec {
            name: "no",
            kind: runtime::ParameterKind::Boolean,
            required: false,
            nullable: false,
            protected: false,
        },
    ],
    object_meta: &[],
    auth: runtime::RequestAuth::Configured,
};
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {}
