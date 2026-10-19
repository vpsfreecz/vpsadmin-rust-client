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
    #[serde(default, rename = "bytes")]
    pub bytes: Option<i64>,
    #[serde(default, rename = "bytes_in")]
    pub bytes_in: Option<i64>,
    #[serde(default, rename = "bytes_out")]
    pub bytes_out: Option<i64>,
    #[serde(default, rename = "delta")]
    pub delta: Option<i64>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "network_interface")]
    pub network_interface: Option<runtime::Association>,
    #[serde(default, rename = "packets")]
    pub packets: Option<i64>,
    #[serde(default, rename = "packets_in")]
    pub packets_in: Option<i64>,
    #[serde(default, rename = "packets_out")]
    pub packets_out: Option<i64>,
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
    name: "network_interface_monitor/show",
    method: runtime::Method::Get,
    path: "v7.0/network_interface_monitors/{network_interface_monitor_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("network_interface_monitor"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "network_interface_monitor",
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
