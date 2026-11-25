use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "backup_first",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub backup_first: runtime::InputValue<bool>,
    #[serde(
        rename = "recursive",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub recursive: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.backup_first)?;
        runtime::Validate::validate(&self.recursive)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(default, rename = "action_state_id")]
    pub action_state_id: Option<i64>,
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "dataset/disable_backups",
    method: runtime::Method::Post,
    path: "v7.0/datasets/{dataset_id}/disable_backups",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dataset"),
        parameters: &[
            runtime::ParameterSpec {
                name: "backup_first",
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
        layout: runtime::Layout::Object,
        namespace: "dataset",
        allow_empty_response: true,
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
    pub fn backup_first(mut self, value: bool) -> Self {
        self.input_mut().backup_first = runtime::InputValue::Value(value);
        self
    }
    pub fn recursive(mut self, value: bool) -> Self {
        self.input_mut().recursive = runtime::InputValue::Value(value);
        self
    }
}
