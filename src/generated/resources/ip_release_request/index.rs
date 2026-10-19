use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "ip_release_campaign",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_release_campaign: runtime::InputValue<i64>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.ip_release_campaign)?;
        runtime::Validate::validate(&self.limit)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "allow_keep")]
    pub allow_keep: Option<bool>,
    #[serde(default, rename = "can_assign")]
    pub can_assign: Option<bool>,
    #[serde(default, rename = "can_keep")]
    pub can_keep: Option<bool>,
    #[serde(default, rename = "closed_at")]
    pub closed_at: Option<String>,
    #[serde(default, rename = "deadline")]
    pub deadline: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "ip_release_campaign")]
    pub ip_release_campaign: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=subject"]
    #[serde(default, rename = "mail_log")]
    pub mail_log: Option<runtime::Association>,
    #[serde(default, rename = "notified_at")]
    pub notified_at: Option<String>,
    #[serde(default, rename = "original_user_id")]
    pub original_user_id: Option<i64>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[serde(default, rename = "user_login")]
    pub user_login: Option<String>,
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
    name: "ip_release_request/index",
    method: runtime::Method::Get,
    path: "v7.0/ip_release_requests",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("ip_release_request"),
        parameters: &[
            runtime::ParameterSpec {
                name: "from_id",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ip_release_campaign",
                kind: runtime::ParameterKind::Resource,
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
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "ip_release_requests",
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
    pub fn ip_release_campaign(mut self, value: i64) -> Self {
        self.input_mut().ip_release_campaign = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
}
