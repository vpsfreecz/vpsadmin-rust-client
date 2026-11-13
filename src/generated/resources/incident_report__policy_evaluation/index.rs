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
    #[serde(default, rename = "count_30d")]
    pub count_30d: Option<i64>,
    #[serde(default, rename = "count_7d")]
    pub count_7d: Option<i64>,
    #[serde(default, rename = "decision")]
    pub decision: Option<String>,
    #[serde(default, rename = "effective_mode")]
    pub effective_mode: Option<String>,
    #[serde(default, rename = "error_class")]
    pub error_class: Option<String>,
    #[serde(default, rename = "error_stage")]
    pub error_stage: Option<String>,
    #[serde(default, rename = "evaluated_at")]
    pub evaluated_at: Option<String>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "matched")]
    pub matched: Option<bool>,
    #[serde(default, rename = "policy_key")]
    pub policy_key: Option<String>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "stop_enforced")]
    pub stop_enforced: Option<bool>,
    #[serde(default, rename = "stop_limit")]
    pub stop_limit: Option<i64>,
    #[serde(default, rename = "stop_period_days")]
    pub stop_period_days: Option<i64>,
    #[serde(default, rename = "warning_limit")]
    pub warning_limit: Option<i64>,
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
    name: "incident_report/policy_evaluation/index",
    method: runtime::Method::Get,
    path: "v7.0/incident_reports/{incident_report_id}/policy_evaluations",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("policy_evaluation"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "policy_evaluations",
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Vec<Output>, Meta> {}
