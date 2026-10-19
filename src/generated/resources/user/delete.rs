use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "change_reason",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub change_reason: runtime::InputValue<String>,
    #[serde(
        rename = "expiration_date",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub expiration_date: runtime::InputValue<String>,
    #[serde(
        rename = "object_state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub object_state: runtime::InputValue<String>,
    #[serde(
        rename = "remind_after_date",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub remind_after_date: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.change_reason)?;
        runtime::Validate::validate(&self.expiration_date)?;
        runtime::Validate::validate(&self.object_state)?;
        runtime::Validate::validate(&self.remind_after_date)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
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
    name: "user/delete",
    method: runtime::Method::Delete,
    path: "v7.0/users/{user_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("user"),
        parameters: &[
            runtime::ParameterSpec {
                name: "change_reason",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "expiration_date",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "object_state",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "remind_after_date",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "user",
        allow_empty_response: true,
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
    pub fn change_reason(mut self, value: impl Into<String>) -> Self {
        self.input_mut().change_reason = runtime::InputValue::Value(value.into());
        self
    }
    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.input_mut().expiration_date = runtime::InputValue::Value(value.into());
        self
    }
    pub fn expiration_date_null(mut self) -> Self {
        self.input_mut().expiration_date = runtime::InputValue::Null;
        self
    }
    pub fn object_state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().object_state = runtime::InputValue::Value(value.into());
        self
    }
    pub fn remind_after_date(mut self, value: impl Into<String>) -> Self {
        self.input_mut().remind_after_date = runtime::InputValue::Value(value.into());
        self
    }
}
