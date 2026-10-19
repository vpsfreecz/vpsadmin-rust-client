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
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "dns_server_zone")]
    pub dns_server_zone: Option<runtime::Association>,
    #[serde(default, rename = "event_at")]
    pub event_at: Option<String>,
    #[serde(default, rename = "event_key")]
    pub event_key: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "message")]
    pub message: Option<String>,
    #[serde(default, rename = "primary_addr")]
    pub primary_addr: Option<String>,
    #[serde(default, rename = "raw_message")]
    pub raw_message: Option<String>,
    #[serde(default, rename = "reason")]
    pub reason: Option<String>,
    #[serde(default, rename = "reason_code")]
    pub reason_code: Option<String>,
    #[serde(default, rename = "serial")]
    pub serial: Option<i64>,
    #[serde(default, rename = "source_cursor")]
    pub source_cursor: Option<String>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
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
    name: "dns_server_zone_transfer_log/show",
    method: runtime::Method::Get,
    path: "v7.0/dns_server_zone_transfer_logs/{dns_server_zone_transfer_log_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dns_server_zone_transfer_log"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "dns_server_zone_transfer_log",
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
