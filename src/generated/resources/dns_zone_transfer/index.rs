use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "dns_tsig_key",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dns_tsig_key: runtime::InputValue<i64>,
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
        rename = "host_ip_address",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub host_ip_address: runtime::InputValue<i64>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(
        rename = "peer_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub peer_type: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.dns_tsig_key)?;
        runtime::Validate::validate(&self.dns_zone)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.host_ip_address)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.peer_type)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "dns_tsig_key")]
    pub dns_tsig_key: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "dns_zone")]
    pub dns_zone: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=addr"]
    #[serde(default, rename = "host_ip_address")]
    pub host_ip_address: Option<runtime::Association>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "peer_type")]
    pub peer_type: Option<String>,
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
    name: "dns_zone_transfer/index",
    method: runtime::Method::Get,
    path: "v7.0/dns_zone_transfers",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dns_zone_transfer"),
        parameters: &[
            runtime::ParameterSpec {
                name: "dns_tsig_key",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: true,
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
                name: "host_ip_address",
                kind: runtime::ParameterKind::Resource,
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
                name: "peer_type",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "dns_zone_transfers",
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
    pub fn dns_tsig_key(mut self, value: i64) -> Self {
        self.input_mut().dns_tsig_key = runtime::InputValue::Value(value);
        self
    }
    pub fn dns_tsig_key_null(mut self) -> Self {
        self.input_mut().dns_tsig_key = runtime::InputValue::Null;
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
    pub fn host_ip_address(mut self, value: i64) -> Self {
        self.input_mut().host_ip_address = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn peer_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().peer_type = runtime::InputValue::Value(value.into());
        self
    }
}
