use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "closes_at",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub closes_at: runtime::InputValue<i64>,
    #[serde(
        rename = "is_open",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub is_open: runtime::InputValue<bool>,
    #[serde(
        rename = "opens_at",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub opens_at: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.closes_at)?;
        runtime::Validate::validate(&self.is_open)?;
        runtime::Validate::validate(&self.opens_at)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "closes_at")]
    pub closes_at: Option<i64>,
    #[serde(default, rename = "is_open")]
    pub is_open: Option<bool>,
    #[serde(default, rename = "opens_at")]
    pub opens_at: Option<i64>,
    #[serde(default, rename = "weekday")]
    pub weekday: Option<i64>,
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
    name: "vps/maintenance_window/update_all",
    method: runtime::Method::Put,
    path: "v7.0/vpses/{vps_id}/maintenance_windows",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("maintenance_window"),
        parameters: &[
            runtime::ParameterSpec {
                name: "closes_at",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "is_open",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "opens_at",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "maintenance_windows",
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
impl<'a, C: runtime::Mode> runtime::Request<'a, C, Input, Vec<Output>, Meta> {
    pub fn closes_at(mut self, value: i64) -> Self {
        self.input_mut().closes_at = runtime::InputValue::Value(value);
        self
    }
    pub fn is_open(mut self, value: bool) -> Self {
        self.input_mut().is_open = runtime::InputValue::Value(value);
        self
    }
    pub fn opens_at(mut self, value: i64) -> Self {
        self.input_mut().opens_at = runtime::InputValue::Value(value);
        self
    }
}
