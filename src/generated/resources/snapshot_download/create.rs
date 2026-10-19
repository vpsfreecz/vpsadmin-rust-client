use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "format",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub format: runtime::InputValue<String>,
    #[serde(
        rename = "from_snapshot",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_snapshot: runtime::InputValue<i64>,
    #[serde(
        rename = "send_mail",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub send_mail: runtime::InputValue<bool>,
    #[serde(
        rename = "snapshot",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub snapshot: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.format)?;
        runtime::Validate::validate(&self.from_snapshot)?;
        runtime::Validate::validate(&self.send_mail)?;
        runtime::Validate::validate(&self.snapshot)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "expiration_date")]
    pub expiration_date: Option<String>,
    #[serde(default, rename = "file_name")]
    pub file_name: Option<String>,
    #[serde(default, rename = "format")]
    pub format: Option<String>,
    #[doc = "Association keys: ID=id, label=created_at"]
    #[serde(default, rename = "from_snapshot")]
    pub from_snapshot: Option<runtime::Association>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "ready")]
    pub ready: Option<bool>,
    #[serde(default, rename = "sha256sum")]
    pub sha256sum: Option<String>,
    #[serde(default, rename = "size")]
    pub size: Option<i64>,
    #[doc = "Association keys: ID=id, label=created_at"]
    #[serde(default, rename = "snapshot")]
    pub snapshot: Option<runtime::Association>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
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
    name: "snapshot_download/create",
    method: runtime::Method::Post,
    path: "v7.0/snapshot_downloads",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("snapshot_download"),
        parameters: &[
            runtime::ParameterSpec {
                name: "format",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "from_snapshot",
                kind: runtime::ParameterKind::Resource,
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
                name: "snapshot",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "snapshot_download",
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
    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.input_mut().format = runtime::InputValue::Value(value.into());
        self
    }
    pub fn from_snapshot(mut self, value: i64) -> Self {
        self.input_mut().from_snapshot = runtime::InputValue::Value(value);
        self
    }
    pub fn send_mail(mut self, value: bool) -> Self {
        self.input_mut().send_mail = runtime::InputValue::Value(value);
        self
    }
    pub fn snapshot(mut self, value: i64) -> Self {
        self.input_mut().snapshot = runtime::InputValue::Value(value);
        self
    }
}
