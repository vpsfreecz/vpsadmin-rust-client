use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "address",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub address: runtime::InputValue<String>,
    #[serde(
        rename = "enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "ip_version",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_version: runtime::InputValue<i64>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(
        rename = "managed",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub managed: runtime::InputValue<bool>,
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
    #[serde(
        rename = "split_access",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub split_access: runtime::InputValue<String>,
    #[serde(
        rename = "split_prefix",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub split_prefix: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.address)?;
        runtime::Validate::validate(&self.enabled)?;
        runtime::Validate::validate(&self.ip_version)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.managed)?;
        runtime::Validate::validate(&self.prefix)?;
        runtime::Validate::validate(&self.purpose)?;
        runtime::Validate::validate(&self.role)?;
        runtime::Validate::validate(&self.split_access)?;
        runtime::Validate::validate(&self.split_prefix)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "address")]
    pub address: Option<String>,
    #[serde(default, rename = "assigned")]
    pub assigned: Option<i64>,
    #[serde(default, rename = "available_to_users")]
    pub available_to_users: Option<i64>,
    #[serde(default, rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "ip_version")]
    pub ip_version: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "managed")]
    pub managed: Option<bool>,
    #[serde(default, rename = "owned")]
    pub owned: Option<i64>,
    #[serde(default, rename = "owned_unassigned")]
    pub owned_unassigned: Option<i64>,
    #[serde(default, rename = "prefix")]
    pub prefix: Option<i64>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "primary_location")]
    pub primary_location: Option<runtime::Association>,
    #[serde(default, rename = "purpose")]
    pub purpose: Option<String>,
    #[serde(default, rename = "role")]
    pub role: Option<String>,
    #[serde(default, rename = "size")]
    pub size: Option<i64>,
    #[serde(default, rename = "split_access")]
    pub split_access: Option<String>,
    #[serde(default, rename = "split_prefix")]
    pub split_prefix: Option<i64>,
    #[serde(default, rename = "taken")]
    pub taken: Option<i64>,
    #[serde(default, rename = "used")]
    pub used: Option<i64>,
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
    name: "network/update",
    method: runtime::Method::Put,
    path: "v7.0/networks/{network_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("network"),
        parameters: &[
            runtime::ParameterSpec {
                name: "address",
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
                name: "ip_version",
                kind: runtime::ParameterKind::Integer,
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
                name: "managed",
                kind: runtime::ParameterKind::Boolean,
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
                name: "split_access",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "split_prefix",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "network",
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
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.input_mut().address = runtime::InputValue::Value(value.into());
        self
    }
    pub fn enabled(mut self, value: bool) -> Self {
        self.input_mut().enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn ip_version(mut self, value: i64) -> Self {
        self.input_mut().ip_version = runtime::InputValue::Value(value);
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn managed(mut self, value: bool) -> Self {
        self.input_mut().managed = runtime::InputValue::Value(value);
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
    pub fn split_access(mut self, value: impl Into<String>) -> Self {
        self.input_mut().split_access = runtime::InputValue::Value(value.into());
        self
    }
    pub fn split_prefix(mut self, value: i64) -> Self {
        self.input_mut().split_prefix = runtime::InputValue::Value(value);
        self
    }
}
