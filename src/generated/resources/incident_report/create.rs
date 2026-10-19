use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "codename",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub codename: runtime::InputValue<String>,
    #[serde(
        rename = "cpu_limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cpu_limit: runtime::InputValue<i64>,
    #[serde(
        rename = "detected_at",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub detected_at: runtime::InputValue<String>,
    #[serde(
        rename = "ip_address_assignment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_address_assignment: runtime::InputValue<i64>,
    #[serde(
        rename = "subject",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub subject: runtime::InputValue<String>,
    #[serde(rename = "text", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub text: runtime::InputValue<String>,
    #[serde(rename = "vps", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub vps: runtime::InputValue<i64>,
    #[serde(
        rename = "vps_action",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub vps_action: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.codename)?;
        runtime::Validate::validate(&self.cpu_limit)?;
        runtime::Validate::validate(&self.detected_at)?;
        runtime::Validate::validate(&self.ip_address_assignment)?;
        runtime::Validate::validate(&self.subject)?;
        runtime::Validate::validate(&self.text)?;
        runtime::Validate::validate(&self.vps)?;
        runtime::Validate::validate(&self.vps_action)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "codename")]
    pub codename: Option<String>,
    #[serde(default, rename = "cpu_limit")]
    pub cpu_limit: Option<i64>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "detected_at")]
    pub detected_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "filed_by")]
    pub filed_by: Option<runtime::Association>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=ip_addr"]
    #[serde(default, rename = "ip_address_assignment")]
    pub ip_address_assignment: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "mailbox")]
    pub mailbox: Option<runtime::Association>,
    #[serde(default, rename = "raw_user_id")]
    pub raw_user_id: Option<i64>,
    #[serde(default, rename = "raw_vps_id")]
    pub raw_vps_id: Option<i64>,
    #[serde(default, rename = "reported_at")]
    pub reported_at: Option<String>,
    #[serde(default, rename = "subject")]
    pub subject: Option<String>,
    #[serde(default, rename = "text")]
    pub text: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=hostname"]
    #[serde(default, rename = "vps")]
    pub vps: Option<runtime::Association>,
    #[serde(default, rename = "vps_action")]
    pub vps_action: Option<String>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(default, rename = "action_state_id")]
    pub action_state_id: Option<i64>,
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "incident_report/create",
    method: runtime::Method::Post,
    path: "v7.0/incident_reports",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("incident_report"),
        parameters: &[
            runtime::ParameterSpec {
                name: "codename",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "cpu_limit",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "detected_at",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ip_address_assignment",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "subject",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "text",
                kind: runtime::ParameterKind::Text,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps_action",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "incident_report",
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {
    pub fn codename(mut self, value: impl Into<String>) -> Self {
        self.input_mut().codename = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cpu_limit(mut self, value: i64) -> Self {
        self.input_mut().cpu_limit = runtime::InputValue::Value(value);
        self
    }
    pub fn detected_at(mut self, value: impl Into<String>) -> Self {
        self.input_mut().detected_at = runtime::InputValue::Value(value.into());
        self
    }
    pub fn ip_address_assignment(mut self, value: i64) -> Self {
        self.input_mut().ip_address_assignment = runtime::InputValue::Value(value);
        self
    }
    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.input_mut().subject = runtime::InputValue::Value(value.into());
        self
    }
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.input_mut().text = runtime::InputValue::Value(value.into());
        self
    }
    pub fn vps(mut self, value: i64) -> Self {
        self.input_mut().vps = runtime::InputValue::Value(value);
        self
    }
    pub fn vps_action(mut self, value: impl Into<String>) -> Self {
        self.input_mut().vps_action = runtime::InputValue::Value(value.into());
        self
    }
}
