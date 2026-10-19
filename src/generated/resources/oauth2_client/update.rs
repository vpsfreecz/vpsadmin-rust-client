use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "access_token_lifetime",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub access_token_lifetime: runtime::InputValue<String>,
    #[serde(
        rename = "access_token_seconds",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub access_token_seconds: runtime::InputValue<i64>,
    #[serde(
        rename = "allow_single_sign_on",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub allow_single_sign_on: runtime::InputValue<bool>,
    #[serde(
        rename = "authorization_start_requires_user_action",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub authorization_start_requires_user_action: runtime::InputValue<bool>,
    #[serde(
        rename = "authorization_start_uri",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub authorization_start_uri: runtime::InputValue<String>,
    #[serde(
        rename = "client_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub client_id: runtime::InputValue<String>,
    #[serde(
        rename = "client_secret",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub client_secret: runtime::InputValue<String>,
    #[serde(
        rename = "is_default",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub is_default: runtime::InputValue<bool>,
    #[serde(
        rename = "issue_refresh_token",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub issue_refresh_token: runtime::InputValue<bool>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(
        rename = "redirect_uri",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub redirect_uri: runtime::InputValue<String>,
    #[serde(
        rename = "refresh_token_seconds",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub refresh_token_seconds: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.access_token_lifetime)?;
        runtime::Validate::validate(&self.access_token_seconds)?;
        runtime::Validate::validate(&self.allow_single_sign_on)?;
        runtime::Validate::validate(&self.authorization_start_requires_user_action)?;
        runtime::Validate::validate(&self.authorization_start_uri)?;
        runtime::Validate::validate(&self.client_id)?;
        runtime::Validate::validate(&self.client_secret)?;
        runtime::Validate::validate(&self.is_default)?;
        runtime::Validate::validate(&self.issue_refresh_token)?;
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.redirect_uri)?;
        runtime::Validate::validate(&self.refresh_token_seconds)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "access_token_lifetime")]
    pub access_token_lifetime: Option<String>,
    #[serde(default, rename = "access_token_seconds")]
    pub access_token_seconds: Option<i64>,
    #[serde(default, rename = "allow_single_sign_on")]
    pub allow_single_sign_on: Option<bool>,
    #[serde(default, rename = "authorization_start_requires_user_action")]
    pub authorization_start_requires_user_action: Option<bool>,
    #[serde(default, rename = "authorization_start_uri")]
    pub authorization_start_uri: Option<String>,
    #[serde(default, rename = "client_id")]
    pub client_id: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "is_default")]
    pub is_default: Option<bool>,
    #[serde(default, rename = "issue_refresh_token")]
    pub issue_refresh_token: Option<bool>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "redirect_uri")]
    pub redirect_uri: Option<String>,
    #[serde(default, rename = "refresh_token_seconds")]
    pub refresh_token_seconds: Option<i64>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
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
    name: "oauth2_client/update",
    method: runtime::Method::Put,
    path: "v7.0/oauth2_clients/{oauth2_client_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("oauth2_client"),
        parameters: &[
            runtime::ParameterSpec {
                name: "access_token_lifetime",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "access_token_seconds",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "allow_single_sign_on",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "authorization_start_requires_user_action",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "authorization_start_uri",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "client_id",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "client_secret",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "is_default",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "issue_refresh_token",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "name",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "redirect_uri",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "refresh_token_seconds",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "oauth2_client",
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
    pub fn access_token_lifetime(mut self, value: impl Into<String>) -> Self {
        self.input_mut().access_token_lifetime = runtime::InputValue::Value(value.into());
        self
    }
    pub fn access_token_seconds(mut self, value: i64) -> Self {
        self.input_mut().access_token_seconds = runtime::InputValue::Value(value);
        self
    }
    pub fn allow_single_sign_on(mut self, value: bool) -> Self {
        self.input_mut().allow_single_sign_on = runtime::InputValue::Value(value);
        self
    }
    pub fn authorization_start_requires_user_action(mut self, value: bool) -> Self {
        self.input_mut().authorization_start_requires_user_action =
            runtime::InputValue::Value(value);
        self
    }
    pub fn authorization_start_uri(mut self, value: impl Into<String>) -> Self {
        self.input_mut().authorization_start_uri = runtime::InputValue::Value(value.into());
        self
    }
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.input_mut().client_id = runtime::InputValue::Value(value.into());
        self
    }
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.input_mut().client_secret = runtime::InputValue::Value(value.into());
        self
    }
    pub fn is_default(mut self, value: bool) -> Self {
        self.input_mut().is_default = runtime::InputValue::Value(value);
        self
    }
    pub fn issue_refresh_token(mut self, value: bool) -> Self {
        self.input_mut().issue_refresh_token = runtime::InputValue::Value(value);
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.input_mut().redirect_uri = runtime::InputValue::Value(value.into());
        self
    }
    pub fn refresh_token_seconds(mut self, value: i64) -> Self {
        self.input_mut().refresh_token_seconds = runtime::InputValue::Value(value);
        self
    }
}
