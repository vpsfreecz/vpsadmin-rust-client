use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cs_message",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cs_message: runtime::InputValue<String>,
    #[serde(
        rename = "cs_summary",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cs_summary: runtime::InputValue<String>,
    #[serde(
        rename = "en_message",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub en_message: runtime::InputValue<String>,
    #[serde(
        rename = "en_summary",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub en_summary: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cs_message)?;
        runtime::Validate::validate(&self.cs_summary)?;
        runtime::Validate::validate(&self.en_message)?;
        runtime::Validate::validate(&self.en_summary)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "cs_message")]
    pub cs_message: Option<String>,
    #[serde(default, rename = "cs_summary")]
    pub cs_summary: Option<String>,
    #[serde(default, rename = "en_message")]
    pub en_message: Option<String>,
    #[serde(default, rename = "en_summary")]
    pub en_summary: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "reported_by")]
    pub reported_by: Option<runtime::Association>,
    #[serde(default, rename = "reporter_name")]
    pub reporter_name: Option<String>,
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "security_advisory")]
    pub security_advisory: Option<runtime::Association>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
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
    name: "security_advisory_update/update",
    method: runtime::Method::Put,
    path: "v7.0/security_advisory_updates/{security_advisory_update_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("security_advisory_update"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cs_message",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "cs_summary",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "en_message",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "en_summary",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "security_advisory_update",
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
    pub fn cs_message(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cs_message = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cs_message_null(mut self) -> Self {
        self.input_mut().cs_message = runtime::InputValue::Null;
        self
    }
    pub fn cs_summary(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cs_summary = runtime::InputValue::Value(value.into());
        self
    }
    pub fn en_message(mut self, value: impl Into<String>) -> Self {
        self.input_mut().en_message = runtime::InputValue::Value(value.into());
        self
    }
    pub fn en_message_null(mut self) -> Self {
        self.input_mut().en_message = runtime::InputValue::Null;
        self
    }
    pub fn en_summary(mut self, value: impl Into<String>) -> Self {
        self.input_mut().en_summary = runtime::InputValue::Value(value.into());
        self
    }
}
