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
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "assigned_by_chain")]
    pub assigned_by_chain: Option<runtime::Association>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "from_date")]
    pub from_date: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "ip_addr")]
    pub ip_addr: Option<String>,
    #[doc = "Association keys: ID=id, label=addr"]
    #[serde(default, rename = "ip_address")]
    pub ip_address: Option<runtime::Association>,
    #[serde(default, rename = "ip_prefix")]
    pub ip_prefix: Option<i64>,
    #[serde(default, rename = "raw_user_id")]
    pub raw_user_id: Option<i64>,
    #[serde(default, rename = "raw_vps_id")]
    pub raw_vps_id: Option<i64>,
    #[serde(default, rename = "reconstructed")]
    pub reconstructed: Option<bool>,
    #[serde(default, rename = "to_date")]
    pub to_date: Option<String>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "unassigned_by_chain")]
    pub unassigned_by_chain: Option<runtime::Association>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
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
    name: "ip_address_assignment/show",
    method: runtime::Method::Get,
    path: "v7.0/ip_address_assignments/{ip_address_assignment_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("ip_address_assignment"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "ip_address_assignment",
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
