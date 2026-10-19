use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "from_duration",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_duration: runtime::InputValue<f64>,
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
        rename = "monitor",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub monitor: runtime::InputValue<String>,
    #[serde(
        rename = "object_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub object_id: runtime::InputValue<i64>,
    #[serde(
        rename = "object_name",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub object_name: runtime::InputValue<String>,
    #[serde(
        rename = "order",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub order: runtime::InputValue<String>,
    #[serde(
        rename = "state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub state: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.from_duration)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.monitor)?;
        runtime::Validate::validate(&self.object_id)?;
        runtime::Validate::validate(&self.object_name)?;
        runtime::Validate::validate(&self.order)?;
        runtime::Validate::validate(&self.state)?;
        runtime::Validate::validate(&self.user)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "duration")]
    pub duration: Option<f64>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "issue")]
    pub issue: Option<String>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "monitor")]
    pub monitor: Option<String>,
    #[serde(default, rename = "object_id")]
    pub object_id: Option<i64>,
    #[serde(default, rename = "object_name")]
    pub object_name: Option<String>,
    #[serde(default, rename = "saved_until")]
    pub saved_until: Option<String>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
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
    name: "monitored_event/index",
    method: runtime::Method::Get,
    path: "v7.0/monitored_events",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("monitored_event"),
        parameters: &[
            runtime::ParameterSpec {
                name: "from_duration",
                kind: runtime::ParameterKind::Float,
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
                name: "monitor",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "object_id",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "object_name",
                kind: runtime::ParameterKind::String,
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
                name: "state",
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
        namespace: "monitored_events",
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
    pub fn from_duration(mut self, value: f64) -> Self {
        self.input_mut().from_duration = runtime::InputValue::Value(value);
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
    pub fn monitor(mut self, value: impl Into<String>) -> Self {
        self.input_mut().monitor = runtime::InputValue::Value(value.into());
        self
    }
    pub fn object_id(mut self, value: i64) -> Self {
        self.input_mut().object_id = runtime::InputValue::Value(value);
        self
    }
    pub fn object_name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().object_name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.input_mut().order = runtime::InputValue::Value(value.into());
        self
    }
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().state = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
}
