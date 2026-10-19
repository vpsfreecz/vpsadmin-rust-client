use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "ip_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_addr: runtime::InputValue<String>,
    #[serde(
        rename = "is_universal",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub is_universal: runtime::InputValue<bool>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(
        rename = "location",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub location: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.ip_addr)?;
        runtime::Validate::validate(&self.is_universal)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.location)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "ip_addr")]
    pub ip_addr: Option<String>,
    #[serde(default, rename = "is_universal")]
    pub is_universal: Option<bool>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "location")]
    pub location: Option<runtime::Association>,
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
    name: "dns_resolver/create",
    method: runtime::Method::Post,
    path: "v7.0/dns_resolvers",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dns_resolver"),
        parameters: &[
            runtime::ParameterSpec {
                name: "ip_addr",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "is_universal",
                kind: runtime::ParameterKind::Boolean,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "label",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "location",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: true,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "dns_resolver",
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
    pub fn ip_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().ip_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn is_universal(mut self, value: bool) -> Self {
        self.input_mut().is_universal = runtime::InputValue::Value(value);
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn location(mut self, value: i64) -> Self {
        self.input_mut().location = runtime::InputValue::Value(value);
        self
    }
    pub fn location_null(mut self) -> Self {
        self.input_mut().location = runtime::InputValue::Null;
        self
    }
}
