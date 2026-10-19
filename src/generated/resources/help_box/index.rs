use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "action",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub action: runtime::InputValue<String>,
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "language",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub language: runtime::InputValue<i64>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(rename = "page", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub page: runtime::InputValue<String>,
    #[serde(rename = "view", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub view: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.action)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.language)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.page)?;
        runtime::Validate::validate(&self.view)?;
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "help_box/index",
    method: runtime::Method::Get,
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
                name: "from_id",
                kind: runtime::ParameterKind::Integer,
                required: false,
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
                name: "limit",
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
            runtime::ParameterSpec {
                name: "view",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "help_boxes",
        allow_empty_response: false,
    }),
    meta_namespace: Some("_meta"),
    global_meta: &[
        runtime::ParameterSpec {
            name: "count",
            kind: runtime::ParameterKind::Boolean,
            required: false,
            nullable: false,
            protected: false,
        },
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Vec<Output>, Meta> {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.input_mut().action = runtime::InputValue::Value(value.into());
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn language(mut self, value: i64) -> Self {
        self.input_mut().language = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn page(mut self, value: impl Into<String>) -> Self {
        self.input_mut().page = runtime::InputValue::Value(value.into());
        self
    }
    pub fn view(mut self, value: bool) -> Self {
        self.input_mut().view = runtime::InputValue::Value(value);
        self
    }
}
