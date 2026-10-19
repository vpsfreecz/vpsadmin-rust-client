use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cs_message",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cs_message: runtime::InputValue<String>,
    #[serde(
        rename = "en_message",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub en_message: runtime::InputValue<String>,
    #[serde(
        rename = "published_at",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub published_at: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cs_message)?;
        runtime::Validate::validate(&self.en_message)?;
        runtime::Validate::validate(&self.published_at)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "cs_message")]
    pub cs_message: Option<String>,
    #[serde(default, rename = "en_message")]
    pub en_message: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "message")]
    pub message: Option<String>,
    #[serde(default, rename = "published_at")]
    pub published_at: Option<String>,
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
    name: "news_log/create",
    method: runtime::Method::Post,
    path: "v7.0/news_logs",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("news_log"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cs_message",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "en_message",
                kind: runtime::ParameterKind::Text,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "published_at",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "news_log",
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
    pub fn cs_message(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cs_message = runtime::InputValue::Value(value.into());
        self
    }
    pub fn en_message(mut self, value: impl Into<String>) -> Self {
        self.input_mut().en_message = runtime::InputValue::Value(value.into());
        self
    }
    pub fn published_at(mut self, value: impl Into<String>) -> Self {
        self.input_mut().published_at = runtime::InputValue::Value(value.into());
        self
    }
}
