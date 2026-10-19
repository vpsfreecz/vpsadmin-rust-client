use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "affected",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub affected: runtime::InputValue<bool>,
    #[serde(
        rename = "component",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub component: runtime::InputValue<i64>,
    #[serde(
        rename = "entity_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub entity_id: runtime::InputValue<i64>,
    #[serde(
        rename = "entity_name",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub entity_name: runtime::InputValue<String>,
    #[serde(
        rename = "environment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub environment: runtime::InputValue<i64>,
    #[serde(
        rename = "export",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub export: runtime::InputValue<i64>,
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "handled_by",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub handled_by: runtime::InputValue<i64>,
    #[serde(
        rename = "impact",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub impact: runtime::InputValue<String>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(
        rename = "location",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub location: runtime::InputValue<i64>,
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
    #[serde(
        rename = "order",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub order: runtime::InputValue<String>,
    #[serde(
        rename = "recent_since",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub recent_since: runtime::InputValue<String>,
    #[serde(
        rename = "security_advisory",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub security_advisory: runtime::InputValue<i64>,
    #[serde(
        rename = "since",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub since: runtime::InputValue<String>,
    #[serde(
        rename = "state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub state: runtime::InputValue<String>,
    #[serde(rename = "type", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub type_: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
    #[serde(rename = "vps", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub vps: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.affected)?;
        runtime::Validate::validate(&self.component)?;
        runtime::Validate::validate(&self.entity_id)?;
        runtime::Validate::validate(&self.entity_name)?;
        runtime::Validate::validate(&self.environment)?;
        runtime::Validate::validate(&self.export)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.handled_by)?;
        runtime::Validate::validate(&self.impact)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.order)?;
        runtime::Validate::validate(&self.recent_since)?;
        runtime::Validate::validate(&self.security_advisory)?;
        runtime::Validate::validate(&self.since)?;
        runtime::Validate::validate(&self.state)?;
        runtime::Validate::validate(&self.type_)?;
        runtime::Validate::validate(&self.user)?;
        runtime::Validate::validate(&self.vps)?;
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "outage/index",
    method: runtime::Method::Get,
    path: "v7.0/outages",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("outage"),
        parameters: &[
            runtime::ParameterSpec {
                name: "affected",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "component",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "entity_id",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "entity_name",
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
                name: "export",
                kind: runtime::ParameterKind::Resource,
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
                name: "handled_by",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
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
                name: "limit",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "location",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "node",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "order",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "recent_since",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "security_advisory",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "since",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "state",
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
            runtime::ParameterSpec {
                name: "user",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "outages",
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
    pub fn affected(mut self, value: bool) -> Self {
        self.input_mut().affected = runtime::InputValue::Value(value);
        self
    }
    pub fn component(mut self, value: i64) -> Self {
        self.input_mut().component = runtime::InputValue::Value(value);
        self
    }
    pub fn entity_id(mut self, value: i64) -> Self {
        self.input_mut().entity_id = runtime::InputValue::Value(value);
        self
    }
    pub fn entity_name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().entity_name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn environment(mut self, value: i64) -> Self {
        self.input_mut().environment = runtime::InputValue::Value(value);
        self
    }
    pub fn export(mut self, value: i64) -> Self {
        self.input_mut().export = runtime::InputValue::Value(value);
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn handled_by(mut self, value: i64) -> Self {
        self.input_mut().handled_by = runtime::InputValue::Value(value);
        self
    }
    pub fn impact(mut self, value: impl Into<String>) -> Self {
        self.input_mut().impact = runtime::InputValue::Value(value.into());
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn location(mut self, value: i64) -> Self {
        self.input_mut().location = runtime::InputValue::Value(value);
        self
    }
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
        self
    }
    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.input_mut().order = runtime::InputValue::Value(value.into());
        self
    }
    pub fn recent_since(mut self, value: impl Into<String>) -> Self {
        self.input_mut().recent_since = runtime::InputValue::Value(value.into());
        self
    }
    pub fn security_advisory(mut self, value: i64) -> Self {
        self.input_mut().security_advisory = runtime::InputValue::Value(value);
        self
    }
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.input_mut().since = runtime::InputValue::Value(value.into());
        self
    }
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().state = runtime::InputValue::Value(value.into());
        self
    }
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.input_mut().type_ = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
    pub fn vps(mut self, value: i64) -> Self {
        self.input_mut().vps = runtime::InputValue::Value(value);
        self
    }
}
