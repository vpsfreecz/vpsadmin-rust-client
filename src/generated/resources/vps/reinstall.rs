use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "os_template",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub os_template: runtime::InputValue<i64>,
    #[serde(
        rename = "user_data_content",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub user_data_content: runtime::InputValue<String>,
    #[serde(
        rename = "user_data_format",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub user_data_format: runtime::InputValue<String>,
    #[serde(
        rename = "vps_user_data",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub vps_user_data: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.os_template)?;
        runtime::Validate::validate(&self.user_data_content)?;
        runtime::Validate::validate(&self.user_data_format)?;
        runtime::Validate::validate(&self.vps_user_data)?;
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
    name: "vps/reinstall",
    method: runtime::Method::Post,
    path: "v7.0/vpses/{vps_id}/reinstall",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("vps"),
        parameters: &[
            runtime::ParameterSpec {
                name: "os_template",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user_data_content",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user_data_format",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps_user_data",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "vps",
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
    pub fn os_template(mut self, value: i64) -> Self {
        self.input_mut().os_template = runtime::InputValue::Value(value);
        self
    }
    pub fn user_data_content(mut self, value: impl Into<String>) -> Self {
        self.input_mut().user_data_content = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user_data_format(mut self, value: impl Into<String>) -> Self {
        self.input_mut().user_data_format = runtime::InputValue::Value(value.into());
        self
    }
    pub fn vps_user_data(mut self, value: i64) -> Self {
        self.input_mut().vps_user_data = runtime::InputValue::Value(value);
        self
    }
}
