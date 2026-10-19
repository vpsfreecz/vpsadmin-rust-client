use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cluster_resource_package",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cluster_resource_package: runtime::InputValue<i64>,
    #[serde(
        rename = "comment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub comment: runtime::InputValue<String>,
    #[serde(
        rename = "environment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub environment: runtime::InputValue<i64>,
    #[serde(
        rename = "from_personal",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_personal: runtime::InputValue<bool>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cluster_resource_package)?;
        runtime::Validate::validate(&self.comment)?;
        runtime::Validate::validate(&self.environment)?;
        runtime::Validate::validate(&self.from_personal)?;
        runtime::Validate::validate(&self.user)?;
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
    name: "user_cluster_resource_package/create",
    method: runtime::Method::Post,
    path: "v7.0/user_cluster_resource_packages",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("user_cluster_resource_package"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cluster_resource_package",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "comment",
                kind: runtime::ParameterKind::Text,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "environment",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "from_personal",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
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
    pub fn cluster_resource_package(mut self, value: i64) -> Self {
        self.input_mut().cluster_resource_package = runtime::InputValue::Value(value);
        self
    }
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.input_mut().comment = runtime::InputValue::Value(value.into());
        self
    }
    pub fn environment(mut self, value: i64) -> Self {
        self.input_mut().environment = runtime::InputValue::Value(value);
        self
    }
    pub fn from_personal(mut self, value: bool) -> Self {
        self.input_mut().from_personal = runtime::InputValue::Value(value);
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
}
