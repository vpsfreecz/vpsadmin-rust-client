use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "content",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub content: runtime::InputValue<String>,
    #[serde(
        rename = "format",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub format: runtime::InputValue<String>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.content)?;
        runtime::Validate::validate(&self.format)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.user)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "content")]
    pub content: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "format")]
    pub format: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
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
    name: "vps_user_data/create",
    method: runtime::Method::Post,
    path: "v7.0/vps_user_data",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("vps_user_data"),
        parameters: &[
            runtime::ParameterSpec {
                name: "content",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "format",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "label",
                kind: runtime::ParameterKind::String,
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
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "vps_user_data",
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
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.input_mut().content = runtime::InputValue::Value(value.into());
        self
    }
    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.input_mut().format = runtime::InputValue::Value(value.into());
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
}
