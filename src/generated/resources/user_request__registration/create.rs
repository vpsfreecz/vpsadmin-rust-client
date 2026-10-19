use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "address",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub address: runtime::InputValue<String>,
    #[serde(
        rename = "currency",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub currency: runtime::InputValue<String>,
    #[serde(
        rename = "email",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub email: runtime::InputValue<String>,
    #[serde(
        rename = "full_name",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub full_name: runtime::InputValue<String>,
    #[serde(rename = "how", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub how: runtime::InputValue<String>,
    #[serde(
        rename = "language",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub language: runtime::InputValue<i64>,
    #[serde(
        rename = "location",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub location: runtime::InputValue<i64>,
    #[serde(
        rename = "login",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub login: runtime::InputValue<String>,
    #[serde(rename = "note", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub note: runtime::InputValue<String>,
    #[serde(
        rename = "org_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub org_id: runtime::InputValue<String>,
    #[serde(
        rename = "org_name",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub org_name: runtime::InputValue<String>,
    #[serde(
        rename = "os_template",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub os_template: runtime::InputValue<i64>,
    #[serde(
        rename = "time_zone",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub time_zone: runtime::InputValue<String>,
    #[serde(
        rename = "year_of_birth",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub year_of_birth: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.address)?;
        runtime::Validate::validate(&self.currency)?;
        runtime::Validate::validate(&self.email)?;
        runtime::Validate::validate(&self.full_name)?;
        runtime::Validate::validate(&self.how)?;
        runtime::Validate::validate(&self.language)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.login)?;
        runtime::Validate::validate(&self.note)?;
        runtime::Validate::validate(&self.org_id)?;
        runtime::Validate::validate(&self.org_name)?;
        runtime::Validate::validate(&self.os_template)?;
        runtime::Validate::validate(&self.time_zone)?;
        runtime::Validate::validate(&self.year_of_birth)?;
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
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "user_request/registration/create",
    method: runtime::Method::Post,
    path: "v7.0/user_request/registrations",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("registration"),
        parameters: &[
            runtime::ParameterSpec {
                name: "address",
                kind: runtime::ParameterKind::Text,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "currency",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "email",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "full_name",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "how",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "language",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "location",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "login",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "note",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "org_id",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "org_name",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "os_template",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "time_zone",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "year_of_birth",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "registration",
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
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.input_mut().address = runtime::InputValue::Value(value.into());
        self
    }
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.input_mut().currency = runtime::InputValue::Value(value.into());
        self
    }
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.input_mut().email = runtime::InputValue::Value(value.into());
        self
    }
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().full_name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn how(mut self, value: impl Into<String>) -> Self {
        self.input_mut().how = runtime::InputValue::Value(value.into());
        self
    }
    pub fn language(mut self, value: i64) -> Self {
        self.input_mut().language = runtime::InputValue::Value(value);
        self
    }
    pub fn location(mut self, value: i64) -> Self {
        self.input_mut().location = runtime::InputValue::Value(value);
        self
    }
    pub fn login(mut self, value: impl Into<String>) -> Self {
        self.input_mut().login = runtime::InputValue::Value(value.into());
        self
    }
    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.input_mut().note = runtime::InputValue::Value(value.into());
        self
    }
    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.input_mut().org_id = runtime::InputValue::Value(value.into());
        self
    }
    pub fn org_name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().org_name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn os_template(mut self, value: i64) -> Self {
        self.input_mut().os_template = runtime::InputValue::Value(value);
        self
    }
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.input_mut().time_zone = runtime::InputValue::Value(value.into());
        self
    }
    pub fn time_zone_null(mut self) -> Self {
        self.input_mut().time_zone = runtime::InputValue::Null;
        self
    }
    pub fn year_of_birth(mut self, value: i64) -> Self {
        self.input_mut().year_of_birth = runtime::InputValue::Value(value);
        self
    }
}
