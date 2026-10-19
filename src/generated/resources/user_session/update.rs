use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.label)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "admin")]
    pub admin: Option<runtime::Association>,
    #[serde(default, rename = "api_ip_addr")]
    pub api_ip_addr: Option<String>,
    #[serde(default, rename = "api_ip_ptr")]
    pub api_ip_ptr: Option<String>,
    #[serde(default, rename = "auth_type")]
    pub auth_type: Option<String>,
    #[serde(default, rename = "client_ip_addr")]
    pub client_ip_addr: Option<String>,
    #[serde(default, rename = "client_ip_ptr")]
    pub client_ip_ptr: Option<String>,
    #[serde(default, rename = "client_version")]
    pub client_version: Option<String>,
    #[serde(default, rename = "closed_at")]
    pub closed_at: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "last_request_at")]
    pub last_request_at: Option<String>,
    #[serde(default, rename = "request_count")]
    pub request_count: Option<i64>,
    #[serde(default, rename = "scope")]
    pub scope: Option<String>,
    #[serde(default, rename = "token_fragment")]
    pub token_fragment: Option<String>,
    #[serde(default, rename = "token_interval")]
    pub token_interval: Option<i64>,
    #[serde(default, rename = "token_lifetime")]
    pub token_lifetime: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[serde(default, rename = "user_agent")]
    pub user_agent: Option<String>,
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
    name: "user_session/update",
    method: runtime::Method::Put,
    path: "v7.0/user_sessions/{user_session_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("user_session"),
        parameters: &[runtime::ParameterSpec {
            name: "label",
            kind: runtime::ParameterKind::String,
            required: true,
            nullable: false,
            protected: false,
        }],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "user_session",
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
}
