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
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "default_ttl")]
    pub default_ttl: Option<i64>,
    #[serde(default, rename = "dnssec_enabled")]
    pub dnssec_enabled: Option<bool>,
    #[serde(default, rename = "email")]
    pub email: Option<String>,
    #[serde(default, rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "reverse_network_address")]
    pub reverse_network_address: Option<String>,
    #[serde(default, rename = "reverse_network_prefix")]
    pub reverse_network_prefix: Option<String>,
    #[serde(default, rename = "role")]
    pub role: Option<String>,
    #[serde(default, rename = "serial")]
    pub serial: Option<i64>,
    #[serde(default, rename = "source")]
    pub source: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
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
    name: "dns_zone/show",
    method: runtime::Method::Get,
    path: "v7.0/dns_zones/{dns_zone_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dns_zone"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "dns_zone",
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
