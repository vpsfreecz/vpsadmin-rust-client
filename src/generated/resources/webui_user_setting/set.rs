use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "value",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub value: runtime::InputValue<serde_json::Value>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.value)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "key")]
    pub key: Option<String>,
    #[serde(default, rename = "namespace")]
    pub namespace: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[serde(default, rename = "value")]
    pub value: Option<serde_json::Value>,
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
    name: "webui_user_setting/set",
    method: runtime::Method::Put,
    path: "v7.0/webui_user_settings/{namespace}/{key}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("webui_user_setting"),
        parameters: &[runtime::ParameterSpec {
            name: "value",
            kind: runtime::ParameterKind::Custom,
            required: true,
            nullable: false,
            protected: false,
        }],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "webui_user_setting",
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
    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.input_mut().value = runtime::InputValue::Value(value);
        self
    }
}
