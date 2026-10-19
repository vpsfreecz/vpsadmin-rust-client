use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "addresses",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub addresses: runtime::InputValue<serde_json::Value>,
    #[serde(
        rename = "allow_keep",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub allow_keep: runtime::InputValue<bool>,
    #[serde(
        rename = "deadline",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub deadline: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.addresses)?;
        runtime::Validate::validate(&self.allow_keep)?;
        runtime::Validate::validate(&self.deadline)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "allow_keep")]
    pub allow_keep: Option<bool>,
    #[serde(default, rename = "closed_at")]
    pub closed_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "closed_by")]
    pub closed_by: Option<runtime::Association>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "created_by")]
    pub created_by: Option<runtime::Association>,
    #[serde(default, rename = "deadline")]
    pub deadline: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
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
    name: "ip_release_campaign/create",
    method: runtime::Method::Post,
    path: "v7.0/ip_release_campaigns",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("ip_release_campaign"),
        parameters: &[
            runtime::ParameterSpec {
                name: "addresses",
                kind: runtime::ParameterKind::Custom,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "allow_keep",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "deadline",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "ip_release_campaign",
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
    pub fn addresses(mut self, value: serde_json::Value) -> Self {
        self.input_mut().addresses = runtime::InputValue::Value(value);
        self
    }
    pub fn allow_keep(mut self, value: bool) -> Self {
        self.input_mut().allow_keep = runtime::InputValue::Value(value);
        self
    }
    pub fn deadline(mut self, value: impl Into<String>) -> Self {
        self.input_mut().deadline = runtime::InputValue::Value(value.into());
        self
    }
}
