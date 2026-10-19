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
    #[serde(default, rename = "arc_c")]
    pub arc_c: Option<i64>,
    #[serde(default, rename = "arc_c_max")]
    pub arc_c_max: Option<i64>,
    #[serde(default, rename = "arc_hitpercent")]
    pub arc_hitpercent: Option<f64>,
    #[serde(default, rename = "arc_size")]
    pub arc_size: Option<i64>,
    #[serde(default, rename = "cgroup_version")]
    pub cgroup_version: Option<String>,
    #[serde(default, rename = "cpu_guest")]
    pub cpu_guest: Option<f64>,
    #[serde(default, rename = "cpu_idle")]
    pub cpu_idle: Option<f64>,
    #[serde(default, rename = "cpu_iowait")]
    pub cpu_iowait: Option<f64>,
    #[serde(default, rename = "cpu_irq")]
    pub cpu_irq: Option<f64>,
    #[serde(default, rename = "cpu_nice")]
    pub cpu_nice: Option<f64>,
    #[serde(default, rename = "cpu_softirq")]
    pub cpu_softirq: Option<f64>,
    #[serde(default, rename = "cpu_system")]
    pub cpu_system: Option<f64>,
    #[serde(default, rename = "cpu_user")]
    pub cpu_user: Option<f64>,
    #[serde(default, rename = "cpus")]
    pub cpus: Option<i64>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "kernel")]
    pub kernel: Option<String>,
    #[serde(default, rename = "loadavg1")]
    pub loadavg1: Option<f64>,
    #[serde(default, rename = "loadavg15")]
    pub loadavg15: Option<f64>,
    #[serde(default, rename = "loadavg5")]
    pub loadavg5: Option<f64>,
    #[serde(default, rename = "process_count")]
    pub process_count: Option<i64>,
    #[serde(default, rename = "total_memory")]
    pub total_memory: Option<i64>,
    #[serde(default, rename = "total_swap")]
    pub total_swap: Option<i64>,
    #[serde(default, rename = "uptime")]
    pub uptime: Option<i64>,
    #[serde(default, rename = "used_memory")]
    pub used_memory: Option<i64>,
    #[serde(default, rename = "used_swap")]
    pub used_swap: Option<i64>,
    #[serde(default, rename = "version")]
    pub version: Option<String>,
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
    name: "node/status/show",
    method: runtime::Method::Get,
    path: "v7.0/nodes/{node_id}/statuses/{status_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("status"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "status",
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
