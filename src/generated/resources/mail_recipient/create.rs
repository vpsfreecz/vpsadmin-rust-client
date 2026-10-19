use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "bcc", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub bcc: runtime::InputValue<String>,
    #[serde(rename = "cc", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub cc: runtime::InputValue<String>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(rename = "to", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub to: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.bcc)?;
        runtime::Validate::validate(&self.cc)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.to)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "bcc")]
    pub bcc: Option<String>,
    #[serde(default, rename = "cc")]
    pub cc: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "to")]
    pub to: Option<String>,
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
    name: "mail_recipient/create",
    method: runtime::Method::Post,
    path: "v7.0/mail_recipients",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("mail_recipient"),
        parameters: &[
            runtime::ParameterSpec {
                name: "bcc",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "cc",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: true,
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
                name: "to",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: true,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "mail_recipient",
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
    pub fn bcc(mut self, value: impl Into<String>) -> Self {
        self.input_mut().bcc = runtime::InputValue::Value(value.into());
        self
    }
    pub fn bcc_null(mut self) -> Self {
        self.input_mut().bcc = runtime::InputValue::Null;
        self
    }
    pub fn cc(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cc = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cc_null(mut self) -> Self {
        self.input_mut().cc = runtime::InputValue::Null;
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.input_mut().to = runtime::InputValue::Value(value.into());
        self
    }
    pub fn to_null(mut self) -> Self {
        self.input_mut().to = runtime::InputValue::Null;
        self
    }
}
