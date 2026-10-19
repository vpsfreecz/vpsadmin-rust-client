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
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "dns_server")]
    pub dns_server: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "dns_zone")]
    pub dns_zone: Option<runtime::Association>,
    #[serde(default, rename = "expires_at")]
    pub expires_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "last_check_at")]
    pub last_check_at: Option<String>,
    #[serde(default, rename = "last_transfer_at")]
    pub last_transfer_at: Option<String>,
    #[serde(default, rename = "last_transfer_log_id")]
    pub last_transfer_log_id: Option<i64>,
    #[serde(default, rename = "last_transfer_primary_addr")]
    pub last_transfer_primary_addr: Option<String>,
    #[serde(default, rename = "last_transfer_reason")]
    pub last_transfer_reason: Option<String>,
    #[serde(default, rename = "last_transfer_reason_code")]
    pub last_transfer_reason_code: Option<String>,
    #[serde(default, rename = "last_transfer_serial")]
    pub last_transfer_serial: Option<i64>,
    #[serde(default, rename = "last_transfer_status")]
    pub last_transfer_status: Option<String>,
    #[serde(default, rename = "loaded_at")]
    pub loaded_at: Option<String>,
    #[serde(default, rename = "refresh_at")]
    pub refresh_at: Option<String>,
    #[serde(default, rename = "serial")]
    pub serial: Option<i64>,
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
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
    name: "dns_server_zone/show",
    method: runtime::Method::Get,
    path: "v7.0/dns_server_zones/{dns_server_zone_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dns_server_zone"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "dns_server_zone",
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
