use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "reason",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reason: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.reason)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "address")]
    pub address: Option<String>,
    #[serde(default, rename = "assign_ip_address_id")]
    pub assign_ip_address_id: Option<i64>,
    #[serde(default, rename = "cleanup_state")]
    pub cleanup_state: Option<String>,
    #[serde(default, rename = "excluded_at")]
    pub excluded_at: Option<String>,
    #[serde(default, rename = "exclusion_reason")]
    pub exclusion_reason: Option<String>,
    #[serde(default, rename = "exempted_at")]
    pub exempted_at: Option<String>,
    #[serde(default, rename = "exempted_by_id")]
    pub exempted_by_id: Option<i64>,
    #[serde(default, rename = "exempted_by_login")]
    pub exempted_by_login: Option<String>,
    #[serde(default, rename = "exemption_reason")]
    pub exemption_reason: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=addr"]
    #[serde(default, rename = "ip_address")]
    pub ip_address: Option<runtime::Association>,
    #[serde(default, rename = "keep_reason")]
    pub keep_reason: Option<String>,
    #[serde(default, rename = "kept_at")]
    pub kept_at: Option<String>,
    #[serde(default, rename = "kept_by_id")]
    pub kept_by_id: Option<i64>,
    #[serde(default, rename = "kept_by_login")]
    pub kept_by_login: Option<String>,
    #[serde(default, rename = "last_result")]
    pub last_result: Option<String>,
    #[serde(default, rename = "location_label")]
    pub location_label: Option<String>,
    #[serde(default, rename = "original_user_id")]
    pub original_user_id: Option<i64>,
    #[serde(default, rename = "prefix")]
    pub prefix: Option<i64>,
    #[serde(default, rename = "protection")]
    pub protection: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "release_chain")]
    pub release_chain: Option<runtime::Association>,
    #[serde(default, rename = "released_at")]
    pub released_at: Option<String>,
    #[serde(default, rename = "size")]
    pub size: Option<i64>,
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
    name: "ip_release_request/address/exempt",
    method: runtime::Method::Post,
    path: "v7.0/ip_release_requests/{ip_release_request_id}/addresses/{address_id}/exempt",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("address"),
        parameters: &[runtime::ParameterSpec {
            name: "reason",
            kind: runtime::ParameterKind::Text,
            required: true,
            nullable: false,
            protected: false,
        }],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "address",
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
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reason = runtime::InputValue::Value(value.into());
        self
    }
}
