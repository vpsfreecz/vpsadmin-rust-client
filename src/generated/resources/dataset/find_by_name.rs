use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.user)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "atime")]
    pub atime: Option<bool>,
    #[serde(default, rename = "avail")]
    pub avail: Option<i64>,
    #[serde(default, rename = "compression")]
    pub compression: Option<bool>,
    #[serde(default, rename = "compressratio")]
    pub compressratio: Option<f64>,
    #[serde(default, rename = "current_history_id")]
    pub current_history_id: Option<i64>,
    #[doc = "Association keys: ID=id, label=created_at"]
    #[serde(default, rename = "dataset_expansion")]
    pub dataset_expansion: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "environment")]
    pub environment: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=path"]
    #[serde(default, rename = "export")]
    pub export: Option<runtime::Association>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "parent")]
    pub parent: Option<runtime::Association>,
    #[serde(default, rename = "quota")]
    pub quota: Option<i64>,
    #[serde(default, rename = "recordsize")]
    pub recordsize: Option<i64>,
    #[serde(default, rename = "refcompressratio")]
    pub refcompressratio: Option<f64>,
    #[serde(default, rename = "referenced")]
    pub referenced: Option<i64>,
    #[serde(default, rename = "refquota")]
    pub refquota: Option<i64>,
    #[serde(default, rename = "relatime")]
    pub relatime: Option<bool>,
    #[serde(default, rename = "sharenfs")]
    pub sharenfs: Option<String>,
    #[serde(default, rename = "sync")]
    pub sync: Option<String>,
    #[serde(default, rename = "used")]
    pub used: Option<i64>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=hostname"]
    #[serde(default, rename = "vps")]
    pub vps: Option<runtime::Association>,
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
    name: "dataset/find_by_name",
    method: runtime::Method::Get,
    path: "v7.0/datasets/find_by_name",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dataset"),
        parameters: &[
            runtime::ParameterSpec {
                name: "name",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "dataset",
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
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
}
