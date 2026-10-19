use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "dns_server_zone",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dns_server_zone: runtime::InputValue<i64>,
    #[serde(
        rename = "dns_zone",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dns_zone: runtime::InputValue<i64>,
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
        rename = "order",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub order: runtime::InputValue<String>,
    #[serde(
        rename = "primary_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub primary_addr: runtime::InputValue<String>,
    #[serde(
        rename = "reason_code",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reason_code: runtime::InputValue<String>,
    #[serde(
        rename = "status",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub status: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.dns_server_zone)?;
        runtime::Validate::validate(&self.dns_zone)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.order)?;
        runtime::Validate::validate(&self.primary_addr)?;
        runtime::Validate::validate(&self.reason_code)?;
        runtime::Validate::validate(&self.status)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "dns_server_zone")]
    pub dns_server_zone: Option<runtime::Association>,
    #[serde(default, rename = "event_at")]
    pub event_at: Option<String>,
    #[serde(default, rename = "event_key")]
    pub event_key: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "message")]
    pub message: Option<String>,
    #[serde(default, rename = "primary_addr")]
    pub primary_addr: Option<String>,
    #[serde(default, rename = "raw_message")]
    pub raw_message: Option<String>,
    #[serde(default, rename = "reason")]
    pub reason: Option<String>,
    #[serde(default, rename = "reason_code")]
    pub reason_code: Option<String>,
    #[serde(default, rename = "serial")]
    pub serial: Option<i64>,
    #[serde(default, rename = "source_cursor")]
    pub source_cursor: Option<String>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "dns_server_zone_transfer_log/index",
    method: runtime::Method::Get,
    path: "v7.0/dns_server_zone_transfer_logs",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dns_server_zone_transfer_log"),
        parameters: &[
            runtime::ParameterSpec {
                name: "dns_server_zone",
                kind: runtime::ParameterKind::Resource,
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
                name: "order",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "primary_addr",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "reason_code",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "status",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "dns_server_zone_transfer_logs",
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
    pub fn dns_server_zone(mut self, value: i64) -> Self {
        self.input_mut().dns_server_zone = runtime::InputValue::Value(value);
        self
    }
    pub fn dns_zone(mut self, value: i64) -> Self {
        self.input_mut().dns_zone = runtime::InputValue::Value(value);
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
    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.input_mut().order = runtime::InputValue::Value(value.into());
        self
    }
    pub fn primary_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().primary_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn reason_code(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reason_code = runtime::InputValue::Value(value.into());
        self
    }
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.input_mut().status = runtime::InputValue::Value(value.into());
        self
    }
}
