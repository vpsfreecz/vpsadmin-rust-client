use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "auth_token",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub auth_token: runtime::InputValue<String>,
    #[serde(
        rename = "challenge_token",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub challenge_token: runtime::InputValue<String>,
    #[serde(
        rename = "public_key_credential",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub public_key_credential: runtime::InputValue<serde_json::Value>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.auth_token)?;
        runtime::Validate::validate(&self.challenge_token)?;
        runtime::Validate::validate(&self.public_key_credential)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
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
    name: "webauthn/authentication/finish",
    method: runtime::Method::Post,
    path: "v7.0/webauthn/authentication/finish",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Hash,
        namespace: Some("authentication"),
        parameters: &[
            runtime::ParameterSpec {
                name: "auth_token",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "challenge_token",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "public_key_credential",
                kind: runtime::ParameterKind::Custom,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "authentication",
        allow_empty_response: true,
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
    pub fn challenge_token(mut self, value: impl Into<String>) -> Self {
        self.input_mut().challenge_token = runtime::InputValue::Value(value.into());
        self
    }
    pub fn public_key_credential(mut self, value: serde_json::Value) -> Self {
        self.input_mut().public_key_credential = runtime::InputValue::Value(value);
        self
    }
}
