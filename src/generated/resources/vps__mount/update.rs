use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "master_enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub master_enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "on_start_fail",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub on_start_fail: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.enabled)?;
        runtime::Validate::validate(&self.master_enabled)?;
        runtime::Validate::validate(&self.on_start_fail)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "current_state")]
    pub current_state: Option<String>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "dataset")]
    pub dataset: Option<runtime::Association>,
    #[serde(default, rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "expiration_date")]
    pub expiration_date: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "master_enabled")]
    pub master_enabled: Option<bool>,
    #[serde(default, rename = "mode")]
    pub mode: Option<String>,
    #[serde(default, rename = "mountpoint")]
    pub mountpoint: Option<String>,
    #[serde(default, rename = "on_start_fail")]
    pub on_start_fail: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "user_namespace_map")]
    pub user_namespace_map: Option<runtime::Association>,
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
    name: "vps/mount/update",
    method: runtime::Method::Put,
    path: "v7.0/vpses/{vps_id}/mounts/{mount_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("mount"),
        parameters: &[
            runtime::ParameterSpec {
                name: "enabled",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "master_enabled",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "on_start_fail",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "mount",
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
    pub fn enabled(mut self, value: bool) -> Self {
        self.input_mut().enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn master_enabled(mut self, value: bool) -> Self {
        self.input_mut().master_enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn on_start_fail(mut self, value: impl Into<String>) -> Self {
        self.input_mut().on_start_fail = runtime::InputValue::Value(value.into());
        self
    }
}
