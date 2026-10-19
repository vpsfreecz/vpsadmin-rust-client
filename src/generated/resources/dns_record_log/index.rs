use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "change_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub change_type: runtime::InputValue<String>,
    #[serde(
        rename = "dns_zone",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dns_zone: runtime::InputValue<i64>,
    #[serde(
        rename = "dns_zone_name",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dns_zone_name: runtime::InputValue<String>,
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
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(rename = "type", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub type_: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.change_type)?;
        runtime::Validate::validate(&self.dns_zone)?;
        runtime::Validate::validate(&self.dns_zone_name)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.type_)?;
        runtime::Validate::validate(&self.user)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "attr_changes")]
    pub attr_changes: Option<serde_json::Value>,
    #[serde(default, rename = "change_type")]
    pub change_type: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "dns_zone")]
    pub dns_zone: Option<runtime::Association>,
    #[serde(default, rename = "dns_zone_name")]
    pub dns_zone_name: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "raw_user_id")]
    pub raw_user_id: Option<i64>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "transaction_chain")]
    pub transaction_chain: Option<runtime::Association>,
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "dns_record_log/index",
    method: runtime::Method::Get,
    path: "v7.0/dns_record_logs",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dns_record_log"),
        parameters: &[
            runtime::ParameterSpec {
                name: "change_type",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "dns_zone",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "dns_zone_name",
                kind: runtime::ParameterKind::String,
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
            runtime::ParameterSpec {
                name: "user",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "dns_record_logs",
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
    pub fn change_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().change_type = runtime::InputValue::Value(value.into());
        self
    }
    pub fn dns_zone(mut self, value: i64) -> Self {
        self.input_mut().dns_zone = runtime::InputValue::Value(value);
        self
    }
    pub fn dns_zone_name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().dns_zone_name = runtime::InputValue::Value(value.into());
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
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.input_mut().type_ = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
}
