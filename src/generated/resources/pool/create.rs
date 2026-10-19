use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "atime",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub atime: runtime::InputValue<bool>,
    #[serde(
        rename = "available_space",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub available_space: runtime::InputValue<i64>,
    #[serde(
        rename = "checked_at",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub checked_at: runtime::InputValue<String>,
    #[serde(
        rename = "compression",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub compression: runtime::InputValue<bool>,
    #[serde(
        rename = "filesystem",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub filesystem: runtime::InputValue<String>,
    #[serde(
        rename = "is_open",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub is_open: runtime::InputValue<bool>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(
        rename = "max_datasets",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub max_datasets: runtime::InputValue<i64>,
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
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
        rename = "refquota_check",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub refquota_check: runtime::InputValue<bool>,
    #[serde(
        rename = "relatime",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub relatime: runtime::InputValue<bool>,
    #[serde(rename = "role", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub role: runtime::InputValue<String>,
    #[serde(rename = "scan", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub scan: runtime::InputValue<String>,
    #[serde(
        rename = "scan_percent",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub scan_percent: runtime::InputValue<f64>,
    #[serde(
        rename = "sharenfs",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub sharenfs: runtime::InputValue<String>,
    #[serde(
        rename = "state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub state: runtime::InputValue<String>,
    #[serde(rename = "sync", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub sync: runtime::InputValue<String>,
    #[serde(
        rename = "total_space",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub total_space: runtime::InputValue<i64>,
    #[serde(
        rename = "used_space",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub used_space: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.atime)?;
        runtime::Validate::validate(&self.available_space)?;
        runtime::Validate::validate(&self.checked_at)?;
        runtime::Validate::validate(&self.compression)?;
        runtime::Validate::validate(&self.filesystem)?;
        runtime::Validate::validate(&self.is_open)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.max_datasets)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.quota)?;
        runtime::Validate::validate(&self.recordsize)?;
        runtime::Validate::validate(&self.refquota)?;
        runtime::Validate::validate(&self.refquota_check)?;
        runtime::Validate::validate(&self.relatime)?;
        runtime::Validate::validate(&self.role)?;
        runtime::Validate::validate(&self.scan)?;
        runtime::Validate::validate(&self.scan_percent)?;
        runtime::Validate::validate(&self.sharenfs)?;
        runtime::Validate::validate(&self.state)?;
        runtime::Validate::validate(&self.sync)?;
        runtime::Validate::validate(&self.total_space)?;
        runtime::Validate::validate(&self.used_space)?;
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
    #[serde(default, rename = "available_space")]
    pub available_space: Option<i64>,
    #[serde(default, rename = "checked_at")]
    pub checked_at: Option<String>,
    #[serde(default, rename = "compression")]
    pub compression: Option<bool>,
    #[serde(default, rename = "compressratio")]
    pub compressratio: Option<f64>,
    #[serde(default, rename = "filesystem")]
    pub filesystem: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "is_open")]
    pub is_open: Option<bool>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "max_datasets")]
    pub max_datasets: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
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
    #[serde(default, rename = "refquota_check")]
    pub refquota_check: Option<bool>,
    #[serde(default, rename = "relatime")]
    pub relatime: Option<bool>,
    #[serde(default, rename = "role")]
    pub role: Option<String>,
    #[serde(default, rename = "scan")]
    pub scan: Option<String>,
    #[serde(default, rename = "scan_percent")]
    pub scan_percent: Option<f64>,
    #[serde(default, rename = "sharenfs")]
    pub sharenfs: Option<String>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "sync")]
    pub sync: Option<String>,
    #[serde(default, rename = "total_space")]
    pub total_space: Option<i64>,
    #[serde(default, rename = "used")]
    pub used: Option<i64>,
    #[serde(default, rename = "used_space")]
    pub used_space: Option<i64>,
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
    name: "pool/create",
    method: runtime::Method::Post,
    path: "v7.0/pools",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("pool"),
        parameters: &[
            runtime::ParameterSpec {
                name: "atime",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "available_space",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "checked_at",
                kind: runtime::ParameterKind::Datetime,
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
                name: "filesystem",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "is_open",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "label",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "max_datasets",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "node",
                kind: runtime::ParameterKind::Resource,
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
                name: "refquota_check",
                kind: runtime::ParameterKind::Boolean,
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
                name: "role",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "scan",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "scan_percent",
                kind: runtime::ParameterKind::Float,
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
                name: "state",
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
            runtime::ParameterSpec {
                name: "total_space",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "used_space",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "pool",
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
    pub fn available_space(mut self, value: i64) -> Self {
        self.input_mut().available_space = runtime::InputValue::Value(value);
        self
    }
    pub fn checked_at(mut self, value: impl Into<String>) -> Self {
        self.input_mut().checked_at = runtime::InputValue::Value(value.into());
        self
    }
    pub fn compression(mut self, value: bool) -> Self {
        self.input_mut().compression = runtime::InputValue::Value(value);
        self
    }
    pub fn filesystem(mut self, value: impl Into<String>) -> Self {
        self.input_mut().filesystem = runtime::InputValue::Value(value.into());
        self
    }
    pub fn is_open(mut self, value: bool) -> Self {
        self.input_mut().is_open = runtime::InputValue::Value(value);
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn max_datasets(mut self, value: i64) -> Self {
        self.input_mut().max_datasets = runtime::InputValue::Value(value);
        self
    }
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
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
    pub fn refquota_check(mut self, value: bool) -> Self {
        self.input_mut().refquota_check = runtime::InputValue::Value(value);
        self
    }
    pub fn relatime(mut self, value: bool) -> Self {
        self.input_mut().relatime = runtime::InputValue::Value(value);
        self
    }
    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.input_mut().role = runtime::InputValue::Value(value.into());
        self
    }
    pub fn scan(mut self, value: impl Into<String>) -> Self {
        self.input_mut().scan = runtime::InputValue::Value(value.into());
        self
    }
    pub fn scan_percent(mut self, value: f64) -> Self {
        self.input_mut().scan_percent = runtime::InputValue::Value(value);
        self
    }
    pub fn sharenfs(mut self, value: impl Into<String>) -> Self {
        self.input_mut().sharenfs = runtime::InputValue::Value(value.into());
        self
    }
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().state = runtime::InputValue::Value(value.into());
        self
    }
    pub fn sync(mut self, value: impl Into<String>) -> Self {
        self.input_mut().sync = runtime::InputValue::Value(value.into());
        self
    }
    pub fn total_space(mut self, value: i64) -> Self {
        self.input_mut().total_space = runtime::InputValue::Value(value);
        self
    }
    pub fn used_space(mut self, value: i64) -> Self {
        self.input_mut().used_space = runtime::InputValue::Value(value);
        self
    }
}
