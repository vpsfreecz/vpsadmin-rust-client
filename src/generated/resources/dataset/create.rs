use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "atime",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub atime: runtime::InputValue<bool>,
    #[serde(
        rename = "automount",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub automount: runtime::InputValue<bool>,
    #[serde(
        rename = "compression",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub compression: runtime::InputValue<bool>,
    #[serde(
        rename = "dataset",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dataset: runtime::InputValue<i64>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
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
        runtime::Validate::validate(&self.atime)?;
        runtime::Validate::validate(&self.automount)?;
        runtime::Validate::validate(&self.compression)?;
        runtime::Validate::validate(&self.dataset)?;
        runtime::Validate::validate(&self.name)?;
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
    name: "dataset/create",
    method: runtime::Method::Post,
    path: "v7.0/datasets",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dataset"),
        parameters: &[
            runtime::ParameterSpec {
                name: "atime",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "automount",
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
                name: "dataset",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "name",
                kind: runtime::ParameterKind::String,
                required: true,
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
    pub fn atime(mut self, value: bool) -> Self {
        self.input_mut().atime = runtime::InputValue::Value(value);
        self
    }
    pub fn automount(mut self, value: bool) -> Self {
        self.input_mut().automount = runtime::InputValue::Value(value);
        self
    }
    pub fn compression(mut self, value: bool) -> Self {
        self.input_mut().compression = runtime::InputValue::Value(value);
        self
    }
    pub fn dataset(mut self, value: i64) -> Self {
        self.input_mut().dataset = runtime::InputValue::Value(value);
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
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
