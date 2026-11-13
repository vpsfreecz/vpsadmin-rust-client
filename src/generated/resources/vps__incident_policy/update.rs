use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "stop_limit_override",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub stop_limit_override: runtime::InputValue<i64>,
    #[serde(
        rename = "warning_limit_override",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub warning_limit_override: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.stop_limit_override)?;
        runtime::Validate::validate(&self.warning_limit_override)?;
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
    name: "vps/incident_policy/update",
    method: runtime::Method::Put,
    path: "v7.0/vpses/{vps_id}/incident_policies/{key}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("incident_policy"),
        parameters: &[
            runtime::ParameterSpec {
                name: "stop_limit_override",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "warning_limit_override",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: true,
                protected: false,
            },
        ],
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {
    pub fn stop_limit_override(mut self, value: i64) -> Self {
        self.input_mut().stop_limit_override = runtime::InputValue::Value(value);
        self
    }
    pub fn stop_limit_override_null(mut self) -> Self {
        self.input_mut().stop_limit_override = runtime::InputValue::Null;
        self
    }
    pub fn warning_limit_override(mut self, value: i64) -> Self {
        self.input_mut().warning_limit_override = runtime::InputValue::Value(value);
        self
    }
    pub fn warning_limit_override_null(mut self) -> Self {
        self.input_mut().warning_limit_override = runtime::InputValue::Null;
        self
    }
}
