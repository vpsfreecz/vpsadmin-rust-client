use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "active",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub active: runtime::InputValue<bool>,
    #[serde(
        rename = "assigned_by_chain",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub assigned_by_chain: runtime::InputValue<i64>,
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "ip_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_addr: runtime::InputValue<String>,
    #[serde(
        rename = "ip_address",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_address: runtime::InputValue<i64>,
    #[serde(
        rename = "ip_prefix",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_prefix: runtime::InputValue<i64>,
    #[serde(
        rename = "ip_version",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_version: runtime::InputValue<i64>,
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
        rename = "order",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub order: runtime::InputValue<String>,
    #[serde(
        rename = "reconstructed",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reconstructed: runtime::InputValue<bool>,
    #[serde(
        rename = "unassigned_by_chain",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub unassigned_by_chain: runtime::InputValue<i64>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
    #[serde(rename = "vps", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub vps: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.active)?;
        runtime::Validate::validate(&self.assigned_by_chain)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.ip_addr)?;
        runtime::Validate::validate(&self.ip_address)?;
        runtime::Validate::validate(&self.ip_prefix)?;
        runtime::Validate::validate(&self.ip_version)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.network)?;
        runtime::Validate::validate(&self.order)?;
        runtime::Validate::validate(&self.reconstructed)?;
        runtime::Validate::validate(&self.unassigned_by_chain)?;
        runtime::Validate::validate(&self.user)?;
        runtime::Validate::validate(&self.vps)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "assigned_by_chain")]
    pub assigned_by_chain: Option<runtime::Association>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "from_date")]
    pub from_date: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "ip_addr")]
    pub ip_addr: Option<String>,
    #[doc = "Association keys: ID=id, label=addr"]
    #[serde(default, rename = "ip_address")]
    pub ip_address: Option<runtime::Association>,
    #[serde(default, rename = "ip_prefix")]
    pub ip_prefix: Option<i64>,
    #[serde(default, rename = "raw_user_id")]
    pub raw_user_id: Option<i64>,
    #[serde(default, rename = "raw_vps_id")]
    pub raw_vps_id: Option<i64>,
    #[serde(default, rename = "reconstructed")]
    pub reconstructed: Option<bool>,
    #[serde(default, rename = "to_date")]
    pub to_date: Option<String>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "unassigned_by_chain")]
    pub unassigned_by_chain: Option<runtime::Association>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "ip_address_assignment/index",
    method: runtime::Method::Get,
    path: "v7.0/ip_address_assignments",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("ip_address_assignment"),
        parameters: &[
            runtime::ParameterSpec {
                name: "active",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "assigned_by_chain",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: true,
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
                name: "ip_addr",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ip_address",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ip_prefix",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ip_version",
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
                name: "order",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "reconstructed",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "unassigned_by_chain",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "ip_address_assignments",
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
    pub fn active(mut self, value: bool) -> Self {
        self.input_mut().active = runtime::InputValue::Value(value);
        self
    }
    pub fn assigned_by_chain(mut self, value: i64) -> Self {
        self.input_mut().assigned_by_chain = runtime::InputValue::Value(value);
        self
    }
    pub fn assigned_by_chain_null(mut self) -> Self {
        self.input_mut().assigned_by_chain = runtime::InputValue::Null;
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn ip_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().ip_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn ip_address(mut self, value: i64) -> Self {
        self.input_mut().ip_address = runtime::InputValue::Value(value);
        self
    }
    pub fn ip_prefix(mut self, value: i64) -> Self {
        self.input_mut().ip_prefix = runtime::InputValue::Value(value);
        self
    }
    pub fn ip_version(mut self, value: i64) -> Self {
        self.input_mut().ip_version = runtime::InputValue::Value(value);
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
    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.input_mut().order = runtime::InputValue::Value(value.into());
        self
    }
    pub fn reconstructed(mut self, value: bool) -> Self {
        self.input_mut().reconstructed = runtime::InputValue::Value(value);
        self
    }
    pub fn unassigned_by_chain(mut self, value: i64) -> Self {
        self.input_mut().unassigned_by_chain = runtime::InputValue::Value(value);
        self
    }
    pub fn unassigned_by_chain_null(mut self) -> Self {
        self.input_mut().unassigned_by_chain = runtime::InputValue::Null;
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
    pub fn vps(mut self, value: i64) -> Self {
        self.input_mut().vps = runtime::InputValue::Value(value);
        self
    }
}
