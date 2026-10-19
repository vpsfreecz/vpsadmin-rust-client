use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "enable_user_dns_zones",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_user_dns_zones: runtime::InputValue<bool>,
    #[serde(
        rename = "hidden",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub hidden: runtime::InputValue<bool>,
    #[serde(
        rename = "ipv4_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ipv4_addr: runtime::InputValue<String>,
    #[serde(
        rename = "ipv6_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ipv6_addr: runtime::InputValue<String>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
    #[serde(
        rename = "user_dns_zone_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub user_dns_zone_type: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.enable_user_dns_zones)?;
        runtime::Validate::validate(&self.hidden)?;
        runtime::Validate::validate(&self.ipv4_addr)?;
        runtime::Validate::validate(&self.ipv6_addr)?;
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.user_dns_zone_type)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "enable_user_dns_zones")]
    pub enable_user_dns_zones: Option<bool>,
    #[serde(default, rename = "hidden")]
    pub hidden: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "ipv4_addr")]
    pub ipv4_addr: Option<String>,
    #[serde(default, rename = "ipv6_addr")]
    pub ipv6_addr: Option<String>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(default, rename = "user_dns_zone_type")]
    pub user_dns_zone_type: Option<String>,
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
    name: "dns_server/create",
    method: runtime::Method::Post,
    path: "v7.0/dns_servers",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dns_server"),
        parameters: &[
            runtime::ParameterSpec {
                name: "enable_user_dns_zones",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "hidden",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ipv4_addr",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ipv6_addr",
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
                name: "node",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user_dns_zone_type",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "dns_server",
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
    pub fn enable_user_dns_zones(mut self, value: bool) -> Self {
        self.input_mut().enable_user_dns_zones = runtime::InputValue::Value(value);
        self
    }
    pub fn hidden(mut self, value: bool) -> Self {
        self.input_mut().hidden = runtime::InputValue::Value(value);
        self
    }
    pub fn ipv4_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().ipv4_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn ipv6_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().ipv6_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
        self
    }
    pub fn user_dns_zone_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().user_dns_zone_type = runtime::InputValue::Value(value.into());
        self
    }
}
