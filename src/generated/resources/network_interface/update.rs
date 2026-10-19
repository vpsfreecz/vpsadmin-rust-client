use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "enable",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable: runtime::InputValue<bool>,
    #[serde(
        rename = "max_rx",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub max_rx: runtime::InputValue<i64>,
    #[serde(
        rename = "max_tx",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub max_tx: runtime::InputValue<i64>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.enable)?;
        runtime::Validate::validate(&self.max_rx)?;
        runtime::Validate::validate(&self.max_tx)?;
        runtime::Validate::validate(&self.name)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "enable")]
    pub enable: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "mac")]
    pub mac: Option<String>,
    #[serde(default, rename = "max_rx")]
    pub max_rx: Option<i64>,
    #[serde(default, rename = "max_tx")]
    pub max_tx: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
    #[doc = "Association keys: ID=id, label=hostname"]
    #[serde(default, rename = "vps")]
    pub vps: Option<runtime::Association>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(default, rename = "action_state_id")]
    pub action_state_id: Option<i64>,
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "network_interface/update",
    method: runtime::Method::Put,
    path: "v7.0/network_interfaces/{network_interface_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("network_interface"),
        parameters: &[
            runtime::ParameterSpec {
                name: "enable",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "max_rx",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "max_tx",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "name",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "network_interface",
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
    pub fn enable(mut self, value: bool) -> Self {
        self.input_mut().enable = runtime::InputValue::Value(value);
        self
    }
    pub fn max_rx(mut self, value: i64) -> Self {
        self.input_mut().max_rx = runtime::InputValue::Value(value);
        self
    }
    pub fn max_tx(mut self, value: i64) -> Self {
        self.input_mut().max_tx = runtime::InputValue::Value(value);
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
}
