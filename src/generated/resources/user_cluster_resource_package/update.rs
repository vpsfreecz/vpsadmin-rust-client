use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "comment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub comment: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.comment)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "added_by")]
    pub added_by: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "cluster_resource_package")]
    pub cluster_resource_package: Option<runtime::Association>,
    #[serde(default, rename = "comment")]
    pub comment: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "environment")]
    pub environment: Option<runtime::Association>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "is_personal")]
    pub is_personal: Option<bool>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
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
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "user_cluster_resource_package/update",
    method: runtime::Method::Put,
    path: "v7.0/user_cluster_resource_packages/{user_cluster_resource_package_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("user_cluster_resource_package"),
        parameters: &[runtime::ParameterSpec {
            name: "comment",
            kind: runtime::ParameterKind::Text,
            required: true,
            nullable: false,
            protected: false,
        }],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "user_cluster_resource_package",
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
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.input_mut().comment = runtime::InputValue::Value(value.into());
        self
    }
}
