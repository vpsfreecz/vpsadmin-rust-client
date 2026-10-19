use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
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
        rename = "environment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub environment: runtime::InputValue<i64>,
    #[serde(
        rename = "has_ipv6",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub has_ipv6: runtime::InputValue<bool>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(
        rename = "remote_console_server",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub remote_console_server: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.description)?;
        runtime::Validate::validate(&self.domain)?;
        runtime::Validate::validate(&self.environment)?;
        runtime::Validate::validate(&self.has_ipv6)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.remote_console_server)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "description")]
    pub description: Option<String>,
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "environment")]
    pub environment: Option<runtime::Association>,
    #[serde(default, rename = "has_ipv6")]
    pub has_ipv6: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "remote_console_server")]
    pub remote_console_server: Option<String>,
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
    name: "location/update",
    method: runtime::Method::Put,
    path: "v7.0/locations/{location_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("location"),
        parameters: &[
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
                name: "environment",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "has_ipv6",
                kind: runtime::ParameterKind::Boolean,
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
                name: "remote_console_server",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "location",
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
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.input_mut().description = runtime::InputValue::Value(value.into());
        self
    }
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.input_mut().domain = runtime::InputValue::Value(value.into());
        self
    }
    pub fn environment(mut self, value: i64) -> Self {
        self.input_mut().environment = runtime::InputValue::Value(value);
        self
    }
    pub fn has_ipv6(mut self, value: bool) -> Self {
        self.input_mut().has_ipv6 = runtime::InputValue::Value(value);
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn remote_console_server(mut self, value: impl Into<String>) -> Self {
        self.input_mut().remote_console_server = runtime::InputValue::Value(value.into());
        self
    }
}
