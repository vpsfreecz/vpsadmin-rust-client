use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "address",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub address: runtime::InputValue<String>,
    #[serde(
        rename = "change_reason",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub change_reason: runtime::InputValue<String>,
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
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.address)?;
        runtime::Validate::validate(&self.change_reason)?;
        runtime::Validate::validate(&self.email)?;
        runtime::Validate::validate(&self.full_name)?;
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
    #[serde(default, rename = "change_reason")]
    pub change_reason: Option<String>,
    #[serde(default, rename = "client_ip_addr")]
    pub client_ip_addr: Option<String>,
    #[serde(default, rename = "client_ip_ptr")]
    pub client_ip_ptr: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "email")]
    pub email: Option<String>,
    #[serde(default, rename = "full_name")]
    pub full_name: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "raw_user_id")]
    pub raw_user_id: Option<i64>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
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
    name: "user_request/change/create",
    method: runtime::Method::Post,
    path: "v7.0/user_request/changes",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("change"),
        parameters: &[
            runtime::ParameterSpec {
                name: "address",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "change_reason",
                kind: runtime::ParameterKind::String,
                required: true,
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
                name: "full_name",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "change",
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
    pub fn change_reason(mut self, value: impl Into<String>) -> Self {
        self.input_mut().change_reason = runtime::InputValue::Value(value.into());
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
}
