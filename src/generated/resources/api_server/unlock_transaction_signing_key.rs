use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "passphrase",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub passphrase: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.passphrase)?;
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
    name: "api_server/unlock_transaction_signing_key",
    method: runtime::Method::Post,
    path: "v7.0/api_servers/unlock_transaction_signing_key",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Hash,
        namespace: Some("api_server"),
        parameters: &[runtime::ParameterSpec {
            name: "passphrase",
            kind: runtime::ParameterKind::String,
            required: true,
            nullable: false,
            protected: false,
        }],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "api_server",
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
    pub fn passphrase(mut self, value: impl Into<String>) -> Self {
        self.input_mut().passphrase = runtime::InputValue::Value(value.into());
        self
    }
}
