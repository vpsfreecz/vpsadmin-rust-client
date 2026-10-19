use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
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
    name: "security_advisory/show",
    method: runtime::Method::Get,
    path: "v7.0/security_advisories/{security_advisory_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("security_advisory"),
        parameters: &[],
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {}
