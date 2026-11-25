use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "days", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub days: runtime::InputValue<i64>,
    #[serde(rename = "vps", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub vps: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.days)?;
        runtime::Validate::validate(&self.vps)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "from_at")]
    pub from_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<String>,
    #[serde(default, rename = "partial_day")]
    pub partial_day: Option<bool>,
    #[serde(default, rename = "recorded_us")]
    pub recorded_us: Option<i64>,
    #[serde(default, rename = "to_at")]
    pub to_at: Option<String>,
    #[serde(default, rename = "_meta")]
    pub object_meta: Option<runtime::JsonMap>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Meta {
    #[serde(default, rename = "accounting_since")]
    pub accounting_since: Option<String>,
    #[serde(default, rename = "available")]
    pub available: Option<bool>,
    #[serde(default, rename = "first_recorded_at")]
    pub first_recorded_at: Option<String>,
    #[serde(default, rename = "ownership_limited")]
    pub ownership_limited: Option<bool>,
    #[serde(default, rename = "range_from")]
    pub range_from: Option<String>,
    #[serde(default, rename = "range_to")]
    pub range_to: Option<String>,
    #[serde(default, rename = "read_at")]
    pub read_at: Option<String>,
    #[serde(default, rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(default, rename = "total_recorded_us")]
    pub total_recorded_us: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "vps_pressure_history/index",
    method: runtime::Method::Get,
    path: "v7.0/vps_pressure_history",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("vps_pressure_history"),
        parameters: &[
            runtime::ParameterSpec {
                name: "days",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "vps_pressure_history",
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Vec<Output>, Meta> {
    pub fn days(mut self, value: i64) -> Self {
        self.input_mut().days = runtime::InputValue::Value(value);
        self
    }
    pub fn vps(mut self, value: i64) -> Self {
        self.input_mut().vps = runtime::InputValue::Value(value);
        self
    }
}
