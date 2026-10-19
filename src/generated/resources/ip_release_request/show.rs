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
    #[serde(default, rename = "allow_keep")]
    pub allow_keep: Option<bool>,
    #[serde(default, rename = "can_assign")]
    pub can_assign: Option<bool>,
    #[serde(default, rename = "can_keep")]
    pub can_keep: Option<bool>,
    #[serde(default, rename = "closed_at")]
    pub closed_at: Option<String>,
    #[serde(default, rename = "deadline")]
    pub deadline: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "ip_release_campaign")]
    pub ip_release_campaign: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=subject"]
    #[serde(default, rename = "mail_log")]
    pub mail_log: Option<runtime::Association>,
    #[serde(default, rename = "notified_at")]
    pub notified_at: Option<String>,
    #[serde(default, rename = "original_user_id")]
    pub original_user_id: Option<i64>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[serde(default, rename = "user_login")]
    pub user_login: Option<String>,
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
    name: "ip_release_request/show",
    method: runtime::Method::Get,
    path: "v7.0/ip_release_requests/{ip_release_request_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("ip_release_request"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "ip_release_request",
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
