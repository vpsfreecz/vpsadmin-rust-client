use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cs_description",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cs_description: runtime::InputValue<String>,
    #[serde(
        rename = "cs_response",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cs_response: runtime::InputValue<String>,
    #[serde(
        rename = "cs_summary",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cs_summary: runtime::InputValue<String>,
    #[serde(rename = "cve", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub cve: runtime::InputValue<String>,
    #[serde(
        rename = "en_description",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub en_description: runtime::InputValue<String>,
    #[serde(
        rename = "en_response",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub en_response: runtime::InputValue<String>,
    #[serde(
        rename = "en_summary",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub en_summary: runtime::InputValue<String>,
    #[serde(
        rename = "external_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub external_id: runtime::InputValue<String>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(
        rename = "published_at",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub published_at: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cs_description)?;
        runtime::Validate::validate(&self.cs_response)?;
        runtime::Validate::validate(&self.cs_summary)?;
        runtime::Validate::validate(&self.cve)?;
        runtime::Validate::validate(&self.en_description)?;
        runtime::Validate::validate(&self.en_response)?;
        runtime::Validate::validate(&self.en_summary)?;
        runtime::Validate::validate(&self.external_id)?;
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.published_at)?;
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
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "security_advisory/create",
    method: runtime::Method::Post,
    path: "v7.0/security_advisories",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("security_advisory"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cs_description",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "cs_response",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
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
                name: "cve",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "en_description",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "en_response",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "en_summary",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "external_id",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "name",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "published_at",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: true,
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
    pub fn cs_description(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cs_description = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cs_response(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cs_response = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cs_summary(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cs_summary = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cve(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cve = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cve_null(mut self) -> Self {
        self.input_mut().cve = runtime::InputValue::Null;
        self
    }
    pub fn en_description(mut self, value: impl Into<String>) -> Self {
        self.input_mut().en_description = runtime::InputValue::Value(value.into());
        self
    }
    pub fn en_response(mut self, value: impl Into<String>) -> Self {
        self.input_mut().en_response = runtime::InputValue::Value(value.into());
        self
    }
    pub fn en_summary(mut self, value: impl Into<String>) -> Self {
        self.input_mut().en_summary = runtime::InputValue::Value(value.into());
        self
    }
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.input_mut().external_id = runtime::InputValue::Value(value.into());
        self
    }
    pub fn external_id_null(mut self) -> Self {
        self.input_mut().external_id = runtime::InputValue::Null;
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn name_null(mut self) -> Self {
        self.input_mut().name = runtime::InputValue::Null;
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
}
