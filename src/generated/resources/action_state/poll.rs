use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "current",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub current: runtime::InputValue<i64>,
    #[serde(
        rename = "status",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub status: runtime::InputValue<bool>,
    #[serde(
        rename = "timeout",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub timeout: runtime::InputValue<f64>,
    #[serde(
        rename = "total",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub total: runtime::InputValue<i64>,
    #[serde(
        rename = "update_in",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub update_in: runtime::InputValue<f64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.current)?;
        runtime::Validate::validate(&self.status)?;
        runtime::Validate::validate(&self.timeout)?;
        runtime::Validate::validate(&self.total)?;
        runtime::Validate::validate(&self.update_in)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "can_cancel")]
    pub can_cancel: Option<bool>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "current")]
    pub current: Option<i64>,
    #[serde(default, rename = "finished")]
    pub finished: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "status")]
    pub status: Option<bool>,
    #[serde(default, rename = "total")]
    pub total: Option<i64>,
    #[serde(default, rename = "unit")]
    pub unit: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
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
    name: "action_state/poll",
    method: runtime::Method::Get,
    path: "v7.0/action_states/{action_state_id}/poll",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Hash,
        namespace: Some("action_state"),
        parameters: &[
            runtime::ParameterSpec {
                name: "current",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "status",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "timeout",
                kind: runtime::ParameterKind::Float,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "total",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "update_in",
                kind: runtime::ParameterKind::Float,
                required: false,
                nullable: true,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Hash,
        namespace: "action_state",
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
    pub fn current(mut self, value: i64) -> Self {
        self.input_mut().current = runtime::InputValue::Value(value);
        self
    }
    pub fn current_null(mut self) -> Self {
        self.input_mut().current = runtime::InputValue::Null;
        self
    }
    pub fn status(mut self, value: bool) -> Self {
        self.input_mut().status = runtime::InputValue::Value(value);
        self
    }
    pub fn status_null(mut self) -> Self {
        self.input_mut().status = runtime::InputValue::Null;
        self
    }
    pub fn timeout(mut self, value: f64) -> Self {
        self.input_mut().timeout = runtime::InputValue::Value(value);
        self
    }
    pub fn total(mut self, value: i64) -> Self {
        self.input_mut().total = runtime::InputValue::Value(value);
        self
    }
    pub fn total_null(mut self) -> Self {
        self.input_mut().total = runtime::InputValue::Null;
        self
    }
    pub fn update_in(mut self, value: f64) -> Self {
        self.input_mut().update_in = runtime::InputValue::Value(value);
        self
    }
    pub fn update_in_null(mut self) -> Self {
        self.input_mut().update_in = runtime::InputValue::Null;
        self
    }
}
