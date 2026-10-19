use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "admin",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub admin: runtime::InputValue<i64>,
    #[serde(
        rename = "api_ip_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub api_ip_addr: runtime::InputValue<String>,
    #[serde(
        rename = "client_ip_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub client_ip_addr: runtime::InputValue<String>,
    #[serde(
        rename = "client_ip_ptr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub client_ip_ptr: runtime::InputValue<String>,
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
    #[serde(
        rename = "state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub state: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.admin)?;
        runtime::Validate::validate(&self.api_ip_addr)?;
        runtime::Validate::validate(&self.client_ip_addr)?;
        runtime::Validate::validate(&self.client_ip_ptr)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.state)?;
        runtime::Validate::validate(&self.user)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "address")]
    pub address: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "admin")]
    pub admin: Option<runtime::Association>,
    #[serde(default, rename = "admin_response")]
    pub admin_response: Option<String>,
    #[serde(default, rename = "api_ip_addr")]
    pub api_ip_addr: Option<String>,
    #[serde(default, rename = "api_ip_ptr")]
    pub api_ip_ptr: Option<String>,
    #[serde(default, rename = "client_ip_addr")]
    pub client_ip_addr: Option<String>,
    #[serde(default, rename = "client_ip_ptr")]
    pub client_ip_ptr: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "currency")]
    pub currency: Option<String>,
    #[serde(default, rename = "email")]
    pub email: Option<String>,
    #[serde(default, rename = "full_name")]
    pub full_name: Option<String>,
    #[serde(default, rename = "how")]
    pub how: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "ip_checked")]
    pub ip_checked: Option<bool>,
    #[serde(default, rename = "ip_crawler")]
    pub ip_crawler: Option<bool>,
    #[serde(default, rename = "ip_errors")]
    pub ip_errors: Option<String>,
    #[serde(default, rename = "ip_fraud_score")]
    pub ip_fraud_score: Option<i64>,
    #[serde(default, rename = "ip_message")]
    pub ip_message: Option<String>,
    #[serde(default, rename = "ip_proxy")]
    pub ip_proxy: Option<bool>,
    #[serde(default, rename = "ip_recent_abuse")]
    pub ip_recent_abuse: Option<bool>,
    #[serde(default, rename = "ip_request_id")]
    pub ip_request_id: Option<String>,
    #[serde(default, rename = "ip_success")]
    pub ip_success: Option<bool>,
    #[serde(default, rename = "ip_tor")]
    pub ip_tor: Option<bool>,
    #[serde(default, rename = "ip_vpn")]
    pub ip_vpn: Option<bool>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "language")]
    pub language: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "location")]
    pub location: Option<runtime::Association>,
    #[serde(default, rename = "login")]
    pub login: Option<String>,
    #[serde(default, rename = "mail_catch_all")]
    pub mail_catch_all: Option<bool>,
    #[serde(default, rename = "mail_checked")]
    pub mail_checked: Option<bool>,
    #[serde(default, rename = "mail_deliverability")]
    pub mail_deliverability: Option<String>,
    #[serde(default, rename = "mail_disposable")]
    pub mail_disposable: Option<bool>,
    #[serde(default, rename = "mail_dns_valid")]
    pub mail_dns_valid: Option<bool>,
    #[serde(default, rename = "mail_errors")]
    pub mail_errors: Option<String>,
    #[serde(default, rename = "mail_fraud_score")]
    pub mail_fraud_score: Option<i64>,
    #[serde(default, rename = "mail_frequent_complainer")]
    pub mail_frequent_complainer: Option<bool>,
    #[serde(default, rename = "mail_honeypot")]
    pub mail_honeypot: Option<bool>,
    #[serde(default, rename = "mail_leaked")]
    pub mail_leaked: Option<bool>,
    #[serde(default, rename = "mail_message")]
    pub mail_message: Option<String>,
    #[serde(default, rename = "mail_overall_score")]
    pub mail_overall_score: Option<i64>,
    #[serde(default, rename = "mail_recent_abuse")]
    pub mail_recent_abuse: Option<bool>,
    #[serde(default, rename = "mail_request_id")]
    pub mail_request_id: Option<String>,
    #[serde(default, rename = "mail_smtp_score")]
    pub mail_smtp_score: Option<i64>,
    #[serde(default, rename = "mail_spam_trap_score")]
    pub mail_spam_trap_score: Option<String>,
    #[serde(default, rename = "mail_success")]
    pub mail_success: Option<bool>,
    #[serde(default, rename = "mail_suspect")]
    pub mail_suspect: Option<bool>,
    #[serde(default, rename = "mail_timed_out")]
    pub mail_timed_out: Option<bool>,
    #[serde(default, rename = "mail_valid")]
    pub mail_valid: Option<bool>,
    #[serde(default, rename = "note")]
    pub note: Option<String>,
    #[serde(default, rename = "org_id")]
    pub org_id: Option<String>,
    #[serde(default, rename = "org_name")]
    pub org_name: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "os_template")]
    pub os_template: Option<runtime::Association>,
    #[serde(default, rename = "raw_user_id")]
    pub raw_user_id: Option<i64>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "time_zone")]
    pub time_zone: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[serde(default, rename = "year_of_birth")]
    pub year_of_birth: Option<i64>,
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
    name: "user_request/registration/index",
    method: runtime::Method::Get,
    path: "v7.0/user_request/registrations",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("registration"),
        parameters: &[
            runtime::ParameterSpec {
                name: "admin",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "api_ip_addr",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "client_ip_addr",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "client_ip_ptr",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
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
                name: "state",
                kind: runtime::ParameterKind::String,
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
        namespace: "registrations",
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
    pub fn admin(mut self, value: i64) -> Self {
        self.input_mut().admin = runtime::InputValue::Value(value);
        self
    }
    pub fn api_ip_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().api_ip_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn client_ip_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().client_ip_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn client_ip_ptr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().client_ip_ptr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().state = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
}
