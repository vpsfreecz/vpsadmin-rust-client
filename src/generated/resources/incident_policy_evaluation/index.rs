use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "action_kind",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub action_kind: runtime::InputValue<String>,
    #[serde(
        rename = "action_state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub action_state: runtime::InputValue<String>,
    #[serde(
        rename = "as_of",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub as_of: runtime::InputValue<String>,
    #[serde(
        rename = "decision",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub decision: runtime::InputValue<String>,
    #[serde(
        rename = "direction",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub direction: runtime::InputValue<String>,
    #[serde(rename = "from", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub from: runtime::InputValue<String>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(
        rename = "mail_state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub mail_state: runtime::InputValue<String>,
    #[serde(
        rename = "offset",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub offset: runtime::InputValue<i64>,
    #[serde(
        rename = "policy_key",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub policy_key: runtime::InputValue<String>,
    #[serde(
        rename = "scope",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub scope: runtime::InputValue<String>,
    #[serde(rename = "to", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub to: runtime::InputValue<String>,
    #[serde(
        rename = "user_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub user_id: runtime::InputValue<i64>,
    #[serde(
        rename = "vps_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub vps_id: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.action_kind)?;
        runtime::Validate::validate(&self.action_state)?;
        runtime::Validate::validate(&self.as_of)?;
        runtime::Validate::validate(&self.decision)?;
        runtime::Validate::validate(&self.direction)?;
        runtime::Validate::validate(&self.from)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.mail_state)?;
        runtime::Validate::validate(&self.offset)?;
        runtime::Validate::validate(&self.policy_key)?;
        runtime::Validate::validate(&self.scope)?;
        runtime::Validate::validate(&self.to)?;
        runtime::Validate::validate(&self.user_id)?;
        runtime::Validate::validate(&self.vps_id)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "action_created_at")]
    pub action_created_at: Option<String>,
    #[serde(default, rename = "action_id")]
    pub action_id: Option<i64>,
    #[serde(default, rename = "action_kind")]
    pub action_kind: Option<String>,
    #[serde(default, rename = "action_state")]
    pub action_state: Option<String>,
    #[serde(default, rename = "action_updated_at")]
    pub action_updated_at: Option<String>,
    #[serde(default, rename = "count_30d")]
    pub count_30d: Option<i64>,
    #[serde(default, rename = "count_7d")]
    pub count_7d: Option<i64>,
    #[serde(default, rename = "decision")]
    pub decision: Option<String>,
    #[serde(default, rename = "detected_at")]
    pub detected_at: Option<String>,
    #[serde(default, rename = "effective_mode")]
    pub effective_mode: Option<String>,
    #[serde(default, rename = "evaluated_at")]
    pub evaluated_at: Option<String>,
    #[serde(default, rename = "hostname")]
    pub hostname: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "incident_report_id")]
    pub incident_report_id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "mail_finished_at")]
    pub mail_finished_at: Option<String>,
    #[serde(default, rename = "mail_queued_at")]
    pub mail_queued_at: Option<String>,
    #[serde(default, rename = "mail_state")]
    pub mail_state: Option<String>,
    #[serde(default, rename = "matched")]
    pub matched: Option<bool>,
    #[serde(default, rename = "notice_report_id")]
    pub notice_report_id: Option<i64>,
    #[serde(default, rename = "physical_stop_action_id")]
    pub physical_stop_action_id: Option<i64>,
    #[serde(default, rename = "policy_key")]
    pub policy_key: Option<String>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "stop_completed_at")]
    pub stop_completed_at: Option<String>,
    #[serde(default, rename = "stop_enforced")]
    pub stop_enforced: Option<bool>,
    #[serde(default, rename = "stop_limit")]
    pub stop_limit: Option<i64>,
    #[serde(default, rename = "stop_period_days")]
    pub stop_period_days: Option<i64>,
    #[serde(default, rename = "threshold_status")]
    pub threshold_status: Option<String>,
    #[serde(default, rename = "transaction_chain_id")]
    pub transaction_chain_id: Option<i64>,
    #[serde(default, rename = "user_id")]
    pub user_id: Option<i64>,
    #[serde(default, rename = "user_login")]
    pub user_login: Option<String>,
    #[serde(default, rename = "vps_id")]
    pub vps_id: Option<i64>,
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
    #[serde(default, rename = "as_of")]
    pub as_of: Option<String>,
    #[serde(default, rename = "from")]
    pub from: Option<String>,
    #[serde(default, rename = "limit")]
    pub limit: Option<i64>,
    #[serde(default, rename = "next_offset")]
    pub next_offset: Option<i64>,
    #[serde(default, rename = "offset")]
    pub offset: Option<i64>,
    #[serde(default, rename = "read_at")]
    pub read_at: Option<String>,
    #[serde(default, rename = "to")]
    pub to: Option<String>,
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "incident_policy_evaluation/index",
    method: runtime::Method::Get,
    path: "v7.0/incident_policy_evaluations",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("incident_policy_evaluation"),
        parameters: &[
            runtime::ParameterSpec {
                name: "action_kind",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "action_state",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "as_of",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "decision",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "direction",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "from",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "limit",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "mail_state",
                kind: runtime::ParameterKind::String,
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
            runtime::ParameterSpec {
                name: "policy_key",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "scope",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "to",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user_id",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps_id",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "incident_policy_evaluations",
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
    pub fn action_kind(mut self, value: impl Into<String>) -> Self {
        self.input_mut().action_kind = runtime::InputValue::Value(value.into());
        self
    }
    pub fn action_state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().action_state = runtime::InputValue::Value(value.into());
        self
    }
    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.input_mut().as_of = runtime::InputValue::Value(value.into());
        self
    }
    pub fn decision(mut self, value: impl Into<String>) -> Self {
        self.input_mut().decision = runtime::InputValue::Value(value.into());
        self
    }
    pub fn direction(mut self, value: impl Into<String>) -> Self {
        self.input_mut().direction = runtime::InputValue::Value(value.into());
        self
    }
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.input_mut().from = runtime::InputValue::Value(value.into());
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn mail_state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().mail_state = runtime::InputValue::Value(value.into());
        self
    }
    pub fn offset(mut self, value: i64) -> Self {
        self.input_mut().offset = runtime::InputValue::Value(value);
        self
    }
    pub fn policy_key(mut self, value: impl Into<String>) -> Self {
        self.input_mut().policy_key = runtime::InputValue::Value(value.into());
        self
    }
    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.input_mut().scope = runtime::InputValue::Value(value.into());
        self
    }
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.input_mut().to = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user_id(mut self, value: i64) -> Self {
        self.input_mut().user_id = runtime::InputValue::Value(value);
        self
    }
    pub fn vps_id(mut self, value: i64) -> Self {
        self.input_mut().vps_id = runtime::InputValue::Value(value);
        self
    }
}
