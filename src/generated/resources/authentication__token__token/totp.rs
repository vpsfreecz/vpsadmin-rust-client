use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "code", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub code: runtime::InputValue<String>,
    #[serde(
        rename = "token",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub token: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.code)?;
        runtime::Validate::validate(&self.token)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "complete")]
    pub complete: Option<bool>,
    #[serde(default, rename = "next_action")]
    pub next_action: Option<String>,
    #[serde(default, rename = "token")]
    pub token: Option<String>,
    #[serde(default, rename = "valid_to")]
    pub valid_to: Option<String>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
impl Output {
    pub fn token_step(&self) -> Result<runtime::TokenStep, runtime::Error> {
        // Missing complete is inconsistent; force the checked protocol-error path.
        let complete = match self.complete {
            Some(value) => value,
            None => return runtime::TokenStep::from_parts(true, None, None, &[]),
        };
        runtime::TokenStep::from_parts(
            complete,
            self.token.clone(),
            self.next_action.clone(),
            &["request", "reset_password", "totp"],
        )
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "authentication/token/token/totp",
    method: runtime::Method::Post,
    path: "_auth/token/tokens/totp",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Hash,
        namespace: Some("token"),
        parameters: &[
            runtime::ParameterSpec {
                name: "code",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "token",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Hash,
        namespace: "token",
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
    auth: runtime::RequestAuth::None,
};
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.input_mut().code = runtime::InputValue::Value(value.into());
        self
    }
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.input_mut().token = runtime::InputValue::Value(value.into());
        self
    }
}
