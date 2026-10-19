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
    #[serde(default, rename = "boot_id")]
    pub boot_id: Option<String>,
    #[serde(default, rename = "booted_at")]
    pub booted_at: Option<String>,
    #[serde(default, rename = "booted_release")]
    pub booted_release: Option<String>,
    #[serde(default, rename = "booted_system")]
    pub booted_system: Option<String>,
    #[serde(default, rename = "current_system")]
    pub current_system: Option<String>,
    #[serde(default, rename = "evidence_revision")]
    pub evidence_revision: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "kernel_command_line")]
    pub kernel_command_line: Option<String>,
    #[serde(default, rename = "kernel_config_available")]
    pub kernel_config_available: Option<bool>,
    #[serde(default, rename = "kernel_config_digest")]
    pub kernel_config_digest: Option<String>,
    #[serde(default, rename = "kernel_source_revision")]
    pub kernel_source_revision: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[serde(default, rename = "observed_at")]
    pub observed_at: Option<String>,
    #[serde(default, rename = "received_at")]
    pub received_at: Option<String>,
    #[serde(default, rename = "report_schema_version")]
    pub report_schema_version: Option<i64>,
    #[serde(default, rename = "reported_release")]
    pub reported_release: Option<String>,
    #[serde(default, rename = "snapshot_revision")]
    pub snapshot_revision: Option<String>,
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
    name: "node_kernel_evidence/show",
    method: runtime::Method::Get,
    path: "v7.0/node_kernel_evidences/{node_kernel_evidence_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node_kernel_evidence"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "node_kernel_evidence",
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
