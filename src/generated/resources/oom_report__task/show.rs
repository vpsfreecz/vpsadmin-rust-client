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
    #[serde(default, rename = "host_pid")]
    pub host_pid: Option<i64>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "oom_score_adj")]
    pub oom_score_adj: Option<i64>,
    #[serde(default, rename = "pgtables_bytes")]
    pub pgtables_bytes: Option<i64>,
    #[serde(default, rename = "rss")]
    pub rss: Option<i64>,
    #[serde(default, rename = "rss_anon")]
    pub rss_anon: Option<i64>,
    #[serde(default, rename = "rss_file")]
    pub rss_file: Option<i64>,
    #[serde(default, rename = "rss_shmem")]
    pub rss_shmem: Option<i64>,
    #[serde(default, rename = "swapents")]
    pub swapents: Option<i64>,
    #[serde(default, rename = "tgid")]
    pub tgid: Option<i64>,
    #[serde(default, rename = "total_vm")]
    pub total_vm: Option<i64>,
    #[serde(default, rename = "vps_pid")]
    pub vps_pid: Option<i64>,
    #[serde(default, rename = "vps_uid")]
    pub vps_uid: Option<i64>,
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
    name: "oom_report/task/show",
    method: runtime::Method::Get,
    path: "v7.0/oom_reports/{oom_report_id}/tasks/{task_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("task"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "task",
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
