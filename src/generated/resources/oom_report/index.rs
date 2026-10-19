use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cgroup",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cgroup: runtime::InputValue<String>,
    #[serde(
        rename = "environment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub environment: runtime::InputValue<i64>,
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
    #[serde(
        rename = "location",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub location: runtime::InputValue<i64>,
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
    #[serde(
        rename = "oom_report_rule",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub oom_report_rule: runtime::InputValue<i64>,
    #[serde(
        rename = "since",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub since: runtime::InputValue<String>,
    #[serde(
        rename = "until",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub until: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
    #[serde(rename = "vps", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub vps: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cgroup)?;
        runtime::Validate::validate(&self.environment)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.oom_report_rule)?;
        runtime::Validate::validate(&self.since)?;
        runtime::Validate::validate(&self.until)?;
        runtime::Validate::validate(&self.user)?;
        runtime::Validate::validate(&self.vps)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "cgroup")]
    pub cgroup: Option<String>,
    #[serde(default, rename = "count")]
    pub count: Option<i64>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "invoked_by_name")]
    pub invoked_by_name: Option<String>,
    #[serde(default, rename = "invoked_by_pid")]
    pub invoked_by_pid: Option<i64>,
    #[serde(default, rename = "killed_name")]
    pub killed_name: Option<String>,
    #[serde(default, rename = "killed_pid")]
    pub killed_pid: Option<i64>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "oom_report_rule")]
    pub oom_report_rule: Option<runtime::Association>,
    #[serde(default, rename = "reported_at")]
    pub reported_at: Option<String>,
    #[doc = "Association keys: ID=id, label=hostname"]
    #[serde(default, rename = "vps")]
    pub vps: Option<runtime::Association>,
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
    name: "oom_report/index",
    method: runtime::Method::Get,
    path: "v7.0/oom_reports",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("oom_report"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cgroup",
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
                name: "oom_report_rule",
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
                name: "until",
                kind: runtime::ParameterKind::Datetime,
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
        namespace: "oom_reports",
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
    pub fn cgroup(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cgroup = runtime::InputValue::Value(value.into());
        self
    }
    pub fn environment(mut self, value: i64) -> Self {
        self.input_mut().environment = runtime::InputValue::Value(value);
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
    pub fn location(mut self, value: i64) -> Self {
        self.input_mut().location = runtime::InputValue::Value(value);
        self
    }
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
        self
    }
    pub fn oom_report_rule(mut self, value: i64) -> Self {
        self.input_mut().oom_report_rule = runtime::InputValue::Value(value);
        self
    }
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.input_mut().since = runtime::InputValue::Value(value.into());
        self
    }
    pub fn until(mut self, value: impl Into<String>) -> Self {
        self.input_mut().until = runtime::InputValue::Value(value.into());
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
