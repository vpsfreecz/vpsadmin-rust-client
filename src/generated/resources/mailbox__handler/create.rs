use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "class_name",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub class_name: runtime::InputValue<String>,
    #[serde(
        rename = "continue",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub continue_: runtime::InputValue<bool>,
    #[serde(
        rename = "order",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub order: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.class_name)?;
        runtime::Validate::validate(&self.continue_)?;
        runtime::Validate::validate(&self.order)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "class_name")]
    pub class_name: Option<String>,
    #[serde(default, rename = "continue")]
    pub continue_: Option<bool>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "order")]
    pub order: Option<i64>,
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
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "mailbox/handler/create",
    method: runtime::Method::Post,
    path: "v7.0/mailboxes/{mailbox_id}/handler",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("handler"),
        parameters: &[
            runtime::ParameterSpec {
                name: "class_name",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "continue",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "order",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "handler",
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
    pub fn class_name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().class_name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn continue_(mut self, value: bool) -> Self {
        self.input_mut().continue_ = runtime::InputValue::Value(value);
        self
    }
    pub fn order(mut self, value: i64) -> Self {
        self.input_mut().order = runtime::InputValue::Value(value);
        self
    }
}
