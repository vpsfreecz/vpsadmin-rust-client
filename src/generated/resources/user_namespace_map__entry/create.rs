use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "count",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub count: runtime::InputValue<i64>,
    #[serde(rename = "kind", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub kind: runtime::InputValue<String>,
    #[serde(
        rename = "ns_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ns_id: runtime::InputValue<i64>,
    #[serde(
        rename = "vps_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub vps_id: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.count)?;
        runtime::Validate::validate(&self.kind)?;
        runtime::Validate::validate(&self.ns_id)?;
        runtime::Validate::validate(&self.vps_id)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "count")]
    pub count: Option<i64>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "kind")]
    pub kind: Option<String>,
    #[serde(default, rename = "ns_id")]
    pub ns_id: Option<i64>,
    #[serde(default, rename = "vps_id")]
    pub vps_id: Option<i64>,
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
    name: "user_namespace_map/entry/create",
    method: runtime::Method::Post,
    path: "v7.0/user_namespace_maps/{user_namespace_map_id}/entries",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("entry"),
        parameters: &[
            runtime::ParameterSpec {
                name: "count",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "kind",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ns_id",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps_id",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "entry",
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
    pub fn count(mut self, value: i64) -> Self {
        self.input_mut().count = runtime::InputValue::Value(value);
        self
    }
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.input_mut().kind = runtime::InputValue::Value(value.into());
        self
    }
    pub fn ns_id(mut self, value: i64) -> Self {
        self.input_mut().ns_id = runtime::InputValue::Value(value);
        self
    }
    pub fn vps_id(mut self, value: i64) -> Self {
        self.input_mut().vps_id = runtime::InputValue::Value(value);
        self
    }
}
