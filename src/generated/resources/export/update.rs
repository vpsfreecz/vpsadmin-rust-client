use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "all_vps",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub all_vps: runtime::InputValue<bool>,
    #[serde(
        rename = "enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "root_squash",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub root_squash: runtime::InputValue<bool>,
    #[serde(rename = "rw", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub rw: runtime::InputValue<bool>,
    #[serde(
        rename = "subtree_check",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub subtree_check: runtime::InputValue<bool>,
    #[serde(rename = "sync", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub sync: runtime::InputValue<bool>,
    #[serde(
        rename = "threads",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub threads: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.all_vps)?;
        runtime::Validate::validate(&self.enabled)?;
        runtime::Validate::validate(&self.root_squash)?;
        runtime::Validate::validate(&self.rw)?;
        runtime::Validate::validate(&self.subtree_check)?;
        runtime::Validate::validate(&self.sync)?;
        runtime::Validate::validate(&self.threads)?;
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
    name: "export/update",
    method: runtime::Method::Put,
    path: "v7.0/exports/{export_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("export"),
        parameters: &[
            runtime::ParameterSpec {
                name: "all_vps",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enabled",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "root_squash",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "rw",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "subtree_check",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "sync",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "threads",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "export",
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
    pub fn all_vps(mut self, value: bool) -> Self {
        self.input_mut().all_vps = runtime::InputValue::Value(value);
        self
    }
    pub fn enabled(mut self, value: bool) -> Self {
        self.input_mut().enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn root_squash(mut self, value: bool) -> Self {
        self.input_mut().root_squash = runtime::InputValue::Value(value);
        self
    }
    pub fn rw(mut self, value: bool) -> Self {
        self.input_mut().rw = runtime::InputValue::Value(value);
        self
    }
    pub fn subtree_check(mut self, value: bool) -> Self {
        self.input_mut().subtree_check = runtime::InputValue::Value(value);
        self
    }
    pub fn sync(mut self, value: bool) -> Self {
        self.input_mut().sync = runtime::InputValue::Value(value);
        self
    }
    pub fn threads(mut self, value: i64) -> Self {
        self.input_mut().threads = runtime::InputValue::Value(value);
        self
    }
}
