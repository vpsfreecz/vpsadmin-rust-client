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
    #[serde(default, rename = "address")]
    pub address: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "dokuwiki_groups")]
    pub dokuwiki_groups: Option<String>,
    #[serde(default, rename = "email")]
    pub email: Option<String>,
    #[serde(default, rename = "enable_basic_auth")]
    pub enable_basic_auth: Option<bool>,
    #[serde(default, rename = "enable_multi_factor_auth")]
    pub enable_multi_factor_auth: Option<bool>,
    #[serde(default, rename = "enable_new_login_notification")]
    pub enable_new_login_notification: Option<bool>,
    #[serde(default, rename = "enable_oauth2_auth")]
    pub enable_oauth2_auth: Option<bool>,
    #[serde(default, rename = "enable_single_sign_on")]
    pub enable_single_sign_on: Option<bool>,
    #[serde(default, rename = "enable_token_auth")]
    pub enable_token_auth: Option<bool>,
    #[serde(default, rename = "expiration_date")]
    pub expiration_date: Option<String>,
    #[serde(default, rename = "full_name")]
    pub full_name: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "info")]
    pub info: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "language")]
    pub language: Option<runtime::Association>,
    #[serde(default, rename = "last_activity_at")]
    pub last_activity_at: Option<String>,
    #[serde(default, rename = "level")]
    pub level: Option<i64>,
    #[serde(default, rename = "lockout")]
    pub lockout: Option<bool>,
    #[serde(default, rename = "login")]
    pub login: Option<String>,
    #[serde(default, rename = "mailer_enabled")]
    pub mailer_enabled: Option<bool>,
    #[serde(default, rename = "monthly_payment")]
    pub monthly_payment: Option<i64>,
    #[serde(default, rename = "object_state")]
    pub object_state: Option<String>,
    #[serde(default, rename = "paid_until")]
    pub paid_until: Option<String>,
    #[serde(default, rename = "password_reset")]
    pub password_reset: Option<bool>,
    #[serde(default, rename = "preferred_logout_all")]
    pub preferred_logout_all: Option<bool>,
    #[serde(default, rename = "preferred_session_length")]
    pub preferred_session_length: Option<i64>,
    #[serde(default, rename = "remind_after_date")]
    pub remind_after_date: Option<String>,
    #[serde(default, rename = "time_zone")]
    pub time_zone: Option<String>,
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
    name: "user/current",
    method: runtime::Method::Get,
    path: "v7.0/users/current",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("user"),
        parameters: &[],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "user",
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
