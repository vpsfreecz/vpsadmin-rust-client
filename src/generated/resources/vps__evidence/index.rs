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
    #[serde(default, rename = "condition_measurement")]
    pub condition_measurement: Option<String>,
    #[serde(default, rename = "condition_operator")]
    pub condition_operator: Option<String>,
    #[serde(default, rename = "condition_unit")]
    pub condition_unit: Option<String>,
    #[serde(default, rename = "condition_value")]
    pub condition_value: Option<i64>,
    #[serde(default, rename = "configuration_valid")]
    pub configuration_valid: Option<bool>,
    #[serde(default, rename = "credit_us")]
    pub credit_us: Option<i64>,
    #[serde(default, rename = "hostname")]
    pub hostname: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<String>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "last_incident_at")]
    pub last_incident_at: Option<String>,
    #[serde(default, rename = "last_observed_at")]
    pub last_observed_at: Option<String>,
    #[serde(default, rename = "measurement")]
    pub measurement: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[serde(default, rename = "observation_age_seconds")]
    pub observation_age_seconds: Option<f64>,
    #[serde(default, rename = "observation_state")]
    pub observation_state: Option<String>,
    #[serde(default, rename = "peak_is_lower_bound")]
    pub peak_is_lower_bound: Option<bool>,
    #[serde(default, rename = "peak_unit")]
    pub peak_unit: Option<String>,
    #[serde(default, rename = "peak_value")]
    pub peak_value: Option<i64>,
    #[serde(default, rename = "qualification")]
    pub qualification: Option<String>,
    #[serde(default, rename = "repeat_seconds")]
    pub repeat_seconds: Option<i64>,
    #[serde(default, rename = "report_due")]
    pub report_due: Option<bool>,
    #[serde(default, rename = "report_eligible")]
    pub report_eligible: Option<bool>,
    #[serde(default, rename = "reporting_enabled")]
    pub reporting_enabled: Option<bool>,
    #[serde(default, rename = "representation")]
    pub representation: Option<String>,
    #[serde(default, rename = "signal")]
    pub signal: Option<String>,
    #[serde(default, rename = "threshold_seconds")]
    pub threshold_seconds: Option<i64>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=hostname"]
    #[serde(default, rename = "vps")]
    pub vps: Option<runtime::Association>,
    #[serde(default, rename = "window_complete")]
    pub window_complete: Option<bool>,
    #[serde(default, rename = "window_from")]
    pub window_from: Option<String>,
    #[serde(default, rename = "window_seconds")]
    pub window_seconds: Option<i64>,
    #[serde(default, rename = "window_to")]
    pub window_to: Option<String>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(default, rename = "configuration_valid")]
    pub configuration_valid: Option<bool>,
    #[serde(default, rename = "read_at")]
    pub read_at: Option<String>,
    #[serde(default, rename = "unavailable_signals")]
    pub unavailable_signals: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "vps/evidence/index",
    method: runtime::Method::Get,
    path: "v7.0/vpses/{vps_id}/evidence",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("evidence"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "evidence",
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
