use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(
        rename = "offset",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub offset: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.offset)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "credit_us")]
    pub credit_us: Option<i64>,
    #[serde(default, rename = "from_at")]
    pub from_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "measurement")]
    pub measurement: Option<String>,
    #[serde(default, rename = "peak_is_lower_bound")]
    pub peak_is_lower_bound: Option<bool>,
    #[serde(default, rename = "peak_unit")]
    pub peak_unit: Option<String>,
    #[serde(default, rename = "peak_value")]
    pub peak_value: Option<i64>,
    #[serde(default, rename = "to_at")]
    pub to_at: Option<String>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(default, rename = "available")]
    pub available: Option<bool>,
    #[serde(default, rename = "condition_measurement")]
    pub condition_measurement: Option<String>,
    #[serde(default, rename = "condition_operator")]
    pub condition_operator: Option<String>,
    #[serde(default, rename = "condition_unit")]
    pub condition_unit: Option<String>,
    #[serde(default, rename = "condition_value")]
    pub condition_value: Option<i64>,
    #[serde(default, rename = "credit_us")]
    pub credit_us: Option<i64>,
    #[serde(default, rename = "evaluated_at")]
    pub evaluated_at: Option<String>,
    #[serde(default, rename = "limit")]
    pub limit: Option<i64>,
    #[serde(default, rename = "next_offset")]
    pub next_offset: Option<i64>,
    #[serde(default, rename = "offset")]
    pub offset: Option<i64>,
    #[serde(default, rename = "qualification_interval_us")]
    pub qualification_interval_us: Option<i64>,
    #[serde(default, rename = "repeat_seconds")]
    pub repeat_seconds: Option<i64>,
    #[serde(default, rename = "representation")]
    pub representation: Option<String>,
    #[serde(default, rename = "signal")]
    pub signal: Option<String>,
    #[serde(default, rename = "threshold_seconds")]
    pub threshold_seconds: Option<i64>,
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(default, rename = "window_complete")]
    pub window_complete: Option<bool>,
    #[serde(default, rename = "window_from")]
    pub window_from: Option<String>,
    #[serde(default, rename = "window_seconds")]
    pub window_seconds: Option<i64>,
    #[serde(default, rename = "window_to")]
    pub window_to: Option<String>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "incident_report/evidence_interval/index",
    method: runtime::Method::Get,
    path: "v7.0/incident_reports/{incident_report_id}/evidence_intervals",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("evidence_interval"),
        parameters: &[
            runtime::ParameterSpec {
                name: "limit",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "offset",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "evidence_intervals",
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Vec<Output>, Meta> {
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn offset(mut self, value: i64) -> Self {
        self.input_mut().offset = runtime::InputValue::Value(value);
        self
    }
}
