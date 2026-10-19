use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "default_ttl",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub default_ttl: runtime::InputValue<i64>,
    #[serde(
        rename = "dnssec_enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dnssec_enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "email",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub email: runtime::InputValue<String>,
    #[serde(
        rename = "enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(
        rename = "reverse_network_address",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reverse_network_address: runtime::InputValue<String>,
    #[serde(
        rename = "reverse_network_prefix",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reverse_network_prefix: runtime::InputValue<String>,
    #[serde(rename = "role", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub role: runtime::InputValue<String>,
    #[serde(
        rename = "seed_vps",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub seed_vps: runtime::InputValue<i64>,
    #[serde(
        rename = "source",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub source: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.default_ttl)?;
        runtime::Validate::validate(&self.dnssec_enabled)?;
        runtime::Validate::validate(&self.email)?;
        runtime::Validate::validate(&self.enabled)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.reverse_network_address)?;
        runtime::Validate::validate(&self.reverse_network_prefix)?;
        runtime::Validate::validate(&self.role)?;
        runtime::Validate::validate(&self.seed_vps)?;
        runtime::Validate::validate(&self.source)?;
        runtime::Validate::validate(&self.user)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "default_ttl")]
    pub default_ttl: Option<i64>,
    #[serde(default, rename = "dnssec_enabled")]
    pub dnssec_enabled: Option<bool>,
    #[serde(default, rename = "email")]
    pub email: Option<String>,
    #[serde(default, rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "reverse_network_address")]
    pub reverse_network_address: Option<String>,
    #[serde(default, rename = "reverse_network_prefix")]
    pub reverse_network_prefix: Option<String>,
    #[serde(default, rename = "role")]
    pub role: Option<String>,
    #[serde(default, rename = "serial")]
    pub serial: Option<i64>,
    #[serde(default, rename = "source")]
    pub source: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
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
    name: "dns_zone/create",
    method: runtime::Method::Post,
    path: "v7.0/dns_zones",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dns_zone"),
        parameters: &[
            runtime::ParameterSpec {
                name: "default_ttl",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "dnssec_enabled",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "email",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enabled",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "label",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "name",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "reverse_network_address",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "reverse_network_prefix",
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
                name: "seed_vps",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "source",
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
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "dns_zone",
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
    pub fn default_ttl(mut self, value: i64) -> Self {
        self.input_mut().default_ttl = runtime::InputValue::Value(value);
        self
    }
    pub fn dnssec_enabled(mut self, value: bool) -> Self {
        self.input_mut().dnssec_enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.input_mut().email = runtime::InputValue::Value(value.into());
        self
    }
    pub fn enabled(mut self, value: bool) -> Self {
        self.input_mut().enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn reverse_network_address(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reverse_network_address = runtime::InputValue::Value(value.into());
        self
    }
    pub fn reverse_network_prefix(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reverse_network_prefix = runtime::InputValue::Value(value.into());
        self
    }
    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.input_mut().role = runtime::InputValue::Value(value.into());
        self
    }
    pub fn seed_vps(mut self, value: i64) -> Self {
        self.input_mut().seed_vps = runtime::InputValue::Value(value);
        self
    }
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.input_mut().source = runtime::InputValue::Value(value.into());
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
