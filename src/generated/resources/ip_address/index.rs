use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "addr", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub addr: runtime::InputValue<String>,
    #[serde(
        rename = "assigned_to_interface",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub assigned_to_interface: runtime::InputValue<bool>,
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
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
        rename = "network_enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub network_enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "network_interface",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub network_interface: runtime::InputValue<i64>,
    #[serde(
        rename = "order",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub order: runtime::InputValue<String>,
    #[serde(
        rename = "prefix",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub prefix: runtime::InputValue<i64>,
    #[serde(
        rename = "purpose",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub purpose: runtime::InputValue<String>,
    #[serde(rename = "role", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub role: runtime::InputValue<String>,
    #[serde(rename = "size", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub size: runtime::InputValue<i64>,
    #[serde(
        rename = "usable_for",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub usable_for: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
    #[serde(
        rename = "version",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub version: runtime::InputValue<i64>,
    #[serde(rename = "vps", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub vps: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.addr)?;
        runtime::Validate::validate(&self.assigned_to_interface)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.network)?;
        runtime::Validate::validate(&self.network_enabled)?;
        runtime::Validate::validate(&self.network_interface)?;
        runtime::Validate::validate(&self.order)?;
        runtime::Validate::validate(&self.prefix)?;
        runtime::Validate::validate(&self.purpose)?;
        runtime::Validate::validate(&self.role)?;
        runtime::Validate::validate(&self.size)?;
        runtime::Validate::validate(&self.usable_for)?;
        runtime::Validate::validate(&self.user)?;
        runtime::Validate::validate(&self.version)?;
        runtime::Validate::validate(&self.vps)?;
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "ip_address/index",
    method: runtime::Method::Get,
    path: "v7.0/ip_addresses",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("ip_address"),
        parameters: &[
            runtime::ParameterSpec {
                name: "addr",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "assigned_to_interface",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "from_id",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "limit",
                kind: runtime::ParameterKind::Integer,
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
                name: "network",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "network_enabled",
                kind: runtime::ParameterKind::Boolean,
                required: false,
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
                name: "order",
                kind: runtime::ParameterKind::String,
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
                name: "purpose",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "role",
                kind: runtime::ParameterKind::String,
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
                name: "usable_for",
                kind: runtime::ParameterKind::String,
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
            runtime::ParameterSpec {
                name: "version",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: true,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "ip_addresses",
        allow_empty_response: false,
    }),
    meta_namespace: Some("_meta"),
    global_meta: &[
        runtime::ParameterSpec {
            name: "count",
            kind: runtime::ParameterKind::Boolean,
            required: false,
            nullable: false,
            protected: false,
        },
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Vec<Output>, Meta> {
    pub fn addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn assigned_to_interface(mut self, value: bool) -> Self {
        self.input_mut().assigned_to_interface = runtime::InputValue::Value(value);
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
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
    pub fn network_enabled(mut self, value: bool) -> Self {
        self.input_mut().network_enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn network_interface(mut self, value: i64) -> Self {
        self.input_mut().network_interface = runtime::InputValue::Value(value);
        self
    }
    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.input_mut().order = runtime::InputValue::Value(value.into());
        self
    }
    pub fn prefix(mut self, value: i64) -> Self {
        self.input_mut().prefix = runtime::InputValue::Value(value);
        self
    }
    pub fn purpose(mut self, value: impl Into<String>) -> Self {
        self.input_mut().purpose = runtime::InputValue::Value(value.into());
        self
    }
    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.input_mut().role = runtime::InputValue::Value(value.into());
        self
    }
    pub fn size(mut self, value: i64) -> Self {
        self.input_mut().size = runtime::InputValue::Value(value);
        self
    }
    pub fn usable_for(mut self, value: impl Into<String>) -> Self {
        self.input_mut().usable_for = runtime::InputValue::Value(value.into());
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
    pub fn version(mut self, value: i64) -> Self {
        self.input_mut().version = runtime::InputValue::Value(value);
        self
    }
    pub fn vps(mut self, value: i64) -> Self {
        self.input_mut().vps = runtime::InputValue::Value(value);
        self
    }
    pub fn vps_null(mut self) -> Self {
        self.input_mut().vps = runtime::InputValue::Null;
        self
    }
}
