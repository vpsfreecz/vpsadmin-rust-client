use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
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
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.root_squash)?;
        runtime::Validate::validate(&self.rw)?;
        runtime::Validate::validate(&self.subtree_check)?;
        runtime::Validate::validate(&self.sync)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=addr"]
    #[serde(default, rename = "ip_address")]
    pub ip_address: Option<runtime::Association>,
    #[serde(default, rename = "root_squash")]
    pub root_squash: Option<bool>,
    #[serde(default, rename = "rw")]
    pub rw: Option<bool>,
    #[serde(default, rename = "subtree_check")]
    pub subtree_check: Option<bool>,
    #[serde(default, rename = "sync")]
    pub sync: Option<bool>,
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
    name: "export/host/update",
    method: runtime::Method::Put,
    path: "v7.0/exports/{export_id}/hosts/{host_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("host"),
        parameters: &[
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
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "host",
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
}
