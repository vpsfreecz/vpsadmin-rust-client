use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cs_note",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cs_note: runtime::InputValue<String>,
    #[serde(
        rename = "en_note",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub en_note: runtime::InputValue<String>,
    #[serde(
        rename = "expected_content_revision",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub expected_content_revision: runtime::InputValue<i64>,
    #[serde(
        rename = "mitigated_since",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub mitigated_since: runtime::InputValue<String>,
    #[serde(
        rename = "state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub state: runtime::InputValue<String>,
    #[serde(
        rename = "vulnerable_until",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub vulnerable_until: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cs_note)?;
        runtime::Validate::validate(&self.en_note)?;
        runtime::Validate::validate(&self.expected_content_revision)?;
        runtime::Validate::validate(&self.mitigated_since)?;
        runtime::Validate::validate(&self.state)?;
        runtime::Validate::validate(&self.vulnerable_until)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "cs_note")]
    pub cs_note: Option<String>,
    #[serde(default, rename = "en_note")]
    pub en_note: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "mitigated_since")]
    pub mitigated_since: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "security_advisory")]
    pub security_advisory: Option<runtime::Association>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "vulnerable_until")]
    pub vulnerable_until: Option<String>,
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
    name: "security_advisory/node_status/update",
    method: runtime::Method::Put,
    path: "v7.0/security_advisories/{security_advisory_id}/node_statuses/{node_status_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node_status"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cs_note",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "en_note",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "expected_content_revision",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "mitigated_since",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "state",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vulnerable_until",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: true,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "node_status",
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
    pub fn cs_note(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cs_note = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cs_note_null(mut self) -> Self {
        self.input_mut().cs_note = runtime::InputValue::Null;
        self
    }
    pub fn en_note(mut self, value: impl Into<String>) -> Self {
        self.input_mut().en_note = runtime::InputValue::Value(value.into());
        self
    }
    pub fn en_note_null(mut self) -> Self {
        self.input_mut().en_note = runtime::InputValue::Null;
        self
    }
    pub fn expected_content_revision(mut self, value: i64) -> Self {
        self.input_mut().expected_content_revision = runtime::InputValue::Value(value);
        self
    }
    pub fn mitigated_since(mut self, value: impl Into<String>) -> Self {
        self.input_mut().mitigated_since = runtime::InputValue::Value(value.into());
        self
    }
    pub fn mitigated_since_null(mut self) -> Self {
        self.input_mut().mitigated_since = runtime::InputValue::Null;
        self
    }
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().state = runtime::InputValue::Value(value.into());
        self
    }
    pub fn vulnerable_until(mut self, value: impl Into<String>) -> Self {
        self.input_mut().vulnerable_until = runtime::InputValue::Value(value.into());
        self
    }
    pub fn vulnerable_until_null(mut self) -> Self {
        self.input_mut().vulnerable_until = runtime::InputValue::Null;
        self
    }
}
