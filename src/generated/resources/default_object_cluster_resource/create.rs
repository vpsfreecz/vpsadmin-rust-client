use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "class_name",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub class_name: runtime::InputValue<String>,
    #[serde(
        rename = "cluster_resource",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cluster_resource: runtime::InputValue<i64>,
    #[serde(
        rename = "environment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub environment: runtime::InputValue<i64>,
    #[serde(
        rename = "value",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub value: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.class_name)?;
        runtime::Validate::validate(&self.cluster_resource)?;
        runtime::Validate::validate(&self.environment)?;
        runtime::Validate::validate(&self.value)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "class_name")]
    pub class_name: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "cluster_resource")]
    pub cluster_resource: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "environment")]
    pub environment: Option<runtime::Association>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "value")]
    pub value: Option<i64>,
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
    name: "default_object_cluster_resource/create",
    method: runtime::Method::Post,
    path: "v7.0/default_object_cluster_resources",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("default_object_cluster_resource"),
        parameters: &[
            runtime::ParameterSpec {
                name: "class_name",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "cluster_resource",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "environment",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "value",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "default_object_cluster_resource",
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
    pub fn class_name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().class_name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cluster_resource(mut self, value: i64) -> Self {
        self.input_mut().cluster_resource = runtime::InputValue::Value(value);
        self
    }
    pub fn environment(mut self, value: i64) -> Self {
        self.input_mut().environment = runtime::InputValue::Value(value);
        self
    }
    pub fn value(mut self, value: i64) -> Self {
        self.input_mut().value = runtime::InputValue::Value(value);
        self
    }
}
