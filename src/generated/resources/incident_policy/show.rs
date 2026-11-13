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
    #[serde(default, rename = "description")]
    pub description: Option<String>,
    #[serde(default, rename = "key")]
    pub key: Option<String>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "settings_category")]
    pub settings_category: Option<String>,
    #[serde(default, rename = "stop_period_days")]
    pub stop_period_days: Option<i64>,
    #[serde(default, rename = "warning_period_days")]
    pub warning_period_days: Option<i64>,
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
    name: "incident_policy/show",
    method: runtime::Method::Get,
    path: "v7.0/incident_policies/{key}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("incident_policy"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "incident_policy",
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
