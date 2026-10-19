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
    #[serde(default, rename = "address")]
    pub address: Option<String>,
    #[serde(default, rename = "assigned")]
    pub assigned: Option<i64>,
    #[serde(default, rename = "available_to_users")]
    pub available_to_users: Option<i64>,
    #[serde(default, rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "ip_version")]
    pub ip_version: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "managed")]
    pub managed: Option<bool>,
    #[serde(default, rename = "owned")]
    pub owned: Option<i64>,
    #[serde(default, rename = "owned_unassigned")]
    pub owned_unassigned: Option<i64>,
    #[serde(default, rename = "prefix")]
    pub prefix: Option<i64>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "primary_location")]
    pub primary_location: Option<runtime::Association>,
    #[serde(default, rename = "purpose")]
    pub purpose: Option<String>,
    #[serde(default, rename = "role")]
    pub role: Option<String>,
    #[serde(default, rename = "size")]
    pub size: Option<i64>,
    #[serde(default, rename = "split_access")]
    pub split_access: Option<String>,
    #[serde(default, rename = "split_prefix")]
    pub split_prefix: Option<i64>,
    #[serde(default, rename = "taken")]
    pub taken: Option<i64>,
    #[serde(default, rename = "used")]
    pub used: Option<i64>,
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
    name: "network/show",
    method: runtime::Method::Get,
    path: "v7.0/networks/{network_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("network"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "network",
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
