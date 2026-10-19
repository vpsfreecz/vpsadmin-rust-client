use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "auto_resolve",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub auto_resolve: runtime::InputValue<bool>,
    #[serde(
        rename = "begins_at",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub begins_at: runtime::InputValue<String>,
    #[serde(
        rename = "cs_description",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cs_description: runtime::InputValue<String>,
    #[serde(
        rename = "cs_summary",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cs_summary: runtime::InputValue<String>,
    #[serde(
        rename = "duration",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub duration: runtime::InputValue<i64>,
    #[serde(
        rename = "en_description",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub en_description: runtime::InputValue<String>,
    #[serde(
        rename = "en_summary",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub en_summary: runtime::InputValue<String>,
    #[serde(
        rename = "finished_at",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub finished_at: runtime::InputValue<String>,
    #[serde(
        rename = "impact",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub impact: runtime::InputValue<String>,
    #[serde(rename = "type", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub type_: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.auto_resolve)?;
        runtime::Validate::validate(&self.begins_at)?;
        runtime::Validate::validate(&self.cs_description)?;
        runtime::Validate::validate(&self.cs_summary)?;
        runtime::Validate::validate(&self.duration)?;
        runtime::Validate::validate(&self.en_description)?;
        runtime::Validate::validate(&self.en_summary)?;
        runtime::Validate::validate(&self.finished_at)?;
        runtime::Validate::validate(&self.impact)?;
        runtime::Validate::validate(&self.type_)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "affected")]
    pub affected: Option<bool>,
    #[serde(default, rename = "affected_direct_vps_count")]
    pub affected_direct_vps_count: Option<i64>,
    #[serde(default, rename = "affected_export_count")]
    pub affected_export_count: Option<i64>,
    #[serde(default, rename = "affected_indirect_vps_count")]
    pub affected_indirect_vps_count: Option<i64>,
    #[serde(default, rename = "affected_user_count")]
    pub affected_user_count: Option<i64>,
    #[serde(default, rename = "auto_resolve")]
    pub auto_resolve: Option<bool>,
    #[serde(default, rename = "begins_at")]
    pub begins_at: Option<String>,
    #[serde(default, rename = "cs_description")]
    pub cs_description: Option<String>,
    #[serde(default, rename = "cs_summary")]
    pub cs_summary: Option<String>,
    #[serde(default, rename = "duration")]
    pub duration: Option<i64>,
    #[serde(default, rename = "en_description")]
    pub en_description: Option<String>,
    #[serde(default, rename = "en_summary")]
    pub en_summary: Option<String>,
    #[serde(default, rename = "finished_at")]
    pub finished_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "impact")]
    pub impact: Option<String>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
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
    name: "outage/update",
    method: runtime::Method::Put,
    path: "v7.0/outages/{outage_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("outage"),
        parameters: &[
            runtime::ParameterSpec {
                name: "auto_resolve",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "begins_at",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "cs_description",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "cs_summary",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "duration",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "en_description",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "en_summary",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "finished_at",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "impact",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "type",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "outage",
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
    pub fn auto_resolve(mut self, value: bool) -> Self {
        self.input_mut().auto_resolve = runtime::InputValue::Value(value);
        self
    }
    pub fn begins_at(mut self, value: impl Into<String>) -> Self {
        self.input_mut().begins_at = runtime::InputValue::Value(value.into());
        self
    }
    pub fn begins_at_null(mut self) -> Self {
        self.input_mut().begins_at = runtime::InputValue::Null;
        self
    }
    pub fn cs_description(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cs_description = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cs_summary(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cs_summary = runtime::InputValue::Value(value.into());
        self
    }
    pub fn duration(mut self, value: i64) -> Self {
        self.input_mut().duration = runtime::InputValue::Value(value);
        self
    }
    pub fn en_description(mut self, value: impl Into<String>) -> Self {
        self.input_mut().en_description = runtime::InputValue::Value(value.into());
        self
    }
    pub fn en_summary(mut self, value: impl Into<String>) -> Self {
        self.input_mut().en_summary = runtime::InputValue::Value(value.into());
        self
    }
    pub fn finished_at(mut self, value: impl Into<String>) -> Self {
        self.input_mut().finished_at = runtime::InputValue::Value(value.into());
        self
    }
    pub fn finished_at_null(mut self) -> Self {
        self.input_mut().finished_at = runtime::InputValue::Null;
        self
    }
    pub fn impact(mut self, value: impl Into<String>) -> Self {
        self.input_mut().impact = runtime::InputValue::Value(value.into());
        self
    }
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.input_mut().type_ = runtime::InputValue::Value(value.into());
        self
    }
}
