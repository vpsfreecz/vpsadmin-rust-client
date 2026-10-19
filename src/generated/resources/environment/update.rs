use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "can_create_vps",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub can_create_vps: runtime::InputValue<bool>,
    #[serde(
        rename = "can_destroy_vps",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub can_destroy_vps: runtime::InputValue<bool>,
    #[serde(
        rename = "description",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub description: runtime::InputValue<String>,
    #[serde(
        rename = "domain",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub domain: runtime::InputValue<String>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(
        rename = "max_vps_count",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub max_vps_count: runtime::InputValue<i64>,
    #[serde(
        rename = "user_ip_ownership",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub user_ip_ownership: runtime::InputValue<bool>,
    #[serde(
        rename = "vps_lifetime",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub vps_lifetime: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.can_create_vps)?;
        runtime::Validate::validate(&self.can_destroy_vps)?;
        runtime::Validate::validate(&self.description)?;
        runtime::Validate::validate(&self.domain)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.max_vps_count)?;
        runtime::Validate::validate(&self.user_ip_ownership)?;
        runtime::Validate::validate(&self.vps_lifetime)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "can_create_vps")]
    pub can_create_vps: Option<bool>,
    #[serde(default, rename = "can_destroy_vps")]
    pub can_destroy_vps: Option<bool>,
    #[serde(default, rename = "description")]
    pub description: Option<String>,
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "max_vps_count")]
    pub max_vps_count: Option<i64>,
    #[serde(default, rename = "user_ip_ownership")]
    pub user_ip_ownership: Option<bool>,
    #[serde(default, rename = "vps_lifetime")]
    pub vps_lifetime: Option<i64>,
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
    name: "environment/update",
    method: runtime::Method::Put,
    path: "v7.0/environments/{environment_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("environment"),
        parameters: &[
            runtime::ParameterSpec {
                name: "can_create_vps",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "can_destroy_vps",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "description",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "domain",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "label",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "max_vps_count",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user_ip_ownership",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps_lifetime",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "environment",
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
    pub fn can_create_vps(mut self, value: bool) -> Self {
        self.input_mut().can_create_vps = runtime::InputValue::Value(value);
        self
    }
    pub fn can_destroy_vps(mut self, value: bool) -> Self {
        self.input_mut().can_destroy_vps = runtime::InputValue::Value(value);
        self
    }
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.input_mut().description = runtime::InputValue::Value(value.into());
        self
    }
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.input_mut().domain = runtime::InputValue::Value(value.into());
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn max_vps_count(mut self, value: i64) -> Self {
        self.input_mut().max_vps_count = runtime::InputValue::Value(value);
        self
    }
    pub fn user_ip_ownership(mut self, value: bool) -> Self {
        self.input_mut().user_ip_ownership = runtime::InputValue::Value(value);
        self
    }
    pub fn vps_lifetime(mut self, value: i64) -> Self {
        self.input_mut().vps_lifetime = runtime::InputValue::Value(value);
        self
    }
}
