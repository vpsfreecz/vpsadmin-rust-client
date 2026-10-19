use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "cpus", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub cpus: runtime::InputValue<i64>,
    #[serde(
        rename = "hypervisor_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub hypervisor_type: runtime::InputValue<String>,
    #[serde(rename = "id", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub id: runtime::InputValue<i64>,
    #[serde(
        rename = "ip_addr",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ip_addr: runtime::InputValue<String>,
    #[serde(
        rename = "location",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub location: runtime::InputValue<i64>,
    #[serde(
        rename = "maintenance",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub maintenance: runtime::InputValue<bool>,
    #[serde(
        rename = "max_rx",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub max_rx: runtime::InputValue<i64>,
    #[serde(
        rename = "max_tx",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub max_tx: runtime::InputValue<i64>,
    #[serde(
        rename = "max_vps",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub max_vps: runtime::InputValue<i64>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(
        rename = "total_memory",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub total_memory: runtime::InputValue<i64>,
    #[serde(
        rename = "total_swap",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub total_swap: runtime::InputValue<i64>,
    #[serde(rename = "type", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub type_: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cpus)?;
        runtime::Validate::validate(&self.hypervisor_type)?;
        runtime::Validate::validate(&self.id)?;
        runtime::Validate::validate(&self.ip_addr)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.maintenance)?;
        runtime::Validate::validate(&self.max_rx)?;
        runtime::Validate::validate(&self.max_tx)?;
        runtime::Validate::validate(&self.max_vps)?;
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.total_memory)?;
        runtime::Validate::validate(&self.total_swap)?;
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
    #[serde(default, rename = "action_state_id")]
    pub action_state_id: Option<i64>,
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "node/create",
    method: runtime::Method::Post,
    path: "v7.0/nodes",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("node"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cpus",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: true,
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
                name: "id",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ip_addr",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "location",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "maintenance",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "max_rx",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "max_tx",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "max_vps",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "name",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "total_memory",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "total_swap",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "type",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "node",
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
    pub fn cpus(mut self, value: i64) -> Self {
        self.input_mut().cpus = runtime::InputValue::Value(value);
        self
    }
    pub fn cpus_null(mut self) -> Self {
        self.input_mut().cpus = runtime::InputValue::Null;
        self
    }
    pub fn hypervisor_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().hypervisor_type = runtime::InputValue::Value(value.into());
        self
    }
    pub fn id(mut self, value: i64) -> Self {
        self.input_mut().id = runtime::InputValue::Value(value);
        self
    }
    pub fn ip_addr(mut self, value: impl Into<String>) -> Self {
        self.input_mut().ip_addr = runtime::InputValue::Value(value.into());
        self
    }
    pub fn location(mut self, value: i64) -> Self {
        self.input_mut().location = runtime::InputValue::Value(value);
        self
    }
    pub fn maintenance(mut self, value: bool) -> Self {
        self.input_mut().maintenance = runtime::InputValue::Value(value);
        self
    }
    pub fn max_rx(mut self, value: i64) -> Self {
        self.input_mut().max_rx = runtime::InputValue::Value(value);
        self
    }
    pub fn max_tx(mut self, value: i64) -> Self {
        self.input_mut().max_tx = runtime::InputValue::Value(value);
        self
    }
    pub fn max_vps(mut self, value: i64) -> Self {
        self.input_mut().max_vps = runtime::InputValue::Value(value);
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn total_memory(mut self, value: i64) -> Self {
        self.input_mut().total_memory = runtime::InputValue::Value(value);
        self
    }
    pub fn total_memory_null(mut self) -> Self {
        self.input_mut().total_memory = runtime::InputValue::Null;
        self
    }
    pub fn total_swap(mut self, value: i64) -> Self {
        self.input_mut().total_swap = runtime::InputValue::Value(value);
        self
    }
    pub fn total_swap_null(mut self) -> Self {
        self.input_mut().total_swap = runtime::InputValue::Null;
        self
    }
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.input_mut().type_ = runtime::InputValue::Value(value.into());
        self
    }
}
