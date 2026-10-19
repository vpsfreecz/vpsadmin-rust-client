use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enabled: runtime::InputValue<bool>,
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
        rename = "purpose",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub purpose: runtime::InputValue<String>,
    #[serde(
        rename = "usable_for",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub usable_for: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.enabled)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.purpose)?;
        runtime::Validate::validate(&self.usable_for)?;
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "network/index",
    method: runtime::Method::Get,
    path: "v7.0/networks",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("network"),
        parameters: &[
            runtime::ParameterSpec {
                name: "enabled",
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
                name: "purpose",
                kind: runtime::ParameterKind::String,
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
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "networks",
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
    pub fn enabled(mut self, value: bool) -> Self {
        self.input_mut().enabled = runtime::InputValue::Value(value);
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
    pub fn purpose(mut self, value: impl Into<String>) -> Self {
        self.input_mut().purpose = runtime::InputValue::Value(value.into());
        self
    }
    pub fn usable_for(mut self, value: impl Into<String>) -> Self {
        self.input_mut().usable_for = runtime::InputValue::Value(value.into());
        self
    }
}
