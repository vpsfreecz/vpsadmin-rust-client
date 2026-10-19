use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "host_ip_address",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub host_ip_address: runtime::InputValue<i64>,
    #[serde(
        rename = "network_interface",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub network_interface: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.host_ip_address)?;
        runtime::Validate::validate(&self.network_interface)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "addr")]
    pub addr: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "charged_environment")]
    pub charged_environment: Option<runtime::Association>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=address"]
    #[serde(default, rename = "network")]
    pub network: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "network_interface")]
    pub network_interface: Option<runtime::Association>,
    #[serde(default, rename = "prefix")]
    pub prefix: Option<i64>,
    #[doc = "Association keys: ID=id, label=addr"]
    #[serde(default, rename = "route_via")]
    pub route_via: Option<runtime::Association>,
    #[serde(default, rename = "size")]
    pub size: Option<i64>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
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
    name: "ip_address/assign_with_host_address",
    method: runtime::Method::Post,
    path: "v7.0/ip_addresses/{ip_address_id}/assign_with_host_address",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("ip_address"),
        parameters: &[
            runtime::ParameterSpec {
                name: "host_ip_address",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "network_interface",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "ip_address",
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
    pub fn host_ip_address(mut self, value: i64) -> Self {
        self.input_mut().host_ip_address = runtime::InputValue::Value(value);
        self
    }
    pub fn network_interface(mut self, value: i64) -> Self {
        self.input_mut().network_interface = runtime::InputValue::Value(value);
        self
    }
}
