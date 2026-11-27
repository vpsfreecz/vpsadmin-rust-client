use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "disable_plans",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub disable_plans: runtime::InputValue<bool>,
    #[serde(
        rename = "recursive",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub recursive: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.disable_plans)?;
        runtime::Validate::validate(&self.recursive)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "manifest")]
    pub manifest: Option<serde_json::Value>,
    #[serde(default, rename = "manifest_digest")]
    pub manifest_digest: Option<String>,
    #[serde(default, rename = "manifest_version")]
    pub manifest_version: Option<i64>,
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
    name: "dataset/preview_snapshot_removal",
    method: runtime::Method::Post,
    path: "v7.0/datasets/{dataset_id}/preview_snapshot_removal",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dataset"),
        parameters: &[
            runtime::ParameterSpec {
                name: "disable_plans",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "recursive",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Hash,
        namespace: "dataset",
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
    pub fn disable_plans(mut self, value: bool) -> Self {
        self.input_mut().disable_plans = runtime::InputValue::Value(value);
        self
    }
    pub fn recursive(mut self, value: bool) -> Self {
        self.input_mut().recursive = runtime::InputValue::Value(value);
        self
    }
}
