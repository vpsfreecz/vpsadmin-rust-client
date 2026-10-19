use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "addr", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub addr: runtime::InputValue<String>,
    #[serde(
        rename = "location",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub location: runtime::InputValue<i64>,
    #[serde(
        rename = "network",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub network: runtime::InputValue<i64>,
    #[serde(
        rename = "network_interface",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub network_interface: runtime::InputValue<i64>,
    #[serde(
        rename = "prefix",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub prefix: runtime::InputValue<i64>,
    #[serde(
        rename = "route_via",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub route_via: runtime::InputValue<i64>,
    #[serde(rename = "size", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub size: runtime::InputValue<i64>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.addr)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.network)?;
        runtime::Validate::validate(&self.network_interface)?;
        runtime::Validate::validate(&self.prefix)?;
        runtime::Validate::validate(&self.route_via)?;
        runtime::Validate::validate(&self.size)?;
        runtime::Validate::validate(&self.user)?;
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
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "ip_address/create",
    method: runtime::Method::Post,
    path: "v7.0/ip_addresses",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("ip_address"),
        parameters: &[
            runtime::ParameterSpec {
                name: "addr",
                kind: runtime::ParameterKind::String,
                required: true,
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
                name: "network",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "network_interface",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "prefix",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "route_via",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "size",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: true,
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
    pub fn addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn location(mut self, value: i64) -> Self {
        self.input_mut().location = runtime::InputValue::Value(value);
        self
    }
    pub fn network(mut self, value: i64) -> Self {
        self.input_mut().network = runtime::InputValue::Value(value);
        self
    }
    pub fn network_interface(mut self, value: i64) -> Self {
        self.input_mut().network_interface = runtime::InputValue::Value(value);
        self
    }
    pub fn prefix(mut self, value: i64) -> Self {
        self.input_mut().prefix = runtime::InputValue::Value(value);
        self
    }
    pub fn route_via(mut self, value: i64) -> Self {
        self.input_mut().route_via = runtime::InputValue::Value(value);
        self
    }
    pub fn size(mut self, value: i64) -> Self {
        self.input_mut().size = runtime::InputValue::Value(value);
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
    pub fn user_null(mut self) -> Self {
        self.input_mut().user = runtime::InputValue::Null;
        self
    }
}
