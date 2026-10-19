use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "interval",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub interval: runtime::InputValue<i64>,
    #[serde(
        rename = "lifetime",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub lifetime: runtime::InputValue<String>,
    #[serde(
        rename = "password",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub password: runtime::InputValue<String>,
    #[serde(
        rename = "scope",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub scope: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.interval)?;
        runtime::Validate::validate(&self.lifetime)?;
        runtime::Validate::validate(&self.password)?;
        runtime::Validate::validate(&self.scope)?;
        runtime::Validate::validate(&self.user)?;
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
    name: "authentication/token/token/request",
    method: runtime::Method::Post,
    path: "_auth/token/tokens",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Hash,
        namespace: Some("token"),
        parameters: &[
            runtime::ParameterSpec {
                name: "interval",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "lifetime",
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
                protected: true,
            },
            runtime::ParameterSpec {
                name: "scope",
                kind: runtime::ParameterKind::String,
                required: false,
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
    pub fn interval(mut self, value: i64) -> Self {
        self.input_mut().interval = runtime::InputValue::Value(value);
        self
    }
    pub fn lifetime(mut self, value: impl Into<String>) -> Self {
        self.input_mut().lifetime = runtime::InputValue::Value(value.into());
        self
    }
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.input_mut().password = runtime::InputValue::Value(value.into());
        self
    }
    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.input_mut().scope = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value.into());
        self
    }
}
