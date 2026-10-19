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
    #[serde(default, rename = "applied_at")]
    pub applied_at: Option<String>,
    #[serde(default, rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "kernel_version")]
    pub kernel_version: Option<String>,
    #[serde(default, rename = "livepatch_id")]
    pub livepatch_id: Option<String>,
    #[serde(default, rename = "loaded")]
    pub loaded: Option<bool>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "node_kernel_evidence")]
    pub node_kernel_evidence: Option<runtime::Association>,
    #[serde(default, rename = "observed_at")]
    pub observed_at: Option<String>,
    #[serde(default, rename = "patch_version")]
    pub patch_version: Option<String>,
    #[serde(default, rename = "source")]
    pub source: Option<String>,
    #[serde(default, rename = "source_revision")]
    pub source_revision: Option<String>,
    #[serde(default, rename = "transition")]
    pub transition: Option<bool>,
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
    name: "node_kernel_livepatch/show",
    method: runtime::Method::Get,
    path: "v7.0/node_kernel_livepatches/{node_kernel_livepatch_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node_kernel_livepatch"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "node_kernel_livepatch",
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
