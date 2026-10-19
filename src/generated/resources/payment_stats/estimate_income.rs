use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "duration",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub duration: runtime::InputValue<i64>,
    #[serde(
        rename = "month",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub month: runtime::InputValue<i64>,
    #[serde(
        rename = "select",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub select: runtime::InputValue<String>,
    #[serde(rename = "year", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub year: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.duration)?;
        runtime::Validate::validate(&self.month)?;
        runtime::Validate::validate(&self.select)?;
        runtime::Validate::validate(&self.year)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "estimated_income")]
    pub estimated_income: Option<i64>,
    #[serde(default, rename = "user_count")]
    pub user_count: Option<i64>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "payment_stats/estimate_income",
    method: runtime::Method::Get,
    path: "v7.0/payment_stat/estimate_income",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Hash,
        namespace: Some("payment_stat"),
        parameters: &[
            runtime::ParameterSpec {
                name: "duration",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "month",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "select",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "year",
                kind: runtime::ParameterKind::Integer,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Hash,
        namespace: "payment_stat",
        allow_empty_response: false,
    }),
    meta_namespace: Some("_meta"),
    global_meta: &[runtime::ParameterSpec {
        name: "no",
        kind: runtime::ParameterKind::Boolean,
        required: false,
        nullable: false,
        protected: false,
    }],
    object_meta: &[],
    auth: runtime::RequestAuth::Configured,
};
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Output, Meta> {
    pub fn duration(mut self, value: i64) -> Self {
        self.input_mut().duration = runtime::InputValue::Value(value);
        self
    }
    pub fn month(mut self, value: i64) -> Self {
        self.input_mut().month = runtime::InputValue::Value(value);
        self
    }
    pub fn select(mut self, value: impl Into<String>) -> Self {
        self.input_mut().select = runtime::InputValue::Value(value.into());
        self
    }
    pub fn year(mut self, value: i64) -> Self {
        self.input_mut().year = runtime::InputValue::Value(value);
        self
    }
}
