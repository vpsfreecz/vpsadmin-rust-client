use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "addr", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub addr: runtime::InputValue<String>,
    #[serde(
        rename = "assigned",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub assigned: runtime::InputValue<bool>,
    #[serde(
        rename = "ip_address",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_address: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.addr)?;
        runtime::Validate::validate(&self.assigned)?;
        runtime::Validate::validate(&self.ip_address)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "addr")]
    pub addr: Option<String>,
    #[serde(default, rename = "assigned")]
    pub assigned: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=addr"]
    #[serde(default, rename = "ip_address")]
    pub ip_address: Option<runtime::Association>,
    #[serde(default, rename = "reverse_record_value")]
    pub reverse_record_value: Option<String>,
    #[serde(default, rename = "user_created")]
    pub user_created: Option<bool>,
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
    name: "host_ip_address/create",
    method: runtime::Method::Post,
    path: "v7.0/host_ip_addresses",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("host_ip_address"),
        parameters: &[
            runtime::ParameterSpec {
                name: "addr",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "assigned",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ip_address",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "host_ip_address",
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
    pub fn addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn assigned(mut self, value: bool) -> Self {
        self.input_mut().assigned = runtime::InputValue::Value(value);
        self
    }
    pub fn ip_address(mut self, value: i64) -> Self {
        self.input_mut().ip_address = runtime::InputValue::Value(value);
        self
    }
}
