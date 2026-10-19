use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "action",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub action: runtime::InputValue<String>,
    #[serde(
        rename = "content",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub content: runtime::InputValue<String>,
    #[serde(
        rename = "language",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub language: runtime::InputValue<i64>,
    #[serde(
        rename = "order",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub order: runtime::InputValue<i64>,
    #[serde(rename = "page", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub page: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.action)?;
        runtime::Validate::validate(&self.content)?;
        runtime::Validate::validate(&self.language)?;
        runtime::Validate::validate(&self.order)?;
        runtime::Validate::validate(&self.page)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "action")]
    pub action: Option<String>,
    #[serde(default, rename = "content")]
    pub content: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "language")]
    pub language: Option<runtime::Association>,
    #[serde(default, rename = "order")]
    pub order: Option<i64>,
    #[serde(default, rename = "page")]
    pub page: Option<String>,
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
    name: "help_box/create",
    method: runtime::Method::Post,
    path: "v7.0/help_boxes",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("help_box"),
        parameters: &[
            runtime::ParameterSpec {
                name: "action",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "content",
                kind: runtime::ParameterKind::Text,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "language",
                kind: runtime::ParameterKind::Resource,
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
            runtime::ParameterSpec {
                name: "page",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "help_box",
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
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.input_mut().action = runtime::InputValue::Value(value.into());
        self
    }
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.input_mut().content = runtime::InputValue::Value(value.into());
        self
    }
    pub fn language(mut self, value: i64) -> Self {
        self.input_mut().language = runtime::InputValue::Value(value);
        self
    }
    pub fn order(mut self, value: i64) -> Self {
        self.input_mut().order = runtime::InputValue::Value(value);
        self
    }
    pub fn page(mut self, value: impl Into<String>) -> Self {
        self.input_mut().page = runtime::InputValue::Value(value.into());
        self
    }
}
