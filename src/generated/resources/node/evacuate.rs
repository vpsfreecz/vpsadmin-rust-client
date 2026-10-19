use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cleanup_data",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cleanup_data: runtime::InputValue<bool>,
    #[serde(
        rename = "concurrency",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub concurrency: runtime::InputValue<i64>,
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
    #[serde(
        rename = "reason",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reason: runtime::InputValue<String>,
    #[serde(
        rename = "send_mail",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub send_mail: runtime::InputValue<bool>,
    #[serde(
        rename = "stop_on_error",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub stop_on_error: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cleanup_data)?;
        runtime::Validate::validate(&self.concurrency)?;
        runtime::Validate::validate(&self.dst_node)?;
        runtime::Validate::validate(&self.maintenance_window)?;
        runtime::Validate::validate(&self.reason)?;
        runtime::Validate::validate(&self.send_mail)?;
        runtime::Validate::validate(&self.stop_on_error)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "migration_plan_id")]
    pub migration_plan_id: Option<i64>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "node/evacuate",
    method: runtime::Method::Post,
    path: "v7.0/nodes/{node_id}/evacuate",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cleanup_data",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "concurrency",
                kind: runtime::ParameterKind::Integer,
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
                name: "reason",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "send_mail",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "stop_on_error",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Hash,
        namespace: "node",
        allow_empty_response: false,
    }),
    meta_namespace: Some("_meta"),
    global_meta: &[runtime::ParameterSpec {
        name: "no",
        kind: runtime::ParameterKind::Boolean,
        required: false,
        nullable: false,
        protected: false,
    }],
    object_meta: &[],
    auth: runtime::RequestAuth::Configured,
};
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {
    pub fn cleanup_data(mut self, value: bool) -> Self {
        self.input_mut().cleanup_data = runtime::InputValue::Value(value);
        self
    }
    pub fn concurrency(mut self, value: i64) -> Self {
        self.input_mut().concurrency = runtime::InputValue::Value(value);
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
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reason = runtime::InputValue::Value(value.into());
        self
    }
    pub fn send_mail(mut self, value: bool) -> Self {
        self.input_mut().send_mail = runtime::InputValue::Value(value);
        self
    }
    pub fn stop_on_error(mut self, value: bool) -> Self {
        self.input_mut().stop_on_error = runtime::InputValue::Value(value);
        self
    }
}
