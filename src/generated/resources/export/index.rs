use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.user)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "all_vps")]
    pub all_vps: Option<bool>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "dataset")]
    pub dataset: Option<runtime::Association>,
    #[serde(default, rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "expiration_date")]
    pub expiration_date: Option<String>,
    #[doc = "Association keys: ID=id, label=addr"]
    #[serde(default, rename = "host_ip_address")]
    pub host_ip_address: Option<runtime::Association>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=addr"]
    #[serde(default, rename = "ip_address")]
    pub ip_address: Option<runtime::Association>,
    #[serde(default, rename = "path")]
    pub path: Option<String>,
    #[serde(default, rename = "root_squash")]
    pub root_squash: Option<bool>,
    #[serde(default, rename = "rw")]
    pub rw: Option<bool>,
    #[doc = "Association keys: ID=id, label=created_at"]
    #[serde(default, rename = "snapshot")]
    pub snapshot: Option<runtime::Association>,
    #[serde(default, rename = "subtree_check")]
    pub subtree_check: Option<bool>,
    #[serde(default, rename = "sync")]
    pub sync: Option<bool>,
    #[serde(default, rename = "threads")]
    pub threads: Option<i64>,
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "export/index",
    method: runtime::Method::Get,
    path: "v7.0/exports",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("export"),
        parameters: &[
            runtime::ParameterSpec {
                name: "from_id",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "limit",
                kind: runtime::ParameterKind::Integer,
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
        layout: runtime::Layout::ObjectList,
        namespace: "exports",
        allow_empty_response: false,
    }),
    meta_namespace: Some("_meta"),
    global_meta: &[
        runtime::ParameterSpec {
            name: "count",
            kind: runtime::ParameterKind::Boolean,
            required: false,
            nullable: false,
            protected: false,
        },
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Vec<Output>, Meta> {
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
}
