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
    #[serde(default, rename = "affected")]
    pub affected: Option<bool>,
    #[serde(default, rename = "affected_direct_vps_count")]
    pub affected_direct_vps_count: Option<i64>,
    #[serde(default, rename = "affected_export_count")]
    pub affected_export_count: Option<i64>,
    #[serde(default, rename = "affected_indirect_vps_count")]
    pub affected_indirect_vps_count: Option<i64>,
    #[serde(default, rename = "affected_user_count")]
    pub affected_user_count: Option<i64>,
    #[serde(default, rename = "auto_resolve")]
    pub auto_resolve: Option<bool>,
    #[serde(default, rename = "begins_at")]
    pub begins_at: Option<String>,
    #[serde(default, rename = "cs_description")]
    pub cs_description: Option<String>,
    #[serde(default, rename = "cs_summary")]
    pub cs_summary: Option<String>,
    #[serde(default, rename = "duration")]
    pub duration: Option<i64>,
    #[serde(default, rename = "en_description")]
    pub en_description: Option<String>,
    #[serde(default, rename = "en_summary")]
    pub en_summary: Option<String>,
    #[serde(default, rename = "finished_at")]
    pub finished_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "impact")]
    pub impact: Option<String>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
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
    name: "outage/show",
    method: runtime::Method::Get,
    path: "v7.0/outages/{outage_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("outage"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "outage",
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
