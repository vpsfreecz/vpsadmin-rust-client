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
        rename = "auth_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub auth_type: runtime::InputValue<String>,
    #[serde(
        rename = "client_ip_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub client_ip_addr: runtime::InputValue<String>,
    #[serde(
        rename = "client_version",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub client_version: runtime::InputValue<String>,
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "ip_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_addr: runtime::InputValue<String>,
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
    #[serde(
        rename = "token_fragment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub token_fragment: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
    #[serde(
        rename = "user_agent",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub user_agent: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.admin)?;
        runtime::Validate::validate(&self.api_ip_addr)?;
        runtime::Validate::validate(&self.auth_type)?;
        runtime::Validate::validate(&self.client_ip_addr)?;
        runtime::Validate::validate(&self.client_version)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.ip_addr)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.state)?;
        runtime::Validate::validate(&self.token_fragment)?;
        runtime::Validate::validate(&self.user)?;
        runtime::Validate::validate(&self.user_agent)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "admin")]
    pub admin: Option<runtime::Association>,
    #[serde(default, rename = "api_ip_addr")]
    pub api_ip_addr: Option<String>,
    #[serde(default, rename = "api_ip_ptr")]
    pub api_ip_ptr: Option<String>,
    #[serde(default, rename = "auth_type")]
    pub auth_type: Option<String>,
    #[serde(default, rename = "client_ip_addr")]
    pub client_ip_addr: Option<String>,
    #[serde(default, rename = "client_ip_ptr")]
    pub client_ip_ptr: Option<String>,
    #[serde(default, rename = "client_version")]
    pub client_version: Option<String>,
    #[serde(default, rename = "closed_at")]
    pub closed_at: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "last_request_at")]
    pub last_request_at: Option<String>,
    #[serde(default, rename = "request_count")]
    pub request_count: Option<i64>,
    #[serde(default, rename = "scope")]
    pub scope: Option<String>,
    #[serde(default, rename = "token_fragment")]
    pub token_fragment: Option<String>,
    #[serde(default, rename = "token_interval")]
    pub token_interval: Option<i64>,
    #[serde(default, rename = "token_lifetime")]
    pub token_lifetime: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[serde(default, rename = "user_agent")]
    pub user_agent: Option<String>,
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
    name: "user_session/index",
    method: runtime::Method::Get,
    path: "v7.0/user_sessions",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("user_session"),
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
                name: "auth_type",
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
                name: "client_version",
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
                name: "ip_addr",
                kind: runtime::ParameterKind::String,
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
                name: "token_fragment",
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
            runtime::ParameterSpec {
                name: "user_agent",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "user_sessions",
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
    pub fn auth_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().auth_type = runtime::InputValue::Value(value.into());
        self
    }
    pub fn client_ip_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().client_ip_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn client_version(mut self, value: impl Into<String>) -> Self {
        self.input_mut().client_version = runtime::InputValue::Value(value.into());
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn ip_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().ip_addr = runtime::InputValue::Value(value.into());
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
    pub fn token_fragment(mut self, value: impl Into<String>) -> Self {
        self.input_mut().token_fragment = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.input_mut().user_agent = runtime::InputValue::Value(value.into());
        self
    }
}
