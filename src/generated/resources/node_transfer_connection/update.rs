use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "node_a_ip_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub node_a_ip_addr: runtime::InputValue<String>,
    #[serde(
        rename = "node_b_ip_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub node_b_ip_addr: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.enabled)?;
        runtime::Validate::validate(&self.node_a_ip_addr)?;
        runtime::Validate::validate(&self.node_b_ip_addr)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node_a")]
    pub node_a: Option<runtime::Association>,
    #[serde(default, rename = "node_a_ip_addr")]
    pub node_a_ip_addr: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node_b")]
    pub node_b: Option<runtime::Association>,
    #[serde(default, rename = "node_b_ip_addr")]
    pub node_b_ip_addr: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
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
    name: "node_transfer_connection/update",
    method: runtime::Method::Put,
    path: "v7.0/node_transfer_connections/{node_transfer_connection_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node_transfer_connection"),
        parameters: &[
            runtime::ParameterSpec {
                name: "enabled",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "node_a_ip_addr",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "node_b_ip_addr",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "node_transfer_connection",
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
    pub fn enabled(mut self, value: bool) -> Self {
        self.input_mut().enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn node_a_ip_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().node_a_ip_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn node_b_ip_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().node_b_ip_addr = runtime::InputValue::Value(value.into());
        self
    }
}
