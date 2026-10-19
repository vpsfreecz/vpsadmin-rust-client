use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "dst_node",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dst_node: runtime::InputValue<i64>,
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
        rename = "src_node",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub src_node: runtime::InputValue<i64>,
    #[serde(
        rename = "state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub state: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.dst_node)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.src_node)?;
        runtime::Validate::validate(&self.state)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "cleanup_data")]
    pub cleanup_data: Option<bool>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "dst_node")]
    pub dst_node: Option<runtime::Association>,
    #[serde(default, rename = "finished_at")]
    pub finished_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "maintenance_window")]
    pub maintenance_window: Option<bool>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "src_node")]
    pub src_node: Option<runtime::Association>,
    #[serde(default, rename = "started_at")]
    pub started_at: Option<String>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "transaction_chain")]
    pub transaction_chain: Option<runtime::Association>,
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
    name: "migration_plan/vps_migration/index",
    method: runtime::Method::Get,
    path: "v7.0/migration_plans/{migration_plan_id}/vps_migrations",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("vps_migration"),
        parameters: &[
            runtime::ParameterSpec {
                name: "dst_node",
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
                name: "src_node",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "state",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "vps_migrations",
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
    pub fn dst_node(mut self, value: i64) -> Self {
        self.input_mut().dst_node = runtime::InputValue::Value(value);
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
    pub fn src_node(mut self, value: i64) -> Self {
        self.input_mut().src_node = runtime::InputValue::Value(value);
        self
    }
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().state = runtime::InputValue::Value(value.into());
        self
    }
}
