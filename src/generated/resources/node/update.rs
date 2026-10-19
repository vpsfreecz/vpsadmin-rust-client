use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "active",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub active: runtime::InputValue<bool>,
    #[serde(rename = "fqdn", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub fqdn: runtime::InputValue<String>,
    #[serde(
        rename = "hypervisor_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub hypervisor_type: runtime::InputValue<String>,
    #[serde(
        rename = "ip_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_addr: runtime::InputValue<String>,
    #[serde(
        rename = "location",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub location: runtime::InputValue<i64>,
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
    #[serde(
        rename = "max_vps",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub max_vps: runtime::InputValue<i64>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(rename = "type", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub type_: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.active)?;
        runtime::Validate::validate(&self.fqdn)?;
        runtime::Validate::validate(&self.hypervisor_type)?;
        runtime::Validate::validate(&self.ip_addr)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.max_rx)?;
        runtime::Validate::validate(&self.max_tx)?;
        runtime::Validate::validate(&self.max_vps)?;
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.type_)?;
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
    name: "node/update",
    method: runtime::Method::Put,
    path: "v7.0/nodes/{node_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node"),
        parameters: &[
            runtime::ParameterSpec {
                name: "active",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "fqdn",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "hypervisor_type",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ip_addr",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "location",
                kind: runtime::ParameterKind::Resource,
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
                name: "max_vps",
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
            runtime::ParameterSpec {
                name: "type",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "node",
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
    pub fn active(mut self, value: bool) -> Self {
        self.input_mut().active = runtime::InputValue::Value(value);
        self
    }
    pub fn fqdn(mut self, value: impl Into<String>) -> Self {
        self.input_mut().fqdn = runtime::InputValue::Value(value.into());
        self
    }
    pub fn hypervisor_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().hypervisor_type = runtime::InputValue::Value(value.into());
        self
    }
    pub fn ip_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().ip_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn location(mut self, value: i64) -> Self {
        self.input_mut().location = runtime::InputValue::Value(value);
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
    pub fn max_vps(mut self, value: i64) -> Self {
        self.input_mut().max_vps = runtime::InputValue::Value(value);
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.input_mut().type_ = runtime::InputValue::Value(value.into());
        self
    }
}
