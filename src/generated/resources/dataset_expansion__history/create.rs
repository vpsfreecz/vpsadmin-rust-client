use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "added_space",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub added_space: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.added_space)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "added_space")]
    pub added_space: Option<i64>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "admin")]
    pub admin: Option<runtime::Association>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "new_refquota")]
    pub new_refquota: Option<i64>,
    #[serde(default, rename = "original_refquota")]
    pub original_refquota: Option<i64>,
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
    name: "dataset_expansion/history/create",
    method: runtime::Method::Post,
    path: "v7.0/dataset_expansions/{dataset_expansion_id}/history",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("history"),
        parameters: &[runtime::ParameterSpec {
            name: "added_space",
            kind: runtime::ParameterKind::Integer,
            required: true,
            nullable: false,
            protected: false,
        }],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "history",
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
    pub fn added_space(mut self, value: i64) -> Self {
        self.input_mut().added_space = runtime::InputValue::Value(value);
        self
    }
}
