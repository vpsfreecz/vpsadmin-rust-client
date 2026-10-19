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
    #[serde(default, rename = "ipv4_count")]
    pub ipv4_count: Option<i64>,
    #[serde(default, rename = "ipv4_used")]
    pub ipv4_used: Option<i64>,
    #[serde(default, rename = "node_count")]
    pub node_count: Option<i64>,
    #[serde(default, rename = "nodes_online")]
    pub nodes_online: Option<i64>,
    #[serde(default, rename = "user_active")]
    pub user_active: Option<i64>,
    #[serde(default, rename = "user_count")]
    pub user_count: Option<i64>,
    #[serde(default, rename = "user_deleted")]
    pub user_deleted: Option<i64>,
    #[serde(default, rename = "user_suspended")]
    pub user_suspended: Option<i64>,
    #[serde(default, rename = "vps_count")]
    pub vps_count: Option<i64>,
    #[serde(default, rename = "vps_deleted")]
    pub vps_deleted: Option<i64>,
    #[serde(default, rename = "vps_running")]
    pub vps_running: Option<i64>,
    #[serde(default, rename = "vps_stopped")]
    pub vps_stopped: Option<i64>,
    #[serde(default, rename = "vps_suspended")]
    pub vps_suspended: Option<i64>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "cluster/full_stats",
    method: runtime::Method::Get,
    path: "v7.0/cluster/full_stats",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("cluster"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Hash,
        namespace: "cluster",
        allow_empty_response: false,
    }),
    meta_namespace: Some("_meta"),
    global_meta: &[runtime::ParameterSpec {
        name: "no",
        kind: runtime::ParameterKind::Boolean,
        required: false,
        nullable: false,
        protected: false,
    }],
    object_meta: &[],
    auth: runtime::RequestAuth::Configured,
};
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {}
