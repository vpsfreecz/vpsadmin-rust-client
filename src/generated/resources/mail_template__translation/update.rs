use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "from", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub from: runtime::InputValue<String>,
    #[serde(
        rename = "language",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub language: runtime::InputValue<i64>,
    #[serde(
        rename = "reply_to",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reply_to: runtime::InputValue<String>,
    #[serde(
        rename = "return_path",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub return_path: runtime::InputValue<String>,
    #[serde(
        rename = "subject",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub subject: runtime::InputValue<String>,
    #[serde(
        rename = "text_html",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub text_html: runtime::InputValue<String>,
    #[serde(
        rename = "text_plain",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub text_plain: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.from)?;
        runtime::Validate::validate(&self.language)?;
        runtime::Validate::validate(&self.reply_to)?;
        runtime::Validate::validate(&self.return_path)?;
        runtime::Validate::validate(&self.subject)?;
        runtime::Validate::validate(&self.text_html)?;
        runtime::Validate::validate(&self.text_plain)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "from")]
    pub from: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "language")]
    pub language: Option<runtime::Association>,
    #[serde(default, rename = "reply_to")]
    pub reply_to: Option<String>,
    #[serde(default, rename = "return_path")]
    pub return_path: Option<String>,
    #[serde(default, rename = "subject")]
    pub subject: Option<String>,
    #[serde(default, rename = "text_html")]
    pub text_html: Option<String>,
    #[serde(default, rename = "text_plain")]
    pub text_plain: Option<String>,
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
    name: "mail_template/translation/update",
    method: runtime::Method::Put,
    path: "v7.0/mail_templates/{mail_template_id}/translations/{translation_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("translation"),
        parameters: &[
            runtime::ParameterSpec {
                name: "from",
                kind: runtime::ParameterKind::String,
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
                name: "reply_to",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "return_path",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "subject",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "text_html",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "text_plain",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: true,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "translation",
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
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.input_mut().from = runtime::InputValue::Value(value.into());
        self
    }
    pub fn language(mut self, value: i64) -> Self {
        self.input_mut().language = runtime::InputValue::Value(value);
        self
    }
    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reply_to = runtime::InputValue::Value(value.into());
        self
    }
    pub fn reply_to_null(mut self) -> Self {
        self.input_mut().reply_to = runtime::InputValue::Null;
        self
    }
    pub fn return_path(mut self, value: impl Into<String>) -> Self {
        self.input_mut().return_path = runtime::InputValue::Value(value.into());
        self
    }
    pub fn return_path_null(mut self) -> Self {
        self.input_mut().return_path = runtime::InputValue::Null;
        self
    }
    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.input_mut().subject = runtime::InputValue::Value(value.into());
        self
    }
    pub fn text_html(mut self, value: impl Into<String>) -> Self {
        self.input_mut().text_html = runtime::InputValue::Value(value.into());
        self
    }
    pub fn text_html_null(mut self) -> Self {
        self.input_mut().text_html = runtime::InputValue::Null;
        self
    }
    pub fn text_plain(mut self, value: impl Into<String>) -> Self {
        self.input_mut().text_plain = runtime::InputValue::Value(value.into());
        self
    }
    pub fn text_plain_null(mut self) -> Self {
        self.input_mut().text_plain = runtime::InputValue::Null;
        self
    }
}
