use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "allocate_chain",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub allocate_chain: runtime::InputValue<String>,
    #[serde(
        rename = "free_chain",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub free_chain: runtime::InputValue<String>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(rename = "max", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub max: runtime::InputValue<i64>,
    #[serde(rename = "min", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub min: runtime::InputValue<i64>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(
        rename = "resource_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub resource_type: runtime::InputValue<String>,
    #[serde(
        rename = "stepsize",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub stepsize: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.allocate_chain)?;
        runtime::Validate::validate(&self.free_chain)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.max)?;
        runtime::Validate::validate(&self.min)?;
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.resource_type)?;
        runtime::Validate::validate(&self.stepsize)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "allocate_chain")]
    pub allocate_chain: Option<String>,
    #[serde(default, rename = "free_chain")]
    pub free_chain: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "max")]
    pub max: Option<i64>,
    #[serde(default, rename = "min")]
    pub min: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "resource_type")]
    pub resource_type: Option<String>,
    #[serde(default, rename = "stepsize")]
    pub stepsize: Option<i64>,
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
    name: "cluster_resource/create",
    method: runtime::Method::Post,
    path: "v7.0/cluster_resources",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("cluster_resource"),
        parameters: &[
            runtime::ParameterSpec {
                name: "allocate_chain",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "free_chain",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "label",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "max",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "min",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "name",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "resource_type",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "stepsize",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "cluster_resource",
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
    pub fn allocate_chain(mut self, value: impl Into<String>) -> Self {
        self.input_mut().allocate_chain = runtime::InputValue::Value(value.into());
        self
    }
    pub fn free_chain(mut self, value: impl Into<String>) -> Self {
        self.input_mut().free_chain = runtime::InputValue::Value(value.into());
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn max(mut self, value: i64) -> Self {
        self.input_mut().max = runtime::InputValue::Value(value);
        self
    }
    pub fn min(mut self, value: i64) -> Self {
        self.input_mut().min = runtime::InputValue::Value(value);
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn resource_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().resource_type = runtime::InputValue::Value(value.into());
        self
    }
    pub fn stepsize(mut self, value: i64) -> Self {
        self.input_mut().stepsize = runtime::InputValue::Value(value);
        self
    }
}
