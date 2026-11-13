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
    #[serde(default, rename = "configuration_valid")]
    pub configuration_valid: Option<bool>,
    #[serde(default, rename = "count_30d")]
    pub count_30d: Option<i64>,
    #[serde(default, rename = "count_7d")]
    pub count_7d: Option<i64>,
    #[serde(default, rename = "description")]
    pub description: Option<String>,
    #[serde(default, rename = "effective_mode")]
    pub effective_mode: Option<String>,
    #[serde(default, rename = "evaluated_at")]
    pub evaluated_at: Option<String>,
    #[serde(default, rename = "evaluation_report_id")]
    pub evaluation_report_id: Option<i64>,
    #[serde(default, rename = "evaluation_state")]
    pub evaluation_state: Option<String>,
    #[serde(default, rename = "global_mode")]
    pub global_mode: Option<String>,
    #[serde(default, rename = "key")]
    pub key: Option<String>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "last_incident_at")]
    pub last_incident_at: Option<String>,
    #[serde(default, rename = "last_warning_at")]
    pub last_warning_at: Option<String>,
    #[serde(default, rename = "settings_category")]
    pub settings_category: Option<String>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "stop_enforced")]
    pub stop_enforced: Option<bool>,
    #[serde(default, rename = "stop_limit")]
    pub stop_limit: Option<i64>,
    #[serde(default, rename = "stop_limit_override")]
    pub stop_limit_override: Option<i64>,
    #[serde(default, rename = "stop_period_days")]
    pub stop_period_days: Option<i64>,
    #[serde(default, rename = "warning_limit")]
    pub warning_limit: Option<i64>,
    #[serde(default, rename = "warning_limit_override")]
    pub warning_limit_override: Option<i64>,
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
    name: "vps/incident_policy/show",
    method: runtime::Method::Get,
    path: "v7.0/vpses/{vps_id}/incident_policies/{key}",
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
