use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "as_of",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub as_of: runtime::InputValue<String>,
    #[serde(
        rename = "direction",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub direction: runtime::InputValue<String>,
    #[serde(
        rename = "effective_mode",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub effective_mode: runtime::InputValue<String>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
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
    #[serde(rename = "sort", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub sort: runtime::InputValue<String>,
    #[serde(
        rename = "status",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub status: runtime::InputValue<String>,
    #[serde(
        rename = "stop_enforced",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub stop_enforced: runtime::InputValue<bool>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
    #[serde(rename = "vps", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub vps: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.as_of)?;
        runtime::Validate::validate(&self.direction)?;
        runtime::Validate::validate(&self.effective_mode)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.offset)?;
        runtime::Validate::validate(&self.policy_key)?;
        runtime::Validate::validate(&self.sort)?;
        runtime::Validate::validate(&self.status)?;
        runtime::Validate::validate(&self.stop_enforced)?;
        runtime::Validate::validate(&self.user)?;
        runtime::Validate::validate(&self.vps)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "accounting_since")]
    pub accounting_since: Option<String>,
    #[serde(default, rename = "configuration_valid")]
    pub configuration_valid: Option<bool>,
    #[serde(default, rename = "count_30d")]
    pub count_30d: Option<i64>,
    #[serde(default, rename = "count_7d")]
    pub count_7d: Option<i64>,
    #[serde(default, rename = "effective_mode")]
    pub effective_mode: Option<String>,
    #[serde(default, rename = "global_mode")]
    pub global_mode: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<String>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "last_incident_at")]
    pub last_incident_at: Option<String>,
    #[serde(default, rename = "last_stop_at")]
    pub last_stop_at: Option<String>,
    #[serde(default, rename = "last_warning_at")]
    pub last_warning_at: Option<String>,
    #[serde(default, rename = "latest_action_chain_id")]
    pub latest_action_chain_id: Option<i64>,
    #[serde(default, rename = "latest_action_id")]
    pub latest_action_id: Option<i64>,
    #[serde(default, rename = "latest_action_kind")]
    pub latest_action_kind: Option<String>,
    #[serde(default, rename = "latest_action_state")]
    pub latest_action_state: Option<String>,
    #[serde(default, rename = "latest_mail_state")]
    pub latest_mail_state: Option<String>,
    #[serde(default, rename = "latest_notice_report_id")]
    pub latest_notice_report_id: Option<i64>,
    #[serde(default, rename = "latest_stop_completed_at")]
    pub latest_stop_completed_at: Option<String>,
    #[serde(default, rename = "next_stop_allowed_at")]
    pub next_stop_allowed_at: Option<String>,
    #[serde(default, rename = "next_warning_allowed_at")]
    pub next_warning_allowed_at: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[serde(default, rename = "outstanding_stop_action_id")]
    pub outstanding_stop_action_id: Option<i64>,
    #[serde(default, rename = "outstanding_stop_chain_id")]
    pub outstanding_stop_chain_id: Option<i64>,
    #[serde(default, rename = "outstanding_stop_state")]
    pub outstanding_stop_state: Option<String>,
    #[serde(default, rename = "owner_active")]
    pub owner_active: Option<bool>,
    #[serde(default, rename = "policy_key")]
    pub policy_key: Option<String>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "stop_enforced")]
    pub stop_enforced: Option<bool>,
    #[serde(default, rename = "stop_limit")]
    pub stop_limit: Option<i64>,
    #[serde(default, rename = "stop_period_days")]
    pub stop_period_days: Option<i64>,
    #[serde(default, rename = "stop_remaining")]
    pub stop_remaining: Option<i64>,
    #[serde(default, rename = "transfer_pending")]
    pub transfer_pending: Option<bool>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=hostname"]
    #[serde(default, rename = "vps")]
    pub vps: Option<runtime::Association>,
    #[serde(default, rename = "warning_limit")]
    pub warning_limit: Option<i64>,
    #[serde(default, rename = "warning_period_days")]
    pub warning_period_days: Option<i64>,
    #[serde(default, rename = "warning_permitted")]
    pub warning_permitted: Option<bool>,
    #[serde(default, rename = "warning_remaining")]
    pub warning_remaining: Option<i64>,
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
    #[serde(default, rename = "limit")]
    pub limit: Option<i64>,
    #[serde(default, rename = "next_offset")]
    pub next_offset: Option<i64>,
    #[serde(default, rename = "offset")]
    pub offset: Option<i64>,
    #[serde(default, rename = "read_at")]
    pub read_at: Option<String>,
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "incident_policy_exposure/index",
    method: runtime::Method::Get,
    path: "v7.0/incident_policy_exposures",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("incident_policy_exposure"),
        parameters: &[
            runtime::ParameterSpec {
                name: "as_of",
                kind: runtime::ParameterKind::Datetime,
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
                name: "effective_mode",
                kind: runtime::ParameterKind::String,
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
                name: "node",
                kind: runtime::ParameterKind::Resource,
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
                name: "sort",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "status",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "stop_enforced",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "incident_policy_exposures",
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
    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.input_mut().as_of = runtime::InputValue::Value(value.into());
        self
    }
    pub fn direction(mut self, value: impl Into<String>) -> Self {
        self.input_mut().direction = runtime::InputValue::Value(value.into());
        self
    }
    pub fn effective_mode(mut self, value: impl Into<String>) -> Self {
        self.input_mut().effective_mode = runtime::InputValue::Value(value.into());
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
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
    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.input_mut().sort = runtime::InputValue::Value(value.into());
        self
    }
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.input_mut().status = runtime::InputValue::Value(value.into());
        self
    }
    pub fn stop_enforced(mut self, value: bool) -> Self {
        self.input_mut().stop_enforced = runtime::InputValue::Value(value);
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
    pub fn vps(mut self, value: i64) -> Self {
        self.input_mut().vps = runtime::InputValue::Value(value);
        self
    }
}
