use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "admin_lock_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub admin_lock_type: runtime::InputValue<String>,
    #[serde(
        rename = "admin_override",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub admin_override: runtime::InputValue<bool>,
    #[serde(
        rename = "atime",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub atime: runtime::InputValue<bool>,
    #[serde(
        rename = "compression",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub compression: runtime::InputValue<bool>,
    #[serde(
        rename = "quota",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub quota: runtime::InputValue<i64>,
    #[serde(
        rename = "recordsize",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub recordsize: runtime::InputValue<i64>,
    #[serde(
        rename = "refquota",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub refquota: runtime::InputValue<i64>,
    #[serde(
        rename = "relatime",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub relatime: runtime::InputValue<bool>,
    #[serde(
        rename = "sharenfs",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub sharenfs: runtime::InputValue<String>,
    #[serde(rename = "sync", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub sync: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.admin_lock_type)?;
        runtime::Validate::validate(&self.admin_override)?;
        runtime::Validate::validate(&self.atime)?;
        runtime::Validate::validate(&self.compression)?;
        runtime::Validate::validate(&self.quota)?;
        runtime::Validate::validate(&self.recordsize)?;
        runtime::Validate::validate(&self.refquota)?;
        runtime::Validate::validate(&self.relatime)?;
        runtime::Validate::validate(&self.sharenfs)?;
        runtime::Validate::validate(&self.sync)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
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
    name: "dataset/update",
    method: runtime::Method::Put,
    path: "v7.0/datasets/{dataset_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dataset"),
        parameters: &[
            runtime::ParameterSpec {
                name: "admin_lock_type",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "admin_override",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "atime",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "compression",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "quota",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "recordsize",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "refquota",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "relatime",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "sharenfs",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "sync",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "dataset",
        allow_empty_response: true,
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
    pub fn admin_lock_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().admin_lock_type = runtime::InputValue::Value(value.into());
        self
    }
    pub fn admin_override(mut self, value: bool) -> Self {
        self.input_mut().admin_override = runtime::InputValue::Value(value);
        self
    }
    pub fn atime(mut self, value: bool) -> Self {
        self.input_mut().atime = runtime::InputValue::Value(value);
        self
    }
    pub fn compression(mut self, value: bool) -> Self {
        self.input_mut().compression = runtime::InputValue::Value(value);
        self
    }
    pub fn quota(mut self, value: i64) -> Self {
        self.input_mut().quota = runtime::InputValue::Value(value);
        self
    }
    pub fn recordsize(mut self, value: i64) -> Self {
        self.input_mut().recordsize = runtime::InputValue::Value(value);
        self
    }
    pub fn refquota(mut self, value: i64) -> Self {
        self.input_mut().refquota = runtime::InputValue::Value(value);
        self
    }
    pub fn relatime(mut self, value: bool) -> Self {
        self.input_mut().relatime = runtime::InputValue::Value(value);
        self
    }
    pub fn sharenfs(mut self, value: impl Into<String>) -> Self {
        self.input_mut().sharenfs = runtime::InputValue::Value(value.into());
        self
    }
    pub fn sync(mut self, value: impl Into<String>) -> Self {
        self.input_mut().sync = runtime::InputValue::Value(value.into());
        self
    }
}
