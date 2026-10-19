use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "from", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub from: runtime::InputValue<String>,
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(rename = "to", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub to: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.from)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.to)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "value")]
    pub value: Option<i64>,
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
    name: "dataset/property_history/index",
    method: runtime::Method::Get,
    path: "v7.0/datasets/{dataset_id}/property_history",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("property_history"),
        parameters: &[
            runtime::ParameterSpec {
                name: "from",
                kind: runtime::ParameterKind::Datetime,
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
                name: "limit",
                kind: runtime::ParameterKind::Integer,
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
                name: "to",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "property_histories",
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
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.input_mut().from = runtime::InputValue::Value(value.into());
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.input_mut().to = runtime::InputValue::Value(value.into());
        self
    }
}
