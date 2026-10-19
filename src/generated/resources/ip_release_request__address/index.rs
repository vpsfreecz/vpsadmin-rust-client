use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "ip_release_request/address/index",
    method: runtime::Method::Get,
    path: "v7.0/ip_release_requests/{ip_release_request_id}/addresses",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("address"),
        parameters: &[
            runtime::ParameterSpec {
                name: "from_id",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "limit",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "addresses",
        allow_empty_response: false,
    }),
    meta_namespace: Some("_meta"),
    global_meta: &[
        runtime::ParameterSpec {
            name: "count",
            kind: runtime::ParameterKind::Boolean,
            required: false,
            nullable: false,
            protected: false,
        },
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Vec<Output>, Meta> {
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
}
