use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "expected_content_revision",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub expected_content_revision: runtime::InputValue<i64>,
    #[serde(
        rename = "expected_state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub expected_state: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.expected_content_revision)?;
        runtime::Validate::validate(&self.expected_state)?;
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
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "security_advisory/delete",
    method: runtime::Method::Delete,
    path: "v7.0/security_advisories/{security_advisory_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("security_advisory"),
        parameters: &[
            runtime::ParameterSpec {
                name: "expected_content_revision",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "expected_state",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "security_advisory",
        allow_empty_response: true,
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
    pub fn expected_content_revision(mut self, value: i64) -> Self {
        self.input_mut().expected_content_revision = runtime::InputValue::Value(value);
        self
    }
    pub fn expected_state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().expected_state = runtime::InputValue::Value(value.into());
        self
    }
}
