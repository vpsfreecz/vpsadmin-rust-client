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
    #[serde(rename = "to", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub to: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.from)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.to)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "booted_at")]
    pub booted_at: Option<String>,
    #[serde(default, rename = "booted_release")]
    pub booted_release: Option<String>,
    #[serde(default, rename = "confidence")]
    pub confidence: Option<String>,
    #[serde(default, rename = "current")]
    pub current: Option<bool>,
    #[serde(default, rename = "effective_at")]
    pub effective_at: Option<String>,
    #[serde(default, rename = "event_type")]
    pub event_type: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "livepatch_action")]
    pub livepatch_action: Option<String>,
    #[serde(default, rename = "observed_after")]
    pub observed_after: Option<String>,
    #[serde(default, rename = "observed_before")]
    pub observed_before: Option<String>,
    #[serde(default, rename = "reported_release")]
    pub reported_release: Option<String>,
    #[serde(default, rename = "source")]
    pub source: Option<String>,
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
    name: "node/kernel_history/index",
    method: runtime::Method::Get,
    path: "v7.0/nodes/{node_id}/kernel_history",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("kernel_history"),
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
        namespace: "kernel_histories",
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
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.input_mut().to = runtime::InputValue::Value(value.into());
        self
    }
}
