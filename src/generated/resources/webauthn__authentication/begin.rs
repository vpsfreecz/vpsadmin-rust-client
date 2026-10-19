use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "auth_token",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub auth_token: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.auth_token)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "challenge_token")]
    pub challenge_token: Option<String>,
    #[serde(default, rename = "options")]
    pub options: Option<serde_json::Value>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "webauthn/authentication/begin",
    method: runtime::Method::Post,
    path: "v7.0/webauthn/authentication/begin",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Hash,
        namespace: Some("authentication"),
        parameters: &[runtime::ParameterSpec {
            name: "auth_token",
            kind: runtime::ParameterKind::String,
            required: true,
            nullable: false,
            protected: false,
        }],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Hash,
        namespace: "authentication",
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
    auth: runtime::RequestAuth::Configured,
};
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {
    pub fn auth_token(mut self, value: impl Into<String>) -> Self {
        self.input_mut().auth_token = runtime::InputValue::Value(value.into());
        self
    }
}
