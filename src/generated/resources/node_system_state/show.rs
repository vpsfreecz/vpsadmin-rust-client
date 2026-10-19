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
    #[serde(default, rename = "cgroup_version")]
    pub cgroup_version: Option<String>,
    #[serde(default, rename = "cpus")]
    pub cpus: Option<i64>,
    #[serde(default, rename = "current")]
    pub current: Option<bool>,
    #[serde(default, rename = "first_observed_at")]
    pub first_observed_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "last_observed_at")]
    pub last_observed_at: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[serde(default, rename = "total_memory")]
    pub total_memory: Option<i64>,
    #[serde(default, rename = "total_swap")]
    pub total_swap: Option<i64>,
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
    name: "node_system_state/show",
    method: runtime::Method::Get,
    path: "v7.0/node_system_states/{node_system_state_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node_system_state"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "node_system_state",
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
