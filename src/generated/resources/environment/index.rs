use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
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
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.has_hypervisor)?;
        runtime::Validate::validate(&self.has_storage)?;
        runtime::Validate::validate(&self.limit)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "can_create_vps")]
    pub can_create_vps: Option<bool>,
    #[serde(default, rename = "can_destroy_vps")]
    pub can_destroy_vps: Option<bool>,
    #[serde(default, rename = "description")]
    pub description: Option<String>,
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "maintenance_lock")]
    pub maintenance_lock: Option<String>,
    #[serde(default, rename = "maintenance_lock_reason")]
    pub maintenance_lock_reason: Option<String>,
    #[serde(default, rename = "max_vps_count")]
    pub max_vps_count: Option<i64>,
    #[serde(default, rename = "user_ip_ownership")]
    pub user_ip_ownership: Option<bool>,
    #[serde(default, rename = "vps_lifetime")]
    pub vps_lifetime: Option<i64>,
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
    name: "environment/index",
    method: runtime::Method::Get,
    path: "v7.0/environments",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("environment"),
        parameters: &[
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
                name: "limit",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "environments",
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
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
}
