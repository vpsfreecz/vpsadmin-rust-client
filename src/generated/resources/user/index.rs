use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "address",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub address: runtime::InputValue<String>,
    #[serde(
        rename = "admin",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub admin: runtime::InputValue<bool>,
    #[serde(
        rename = "email",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub email: runtime::InputValue<String>,
    #[serde(
        rename = "enable_basic_auth",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_basic_auth: runtime::InputValue<bool>,
    #[serde(
        rename = "enable_multi_factor_auth",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_multi_factor_auth: runtime::InputValue<bool>,
    #[serde(
        rename = "enable_new_login_notification",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_new_login_notification: runtime::InputValue<bool>,
    #[serde(
        rename = "enable_oauth2_auth",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_oauth2_auth: runtime::InputValue<bool>,
    #[serde(
        rename = "enable_single_sign_on",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_single_sign_on: runtime::InputValue<bool>,
    #[serde(
        rename = "enable_token_auth",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_token_auth: runtime::InputValue<bool>,
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "full_name",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub full_name: runtime::InputValue<String>,
    #[serde(rename = "info", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub info: runtime::InputValue<String>,
    #[serde(
        rename = "language",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub language: runtime::InputValue<i64>,
    #[serde(
        rename = "level",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub level: runtime::InputValue<i64>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(
        rename = "lockout",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub lockout: runtime::InputValue<bool>,
    #[serde(
        rename = "login",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub login: runtime::InputValue<String>,
    #[serde(
        rename = "mailer_enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub mailer_enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "object_state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub object_state: runtime::InputValue<String>,
    #[serde(
        rename = "password_reset",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub password_reset: runtime::InputValue<bool>,
    #[serde(
        rename = "preferred_logout_all",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub preferred_logout_all: runtime::InputValue<bool>,
    #[serde(
        rename = "preferred_session_length",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub preferred_session_length: runtime::InputValue<i64>,
    #[serde(
        rename = "time_zone",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub time_zone: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.address)?;
        runtime::Validate::validate(&self.admin)?;
        runtime::Validate::validate(&self.email)?;
        runtime::Validate::validate(&self.enable_basic_auth)?;
        runtime::Validate::validate(&self.enable_multi_factor_auth)?;
        runtime::Validate::validate(&self.enable_new_login_notification)?;
        runtime::Validate::validate(&self.enable_oauth2_auth)?;
        runtime::Validate::validate(&self.enable_single_sign_on)?;
        runtime::Validate::validate(&self.enable_token_auth)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.full_name)?;
        runtime::Validate::validate(&self.info)?;
        runtime::Validate::validate(&self.language)?;
        runtime::Validate::validate(&self.level)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.lockout)?;
        runtime::Validate::validate(&self.login)?;
        runtime::Validate::validate(&self.mailer_enabled)?;
        runtime::Validate::validate(&self.object_state)?;
        runtime::Validate::validate(&self.password_reset)?;
        runtime::Validate::validate(&self.preferred_logout_all)?;
        runtime::Validate::validate(&self.preferred_session_length)?;
        runtime::Validate::validate(&self.time_zone)?;
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "user/index",
    method: runtime::Method::Get,
    path: "v7.0/users",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("user"),
        parameters: &[
            runtime::ParameterSpec {
                name: "address",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "admin",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "email",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_basic_auth",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_multi_factor_auth",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_new_login_notification",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_oauth2_auth",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_single_sign_on",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_token_auth",
                kind: runtime::ParameterKind::Boolean,
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
                name: "full_name",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "info",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "language",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "level",
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
                name: "lockout",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "login",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "mailer_enabled",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "object_state",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "password_reset",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "preferred_logout_all",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "preferred_session_length",
                kind: runtime::ParameterKind::Integer,
                required: false,
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
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "users",
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
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.input_mut().address = runtime::InputValue::Value(value.into());
        self
    }
    pub fn admin(mut self, value: bool) -> Self {
        self.input_mut().admin = runtime::InputValue::Value(value);
        self
    }
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.input_mut().email = runtime::InputValue::Value(value.into());
        self
    }
    pub fn enable_basic_auth(mut self, value: bool) -> Self {
        self.input_mut().enable_basic_auth = runtime::InputValue::Value(value);
        self
    }
    pub fn enable_multi_factor_auth(mut self, value: bool) -> Self {
        self.input_mut().enable_multi_factor_auth = runtime::InputValue::Value(value);
        self
    }
    pub fn enable_new_login_notification(mut self, value: bool) -> Self {
        self.input_mut().enable_new_login_notification = runtime::InputValue::Value(value);
        self
    }
    pub fn enable_oauth2_auth(mut self, value: bool) -> Self {
        self.input_mut().enable_oauth2_auth = runtime::InputValue::Value(value);
        self
    }
    pub fn enable_single_sign_on(mut self, value: bool) -> Self {
        self.input_mut().enable_single_sign_on = runtime::InputValue::Value(value);
        self
    }
    pub fn enable_token_auth(mut self, value: bool) -> Self {
        self.input_mut().enable_token_auth = runtime::InputValue::Value(value);
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().full_name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn info(mut self, value: impl Into<String>) -> Self {
        self.input_mut().info = runtime::InputValue::Value(value.into());
        self
    }
    pub fn language(mut self, value: i64) -> Self {
        self.input_mut().language = runtime::InputValue::Value(value);
        self
    }
    pub fn level(mut self, value: i64) -> Self {
        self.input_mut().level = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn lockout(mut self, value: bool) -> Self {
        self.input_mut().lockout = runtime::InputValue::Value(value);
        self
    }
    pub fn login(mut self, value: impl Into<String>) -> Self {
        self.input_mut().login = runtime::InputValue::Value(value.into());
        self
    }
    pub fn mailer_enabled(mut self, value: bool) -> Self {
        self.input_mut().mailer_enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn object_state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().object_state = runtime::InputValue::Value(value.into());
        self
    }
    pub fn password_reset(mut self, value: bool) -> Self {
        self.input_mut().password_reset = runtime::InputValue::Value(value);
        self
    }
    pub fn preferred_logout_all(mut self, value: bool) -> Self {
        self.input_mut().preferred_logout_all = runtime::InputValue::Value(value);
        self
    }
    pub fn preferred_session_length(mut self, value: i64) -> Self {
        self.input_mut().preferred_session_length = runtime::InputValue::Value(value);
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
}
