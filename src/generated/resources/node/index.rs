use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
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
        rename = "hypervisor_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub hypervisor_type: runtime::InputValue<String>,
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
    #[serde(
        rename = "state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub state: runtime::InputValue<String>,
    #[serde(rename = "type", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub type_: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.environment)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.hypervisor_type)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.state)?;
        runtime::Validate::validate(&self.type_)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "active")]
    pub active: Option<bool>,
    #[serde(default, rename = "arc_c")]
    pub arc_c: Option<i64>,
    #[serde(default, rename = "arc_c_max")]
    pub arc_c_max: Option<i64>,
    #[serde(default, rename = "arc_hitpercent")]
    pub arc_hitpercent: Option<i64>,
    #[serde(default, rename = "arc_size")]
    pub arc_size: Option<i64>,
    #[serde(default, rename = "cgroup_version")]
    pub cgroup_version: Option<String>,
    #[serde(default, rename = "cpu_guest")]
    pub cpu_guest: Option<f64>,
    #[serde(default, rename = "cpu_idle")]
    pub cpu_idle: Option<f64>,
    #[serde(default, rename = "cpu_iowait")]
    pub cpu_iowait: Option<f64>,
    #[serde(default, rename = "cpu_irq")]
    pub cpu_irq: Option<f64>,
    #[serde(default, rename = "cpu_nice")]
    pub cpu_nice: Option<f64>,
    #[serde(default, rename = "cpu_softirq")]
    pub cpu_softirq: Option<f64>,
    #[serde(default, rename = "cpu_system")]
    pub cpu_system: Option<f64>,
    #[serde(default, rename = "cpu_user")]
    pub cpu_user: Option<f64>,
    #[serde(default, rename = "cpus")]
    pub cpus: Option<i64>,
    #[serde(default, rename = "domain_name")]
    pub domain_name: Option<String>,
    #[serde(default, rename = "fqdn")]
    pub fqdn: Option<String>,
    #[serde(default, rename = "hypervisor_type")]
    pub hypervisor_type: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "ip_addr")]
    pub ip_addr: Option<String>,
    #[serde(default, rename = "kernel")]
    pub kernel: Option<String>,
    #[serde(default, rename = "loadavg1")]
    pub loadavg1: Option<f64>,
    #[serde(default, rename = "loadavg15")]
    pub loadavg15: Option<f64>,
    #[serde(default, rename = "loadavg5")]
    pub loadavg5: Option<f64>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "location")]
    pub location: Option<runtime::Association>,
    #[serde(default, rename = "maintenance_lock")]
    pub maintenance_lock: Option<String>,
    #[serde(default, rename = "maintenance_lock_reason")]
    pub maintenance_lock_reason: Option<String>,
    #[serde(default, rename = "max_rx")]
    pub max_rx: Option<i64>,
    #[serde(default, rename = "max_tx")]
    pub max_tx: Option<i64>,
    #[serde(default, rename = "max_vps")]
    pub max_vps: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "pool_checked_at")]
    pub pool_checked_at: Option<String>,
    #[serde(default, rename = "pool_scan")]
    pub pool_scan: Option<String>,
    #[serde(default, rename = "pool_scan_percent")]
    pub pool_scan_percent: Option<f64>,
    #[serde(default, rename = "pool_state")]
    pub pool_state: Option<String>,
    #[serde(default, rename = "pool_status")]
    pub pool_status: Option<bool>,
    #[serde(default, rename = "process_count")]
    pub process_count: Option<i64>,
    #[serde(default, rename = "status")]
    pub status: Option<bool>,
    #[serde(default, rename = "total_memory")]
    pub total_memory: Option<i64>,
    #[serde(default, rename = "total_swap")]
    pub total_swap: Option<i64>,
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
    #[serde(default, rename = "uptime")]
    pub uptime: Option<i64>,
    #[serde(default, rename = "used_memory")]
    pub used_memory: Option<i64>,
    #[serde(default, rename = "used_swap")]
    pub used_swap: Option<i64>,
    #[serde(default, rename = "version")]
    pub version: Option<String>,
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
    name: "node/index",
    method: runtime::Method::Get,
    path: "v7.0/nodes",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node"),
        parameters: &[
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
                name: "hypervisor_type",
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
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "nodes",
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
    pub fn environment(mut self, value: i64) -> Self {
        self.input_mut().environment = runtime::InputValue::Value(value);
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn hypervisor_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().hypervisor_type = runtime::InputValue::Value(value.into());
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
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().state = runtime::InputValue::Value(value.into());
        self
    }
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.input_mut().type_ = runtime::InputValue::Value(value.into());
        self
    }
}
