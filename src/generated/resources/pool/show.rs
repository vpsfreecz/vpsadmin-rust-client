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
    #[serde(default, rename = "maintenance_lock")]
    pub maintenance_lock: Option<String>,
    #[serde(default, rename = "maintenance_lock_reason")]
    pub maintenance_lock_reason: Option<String>,
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
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "pool/show",
    method: runtime::Method::Get,
    path: "v7.0/pools/{pool_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("pool"),
        parameters: &[],
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {}
