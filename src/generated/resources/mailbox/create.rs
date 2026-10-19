use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "enable_ssl",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_ssl: runtime::InputValue<bool>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(
        rename = "password",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub password: runtime::InputValue<String>,
    #[serde(rename = "port", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub port: runtime::InputValue<i64>,
    #[serde(
        rename = "server",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub server: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.enable_ssl)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.password)?;
        runtime::Validate::validate(&self.port)?;
        runtime::Validate::validate(&self.server)?;
        runtime::Validate::validate(&self.user)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "enable_ssl")]
    pub enable_ssl: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "port")]
    pub port: Option<i64>,
    #[serde(default, rename = "server")]
    pub server: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(default, rename = "user")]
    pub user: Option<String>,
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
    name: "mailbox/create",
    method: runtime::Method::Post,
    path: "v7.0/mailboxes",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("mailbox"),
        parameters: &[
            runtime::ParameterSpec {
                name: "enable_ssl",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "label",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "password",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "port",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "server",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "mailbox",
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
    pub fn enable_ssl(mut self, value: bool) -> Self {
        self.input_mut().enable_ssl = runtime::InputValue::Value(value);
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.input_mut().password = runtime::InputValue::Value(value.into());
        self
    }
    pub fn port(mut self, value: i64) -> Self {
        self.input_mut().port = runtime::InputValue::Value(value);
        self
    }
    pub fn server(mut self, value: impl Into<String>) -> Self {
        self.input_mut().server = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value.into());
        self
    }
}
