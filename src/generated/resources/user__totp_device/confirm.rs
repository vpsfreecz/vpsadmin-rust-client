use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "code", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub code: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.code)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "recovery_code")]
    pub recovery_code: Option<String>,
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
    name: "user/totp_device/confirm",
    method: runtime::Method::Post,
    path: "v7.0/users/{user_id}/totp_devices/{totp_device_id}/confirm",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Hash,
        namespace: Some("totp_device"),
        parameters: &[runtime::ParameterSpec {
            name: "code",
            kind: runtime::ParameterKind::String,
            required: true,
            nullable: false,
            protected: false,
        }],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Hash,
        namespace: "totp_device",
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
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.input_mut().code = runtime::InputValue::Value(value.into());
        self
    }
}
