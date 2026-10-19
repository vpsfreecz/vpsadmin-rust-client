use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cve_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cve_id: runtime::InputValue<String>,
    #[serde(
        rename = "expected_content_revision",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub expected_content_revision: runtime::InputValue<i64>,
    #[serde(
        rename = "security_advisory",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub security_advisory: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cve_id)?;
        runtime::Validate::validate(&self.expected_content_revision)?;
        runtime::Validate::validate(&self.security_advisory)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "cve_id")]
    pub cve_id: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "security_advisory")]
    pub security_advisory: Option<runtime::Association>,
    #[serde(default, rename = "security_advisory_id")]
    pub security_advisory_id: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
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
    name: "security_advisory_cve/create",
    method: runtime::Method::Post,
    path: "v7.0/security_advisory_cves",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("security_advisory_cve"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cve_id",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
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
                name: "security_advisory",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "security_advisory_cve",
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
    pub fn cve_id(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cve_id = runtime::InputValue::Value(value.into());
        self
    }
    pub fn expected_content_revision(mut self, value: i64) -> Self {
        self.input_mut().expected_content_revision = runtime::InputValue::Value(value);
        self
    }
    pub fn security_advisory(mut self, value: i64) -> Self {
        self.input_mut().security_advisory = runtime::InputValue::Value(value);
        self
    }
}
