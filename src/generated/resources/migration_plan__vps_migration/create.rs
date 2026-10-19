use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cleanup_data",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cleanup_data: runtime::InputValue<bool>,
    #[serde(
        rename = "dst_node",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dst_node: runtime::InputValue<i64>,
    #[serde(
        rename = "maintenance_window",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub maintenance_window: runtime::InputValue<bool>,
    #[serde(rename = "vps", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub vps: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cleanup_data)?;
        runtime::Validate::validate(&self.dst_node)?;
        runtime::Validate::validate(&self.maintenance_window)?;
        runtime::Validate::validate(&self.vps)?;
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
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "migration_plan/vps_migration/create",
    method: runtime::Method::Post,
    path: "v7.0/migration_plans/{migration_plan_id}/vps_migrations",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("vps_migration"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cleanup_data",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "dst_node",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "maintenance_window",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "vps_migration",
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
    pub fn cleanup_data(mut self, value: bool) -> Self {
        self.input_mut().cleanup_data = runtime::InputValue::Value(value);
        self
    }
    pub fn dst_node(mut self, value: i64) -> Self {
        self.input_mut().dst_node = runtime::InputValue::Value(value);
        self
    }
    pub fn maintenance_window(mut self, value: bool) -> Self {
        self.input_mut().maintenance_window = runtime::InputValue::Value(value);
        self
    }
    pub fn vps(mut self, value: i64) -> Self {
        self.input_mut().vps = runtime::InputValue::Value(value);
        self
    }
}
