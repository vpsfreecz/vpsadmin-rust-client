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
    #[serde(default, rename = "active")]
    pub active: Option<bool>,
    #[serde(default, rename = "attached_at")]
    pub attached_at: Option<String>,
    #[serde(default, rename = "description")]
    pub description: Option<String>,
    #[serde(default, rename = "digest")]
    pub digest: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "node_kernel_evidence")]
    pub node_kernel_evidence: Option<runtime::Association>,
    #[serde(default, rename = "observed_at")]
    pub observed_at: Option<String>,
    #[serde(default, rename = "revision")]
    pub revision: Option<String>,
    #[serde(default, rename = "since_kernel")]
    pub since_kernel: Option<String>,
    #[serde(default, rename = "source")]
    pub source: Option<String>,
    #[serde(default, rename = "source_revision")]
    pub source_revision: Option<String>,
    #[serde(default, rename = "until_kernel")]
    pub until_kernel: Option<String>,
    #[serde(default, rename = "verified_at")]
    pub verified_at: Option<String>,
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
    name: "node_ebpf_program/show",
    method: runtime::Method::Get,
    path: "v7.0/node_ebpf_programs/{node_ebpf_program_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node_ebpf_program"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "node_ebpf_program",
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
