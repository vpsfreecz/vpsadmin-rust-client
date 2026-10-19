use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "from", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub from: runtime::InputValue<String>,
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
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
    #[serde(
        rename = "node_active",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub node_active: runtime::InputValue<bool>,
    #[serde(
        rename = "node_kernel_history_state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub node_kernel_history_state: runtime::InputValue<i64>,
    #[serde(rename = "to", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub to: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.from)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.node_active)?;
        runtime::Validate::validate(&self.node_kernel_history_state)?;
        runtime::Validate::validate(&self.to)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "from")]
    pub from: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "node_kernel_history_state")]
    pub node_kernel_history_state: Option<runtime::Association>,
    #[serde(default, rename = "reason")]
    pub reason: Option<String>,
    #[serde(default, rename = "to")]
    pub to: Option<String>,
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
    name: "node_kernel_history_gap/index",
    method: runtime::Method::Get,
    path: "v7.0/node_kernel_history_gaps",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node_kernel_history_gap"),
        parameters: &[
            runtime::ParameterSpec {
                name: "from",
                kind: runtime::ParameterKind::Datetime,
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
                name: "node_kernel_history_state",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "to",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "node_kernel_history_gaps",
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
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.input_mut().from = runtime::InputValue::Value(value.into());
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
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
        self
    }
    pub fn node_active(mut self, value: bool) -> Self {
        self.input_mut().node_active = runtime::InputValue::Value(value);
        self
    }
    pub fn node_kernel_history_state(mut self, value: i64) -> Self {
        self.input_mut().node_kernel_history_state = runtime::InputValue::Value(value);
        self
    }
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.input_mut().to = runtime::InputValue::Value(value.into());
        self
    }
}
