use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "addresses",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub addresses: runtime::InputValue<serde_json::Value>,
    #[serde(
        rename = "reason",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reason: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.addresses)?;
        runtime::Validate::validate(&self.reason)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "can_assign")]
    pub can_assign: Option<bool>,
    #[serde(default, rename = "can_keep")]
    pub can_keep: Option<bool>,
    #[serde(default, rename = "deadline")]
    pub deadline: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
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
    name: "ip_release_request/keep",
    method: runtime::Method::Post,
    path: "v7.0/ip_release_requests/{ip_release_request_id}/keep",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("ip_release_request"),
        parameters: &[
            runtime::ParameterSpec {
                name: "addresses",
                kind: runtime::ParameterKind::Custom,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "reason",
                kind: runtime::ParameterKind::Text,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "ip_release_request",
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
    pub fn addresses(mut self, value: serde_json::Value) -> Self {
        self.input_mut().addresses = runtime::InputValue::Value(value);
        self
    }
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reason = runtime::InputValue::Value(value.into());
        self
    }
}
