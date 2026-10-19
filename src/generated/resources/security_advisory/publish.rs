use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "expected_content_revision",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub expected_content_revision: runtime::InputValue<i64>,
    #[serde(
        rename = "published_at",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub published_at: runtime::InputValue<String>,
    #[serde(
        rename = "send_mail",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub send_mail: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.expected_content_revision)?;
        runtime::Validate::validate(&self.published_at)?;
        runtime::Validate::validate(&self.send_mail)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "affected")]
    pub affected: Option<bool>,
    #[serde(default, rename = "affected_node_count")]
    pub affected_node_count: Option<i64>,
    #[serde(default, rename = "affected_user_count")]
    pub affected_user_count: Option<i64>,
    #[serde(default, rename = "affected_vps_count")]
    pub affected_vps_count: Option<i64>,
    #[serde(default, rename = "content_revision")]
    pub content_revision: Option<i64>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "created_by")]
    pub created_by: Option<runtime::Association>,
    #[serde(default, rename = "cs_description")]
    pub cs_description: Option<String>,
    #[serde(default, rename = "cs_response")]
    pub cs_response: Option<String>,
    #[serde(default, rename = "cs_summary")]
    pub cs_summary: Option<String>,
    #[serde(default, rename = "en_description")]
    pub en_description: Option<String>,
    #[serde(default, rename = "en_response")]
    pub en_response: Option<String>,
    #[serde(default, rename = "en_summary")]
    pub en_summary: Option<String>,
    #[serde(default, rename = "external_id")]
    pub external_id: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "published_at")]
    pub published_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "published_by")]
    pub published_by: Option<runtime::Association>,
    #[serde(default, rename = "retracted_at")]
    pub retracted_at: Option<String>,
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
    name: "security_advisory/publish",
    method: runtime::Method::Post,
    path: "v7.0/security_advisories/{security_advisory_id}/publish",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("security_advisory"),
        parameters: &[
            runtime::ParameterSpec {
                name: "expected_content_revision",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "published_at",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "send_mail",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "security_advisory",
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
    pub fn expected_content_revision(mut self, value: i64) -> Self {
        self.input_mut().expected_content_revision = runtime::InputValue::Value(value);
        self
    }
    pub fn published_at(mut self, value: impl Into<String>) -> Self {
        self.input_mut().published_at = runtime::InputValue::Value(value.into());
        self
    }
    pub fn published_at_null(mut self) -> Self {
        self.input_mut().published_at = runtime::InputValue::Null;
        self
    }
    pub fn send_mail(mut self, value: bool) -> Self {
        self.input_mut().send_mail = runtime::InputValue::Value(value);
        self
    }
}
