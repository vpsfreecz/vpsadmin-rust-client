use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "outage",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub outage: runtime::InputValue<i64>,
    #[serde(
        rename = "security_advisory",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub security_advisory: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.outage)?;
        runtime::Validate::validate(&self.security_advisory)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=begins_at"]
    #[serde(default, rename = "outage")]
    pub outage: Option<runtime::Association>,
    #[serde(default, rename = "outage_id")]
    pub outage_id: Option<i64>,
    #[doc = "Association keys: ID=id, label=id"]
    #[serde(default, rename = "security_advisory")]
    pub security_advisory: Option<runtime::Association>,
    #[serde(default, rename = "security_advisory_id")]
    pub security_advisory_id: Option<i64>,
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
    name: "outage_security_advisory/create",
    method: runtime::Method::Post,
    path: "v7.0/outage_security_advisories",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("outage_security_advisory"),
        parameters: &[
            runtime::ParameterSpec {
                name: "outage",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "security_advisory",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "outage_security_advisory",
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
    pub fn outage(mut self, value: i64) -> Self {
        self.input_mut().outage = runtime::InputValue::Value(value);
        self
    }
    pub fn security_advisory(mut self, value: i64) -> Self {
        self.input_mut().security_advisory = runtime::InputValue::Value(value);
        self
    }
}
