use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "dataset",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dataset: runtime::InputValue<i64>,
    #[serde(
        rename = "enable_notifications",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_notifications: runtime::InputValue<bool>,
    #[serde(
        rename = "enable_shrink",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_shrink: runtime::InputValue<bool>,
    #[serde(
        rename = "max_over_refquota_seconds",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub max_over_refquota_seconds: runtime::InputValue<i64>,
    #[serde(
        rename = "original_refquota",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub original_refquota: runtime::InputValue<i64>,
    #[serde(
        rename = "stop_vps",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub stop_vps: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.dataset)?;
        runtime::Validate::validate(&self.enable_notifications)?;
        runtime::Validate::validate(&self.enable_shrink)?;
        runtime::Validate::validate(&self.max_over_refquota_seconds)?;
        runtime::Validate::validate(&self.original_refquota)?;
        runtime::Validate::validate(&self.stop_vps)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "added_space")]
    pub added_space: Option<i64>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "dataset")]
    pub dataset: Option<runtime::Association>,
    #[serde(default, rename = "enable_notifications")]
    pub enable_notifications: Option<bool>,
    #[serde(default, rename = "enable_shrink")]
    pub enable_shrink: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "max_over_refquota_seconds")]
    pub max_over_refquota_seconds: Option<i64>,
    #[serde(default, rename = "original_refquota")]
    pub original_refquota: Option<i64>,
    #[serde(default, rename = "over_refquota_seconds")]
    pub over_refquota_seconds: Option<i64>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "stop_vps")]
    pub stop_vps: Option<bool>,
    #[doc = "Association keys: ID=id, label=hostname"]
    #[serde(default, rename = "vps")]
    pub vps: Option<runtime::Association>,
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
    name: "dataset_expansion/register_expanded",
    method: runtime::Method::Post,
    path: "v7.0/dataset_expansions/register_expanded",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dataset_expansion"),
        parameters: &[
            runtime::ParameterSpec {
                name: "dataset",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_notifications",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_shrink",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "max_over_refquota_seconds",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "original_refquota",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "stop_vps",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "dataset_expansion",
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
    pub fn dataset(mut self, value: i64) -> Self {
        self.input_mut().dataset = runtime::InputValue::Value(value);
        self
    }
    pub fn enable_notifications(mut self, value: bool) -> Self {
        self.input_mut().enable_notifications = runtime::InputValue::Value(value);
        self
    }
    pub fn enable_shrink(mut self, value: bool) -> Self {
        self.input_mut().enable_shrink = runtime::InputValue::Value(value);
        self
    }
    pub fn max_over_refquota_seconds(mut self, value: i64) -> Self {
        self.input_mut().max_over_refquota_seconds = runtime::InputValue::Value(value);
        self
    }
    pub fn original_refquota(mut self, value: i64) -> Self {
        self.input_mut().original_refquota = runtime::InputValue::Value(value);
        self
    }
    pub fn stop_vps(mut self, value: bool) -> Self {
        self.input_mut().stop_vps = runtime::InputValue::Value(value);
        self
    }
}
