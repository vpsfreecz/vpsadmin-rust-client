use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "environment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub environment: runtime::InputValue<i64>,
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "has_hypervisor",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub has_hypervisor: runtime::InputValue<bool>,
    #[serde(
        rename = "has_storage",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub has_storage: runtime::InputValue<bool>,
    #[serde(
        rename = "hypervisor_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub hypervisor_type: runtime::InputValue<String>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(
        rename = "shares_any_networks_with",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub shares_any_networks_with: runtime::InputValue<i64>,
    #[serde(
        rename = "shares_networks_primary",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub shares_networks_primary: runtime::InputValue<bool>,
    #[serde(
        rename = "shares_v4_networks_with",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub shares_v4_networks_with: runtime::InputValue<i64>,
    #[serde(
        rename = "shares_v6_networks_with",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub shares_v6_networks_with: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.environment)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.has_hypervisor)?;
        runtime::Validate::validate(&self.has_storage)?;
        runtime::Validate::validate(&self.hypervisor_type)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.shares_any_networks_with)?;
        runtime::Validate::validate(&self.shares_networks_primary)?;
        runtime::Validate::validate(&self.shares_v4_networks_with)?;
        runtime::Validate::validate(&self.shares_v6_networks_with)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "description")]
    pub description: Option<String>,
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "environment")]
    pub environment: Option<runtime::Association>,
    #[serde(default, rename = "has_ipv6")]
    pub has_ipv6: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "maintenance_lock")]
    pub maintenance_lock: Option<String>,
    #[serde(default, rename = "maintenance_lock_reason")]
    pub maintenance_lock_reason: Option<String>,
    #[serde(default, rename = "remote_console_server")]
    pub remote_console_server: Option<String>,
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
    name: "location/index",
    method: runtime::Method::Get,
    path: "v7.0/locations",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("location"),
        parameters: &[
            runtime::ParameterSpec {
                name: "environment",
                kind: runtime::ParameterKind::Resource,
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
                name: "has_hypervisor",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "has_storage",
                kind: runtime::ParameterKind::Boolean,
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
                name: "limit",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "shares_any_networks_with",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "shares_networks_primary",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "shares_v4_networks_with",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "shares_v6_networks_with",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "locations",
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
    pub fn environment(mut self, value: i64) -> Self {
        self.input_mut().environment = runtime::InputValue::Value(value);
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn has_hypervisor(mut self, value: bool) -> Self {
        self.input_mut().has_hypervisor = runtime::InputValue::Value(value);
        self
    }
    pub fn has_storage(mut self, value: bool) -> Self {
        self.input_mut().has_storage = runtime::InputValue::Value(value);
        self
    }
    pub fn hypervisor_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().hypervisor_type = runtime::InputValue::Value(value.into());
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn shares_any_networks_with(mut self, value: i64) -> Self {
        self.input_mut().shares_any_networks_with = runtime::InputValue::Value(value);
        self
    }
    pub fn shares_networks_primary(mut self, value: bool) -> Self {
        self.input_mut().shares_networks_primary = runtime::InputValue::Value(value);
        self
    }
    pub fn shares_v4_networks_with(mut self, value: i64) -> Self {
        self.input_mut().shares_v4_networks_with = runtime::InputValue::Value(value);
        self
    }
    pub fn shares_v6_networks_with(mut self, value: i64) -> Self {
        self.input_mut().shares_v6_networks_with = runtime::InputValue::Value(value);
        self
    }
}
