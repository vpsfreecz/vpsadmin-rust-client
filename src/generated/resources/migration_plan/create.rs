use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "concurrency",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub concurrency: runtime::InputValue<i64>,
    #[serde(
        rename = "reason",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reason: runtime::InputValue<String>,
    #[serde(
        rename = "send_mail",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub send_mail: runtime::InputValue<bool>,
    #[serde(
        rename = "stop_on_error",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub stop_on_error: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.concurrency)?;
        runtime::Validate::validate(&self.reason)?;
        runtime::Validate::validate(&self.send_mail)?;
        runtime::Validate::validate(&self.stop_on_error)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "concurrency")]
    pub concurrency: Option<i64>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "finished_at")]
    pub finished_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "reason")]
    pub reason: Option<String>,
    #[serde(default, rename = "send_mail")]
    pub send_mail: Option<bool>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "stop_on_error")]
    pub stop_on_error: Option<bool>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "migration_plan/create",
    method: runtime::Method::Post,
    path: "v7.0/migration_plans",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("migration_plan"),
        parameters: &[
            runtime::ParameterSpec {
                name: "concurrency",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "reason",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "send_mail",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "stop_on_error",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "migration_plan",
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
    pub fn concurrency(mut self, value: i64) -> Self {
        self.input_mut().concurrency = runtime::InputValue::Value(value);
        self
    }
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reason = runtime::InputValue::Value(value.into());
        self
    }
    pub fn send_mail(mut self, value: bool) -> Self {
        self.input_mut().send_mail = runtime::InputValue::Value(value);
        self
    }
    pub fn stop_on_error(mut self, value: bool) -> Self {
        self.input_mut().stop_on_error = runtime::InputValue::Value(value);
        self
    }
}
