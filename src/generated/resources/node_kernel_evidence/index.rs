use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
    #[serde(
        rename = "node_active",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub node_active: runtime::InputValue<bool>,
    #[serde(
        rename = "offset",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub offset: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.node_active)?;
        runtime::Validate::validate(&self.offset)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "boot_id")]
    pub boot_id: Option<String>,
    #[serde(default, rename = "booted_at")]
    pub booted_at: Option<String>,
    #[serde(default, rename = "booted_release")]
    pub booted_release: Option<String>,
    #[serde(default, rename = "booted_system")]
    pub booted_system: Option<String>,
    #[serde(default, rename = "current_system")]
    pub current_system: Option<String>,
    #[serde(default, rename = "evidence_revision")]
    pub evidence_revision: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "kernel_command_line")]
    pub kernel_command_line: Option<String>,
    #[serde(default, rename = "kernel_config_available")]
    pub kernel_config_available: Option<bool>,
    #[serde(default, rename = "kernel_config_digest")]
    pub kernel_config_digest: Option<String>,
    #[serde(default, rename = "kernel_source_revision")]
    pub kernel_source_revision: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[serde(default, rename = "observed_at")]
    pub observed_at: Option<String>,
    #[serde(default, rename = "received_at")]
    pub received_at: Option<String>,
    #[serde(default, rename = "report_schema_version")]
    pub report_schema_version: Option<i64>,
    #[serde(default, rename = "reported_release")]
    pub reported_release: Option<String>,
    #[serde(default, rename = "snapshot_revision")]
    pub snapshot_revision: Option<String>,
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
    name: "node_kernel_evidence/index",
    method: runtime::Method::Get,
    path: "v7.0/node_kernel_evidences",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node_kernel_evidence"),
        parameters: &[
            runtime::ParameterSpec {
                name: "limit",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "node",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "node_active",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "offset",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "node_kernel_evidences",
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
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
        self
    }
    pub fn node_active(mut self, value: bool) -> Self {
        self.input_mut().node_active = runtime::InputValue::Value(value);
        self
    }
    pub fn offset(mut self, value: i64) -> Self {
        self.input_mut().offset = runtime::InputValue::Value(value);
        self
    }
}
