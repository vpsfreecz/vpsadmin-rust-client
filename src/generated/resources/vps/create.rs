use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "address_location",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub address_location: runtime::InputValue<i64>,
    #[serde(
        rename = "allow_admin_modifications",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub allow_admin_modifications: runtime::InputValue<bool>,
    #[serde(
        rename = "autostart_enable",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub autostart_enable: runtime::InputValue<bool>,
    #[serde(
        rename = "autostart_priority",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub autostart_priority: runtime::InputValue<i64>,
    #[serde(
        rename = "cgroup_version",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cgroup_version: runtime::InputValue<String>,
    #[serde(
        rename = "config",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub config: runtime::InputValue<String>,
    #[serde(rename = "cpu", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub cpu: runtime::InputValue<i64>,
    #[serde(
        rename = "cpu_limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cpu_limit: runtime::InputValue<i64>,
    #[serde(
        rename = "diskspace",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub diskspace: runtime::InputValue<i64>,
    #[serde(
        rename = "dns_resolver",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dns_resolver: runtime::InputValue<i64>,
    #[serde(
        rename = "enable_network",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_network: runtime::InputValue<bool>,
    #[serde(
        rename = "enable_os_template_auto_update",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_os_template_auto_update: runtime::InputValue<bool>,
    #[serde(
        rename = "environment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub environment: runtime::InputValue<i64>,
    #[serde(
        rename = "hostname",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub hostname: runtime::InputValue<String>,
    #[serde(rename = "info", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub info: runtime::InputValue<String>,
    #[serde(rename = "ipv4", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub ipv4: runtime::InputValue<i64>,
    #[serde(
        rename = "ipv4_private",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub ipv4_private: runtime::InputValue<i64>,
    #[serde(rename = "ipv6", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub ipv6: runtime::InputValue<i64>,
    #[serde(
        rename = "location",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub location: runtime::InputValue<i64>,
    #[serde(
        rename = "map_mode",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub map_mode: runtime::InputValue<String>,
    #[serde(
        rename = "memory",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub memory: runtime::InputValue<i64>,
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
    #[serde(
        rename = "onstartall",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub onstartall: runtime::InputValue<bool>,
    #[serde(
        rename = "os_template",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub os_template: runtime::InputValue<i64>,
    #[serde(
        rename = "start",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub start: runtime::InputValue<bool>,
    #[serde(
        rename = "start_menu_timeout",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub start_menu_timeout: runtime::InputValue<i64>,
    #[serde(rename = "swap", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub swap: runtime::InputValue<i64>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
    #[serde(
        rename = "user_data_content",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub user_data_content: runtime::InputValue<String>,
    #[serde(
        rename = "user_data_format",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub user_data_format: runtime::InputValue<String>,
    #[serde(
        rename = "user_namespace_map",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub user_namespace_map: runtime::InputValue<i64>,
    #[serde(
        rename = "vps_user_data",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub vps_user_data: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.address_location)?;
        runtime::Validate::validate(&self.allow_admin_modifications)?;
        runtime::Validate::validate(&self.autostart_enable)?;
        runtime::Validate::validate(&self.autostart_priority)?;
        runtime::Validate::validate(&self.cgroup_version)?;
        runtime::Validate::validate(&self.config)?;
        runtime::Validate::validate(&self.cpu)?;
        runtime::Validate::validate(&self.cpu_limit)?;
        runtime::Validate::validate(&self.diskspace)?;
        runtime::Validate::validate(&self.dns_resolver)?;
        runtime::Validate::validate(&self.enable_network)?;
        runtime::Validate::validate(&self.enable_os_template_auto_update)?;
        runtime::Validate::validate(&self.environment)?;
        runtime::Validate::validate(&self.hostname)?;
        runtime::Validate::validate(&self.info)?;
        runtime::Validate::validate(&self.ipv4)?;
        runtime::Validate::validate(&self.ipv4_private)?;
        runtime::Validate::validate(&self.ipv6)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.map_mode)?;
        runtime::Validate::validate(&self.memory)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.onstartall)?;
        runtime::Validate::validate(&self.os_template)?;
        runtime::Validate::validate(&self.start)?;
        runtime::Validate::validate(&self.start_menu_timeout)?;
        runtime::Validate::validate(&self.swap)?;
        runtime::Validate::validate(&self.user)?;
        runtime::Validate::validate(&self.user_data_content)?;
        runtime::Validate::validate(&self.user_data_format)?;
        runtime::Validate::validate(&self.user_namespace_map)?;
        runtime::Validate::validate(&self.vps_user_data)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "allow_admin_modifications")]
    pub allow_admin_modifications: Option<bool>,
    #[serde(default, rename = "autostart_enable")]
    pub autostart_enable: Option<bool>,
    #[serde(default, rename = "autostart_priority")]
    pub autostart_priority: Option<i64>,
    #[serde(default, rename = "cgroup_version")]
    pub cgroup_version: Option<String>,
    #[serde(default, rename = "config")]
    pub config: Option<String>,
    #[serde(default, rename = "cpu")]
    pub cpu: Option<i64>,
    #[serde(default, rename = "cpu_idle")]
    pub cpu_idle: Option<f64>,
    #[serde(default, rename = "cpu_iowait")]
    pub cpu_iowait: Option<f64>,
    #[serde(default, rename = "cpu_irq")]
    pub cpu_irq: Option<f64>,
    #[serde(default, rename = "cpu_limit")]
    pub cpu_limit: Option<i64>,
    #[serde(default, rename = "cpu_nice")]
    pub cpu_nice: Option<f64>,
    #[serde(default, rename = "cpu_softirq")]
    pub cpu_softirq: Option<f64>,
    #[serde(default, rename = "cpu_system")]
    pub cpu_system: Option<f64>,
    #[serde(default, rename = "cpu_user")]
    pub cpu_user: Option<f64>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "dataset")]
    pub dataset: Option<runtime::Association>,
    #[serde(default, rename = "diskspace")]
    pub diskspace: Option<i64>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "dns_resolver")]
    pub dns_resolver: Option<runtime::Association>,
    #[serde(default, rename = "enable_network")]
    pub enable_network: Option<bool>,
    #[serde(default, rename = "enable_os_template_auto_update")]
    pub enable_os_template_auto_update: Option<bool>,
    #[serde(default, rename = "expiration_date")]
    pub expiration_date: Option<String>,
    #[serde(default, rename = "hostname")]
    pub hostname: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "implicit_oom_report_rule_hit_count")]
    pub implicit_oom_report_rule_hit_count: Option<i64>,
    #[serde(default, rename = "in_rescue_mode")]
    pub in_rescue_mode: Option<bool>,
    #[serde(default, rename = "info")]
    pub info: Option<String>,
    #[serde(default, rename = "is_running")]
    pub is_running: Option<bool>,
    #[serde(default, rename = "loadavg1")]
    pub loadavg1: Option<f64>,
    #[serde(default, rename = "loadavg15")]
    pub loadavg15: Option<f64>,
    #[serde(default, rename = "loadavg5")]
    pub loadavg5: Option<f64>,
    #[serde(default, rename = "manage_hostname")]
    pub manage_hostname: Option<bool>,
    #[serde(default, rename = "map_mode")]
    pub map_mode: Option<String>,
    #[serde(default, rename = "memory")]
    pub memory: Option<i64>,
    #[doc = "Association keys: ID=id, label=domain_name"]
    #[serde(default, rename = "node")]
    pub node: Option<runtime::Association>,
    #[serde(default, rename = "object_state")]
    pub object_state: Option<String>,
    #[serde(default, rename = "onstartall")]
    pub onstartall: Option<bool>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "os_template")]
    pub os_template: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "pool")]
    pub pool: Option<runtime::Association>,
    #[serde(default, rename = "process_count")]
    pub process_count: Option<i64>,
    #[serde(default, rename = "remind_after_date")]
    pub remind_after_date: Option<String>,
    #[serde(default, rename = "start_menu_timeout")]
    pub start_menu_timeout: Option<i64>,
    #[serde(default, rename = "swap")]
    pub swap: Option<i64>,
    #[serde(default, rename = "uptime")]
    pub uptime: Option<i64>,
    #[serde(default, rename = "used_diskspace")]
    pub used_diskspace: Option<i64>,
    #[serde(default, rename = "used_memory")]
    pub used_memory: Option<i64>,
    #[serde(default, rename = "used_swap")]
    pub used_swap: Option<i64>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "user_namespace_map")]
    pub user_namespace_map: Option<runtime::Association>,
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
    name: "vps/create",
    method: runtime::Method::Post,
    path: "v7.0/vpses",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("vps"),
        parameters: &[
            runtime::ParameterSpec {
                name: "address_location",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "allow_admin_modifications",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "autostart_enable",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "autostart_priority",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "cgroup_version",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "config",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "cpu",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "cpu_limit",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "diskspace",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "dns_resolver",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_network",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_os_template_auto_update",
                kind: runtime::ParameterKind::Boolean,
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
                name: "hostname",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "info",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ipv4",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ipv4_private",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ipv6",
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
                name: "map_mode",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "memory",
                kind: runtime::ParameterKind::Integer,
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
                name: "onstartall",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "os_template",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "start",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "start_menu_timeout",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "swap",
                kind: runtime::ParameterKind::Integer,
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
                name: "user_data_content",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user_data_format",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "user_namespace_map",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vps_user_data",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "vps",
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
    pub fn address_location(mut self, value: i64) -> Self {
        self.input_mut().address_location = runtime::InputValue::Value(value);
        self
    }
    pub fn allow_admin_modifications(mut self, value: bool) -> Self {
        self.input_mut().allow_admin_modifications = runtime::InputValue::Value(value);
        self
    }
    pub fn autostart_enable(mut self, value: bool) -> Self {
        self.input_mut().autostart_enable = runtime::InputValue::Value(value);
        self
    }
    pub fn autostart_priority(mut self, value: i64) -> Self {
        self.input_mut().autostart_priority = runtime::InputValue::Value(value);
        self
    }
    pub fn cgroup_version(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cgroup_version = runtime::InputValue::Value(value.into());
        self
    }
    pub fn config(mut self, value: impl Into<String>) -> Self {
        self.input_mut().config = runtime::InputValue::Value(value.into());
        self
    }
    pub fn cpu(mut self, value: i64) -> Self {
        self.input_mut().cpu = runtime::InputValue::Value(value);
        self
    }
    pub fn cpu_limit(mut self, value: i64) -> Self {
        self.input_mut().cpu_limit = runtime::InputValue::Value(value);
        self
    }
    pub fn cpu_limit_null(mut self) -> Self {
        self.input_mut().cpu_limit = runtime::InputValue::Null;
        self
    }
    pub fn diskspace(mut self, value: i64) -> Self {
        self.input_mut().diskspace = runtime::InputValue::Value(value);
        self
    }
    pub fn dns_resolver(mut self, value: i64) -> Self {
        self.input_mut().dns_resolver = runtime::InputValue::Value(value);
        self
    }
    pub fn dns_resolver_null(mut self) -> Self {
        self.input_mut().dns_resolver = runtime::InputValue::Null;
        self
    }
    pub fn enable_network(mut self, value: bool) -> Self {
        self.input_mut().enable_network = runtime::InputValue::Value(value);
        self
    }
    pub fn enable_os_template_auto_update(mut self, value: bool) -> Self {
        self.input_mut().enable_os_template_auto_update = runtime::InputValue::Value(value);
        self
    }
    pub fn environment(mut self, value: i64) -> Self {
        self.input_mut().environment = runtime::InputValue::Value(value);
        self
    }
    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.input_mut().hostname = runtime::InputValue::Value(value.into());
        self
    }
    pub fn info(mut self, value: impl Into<String>) -> Self {
        self.input_mut().info = runtime::InputValue::Value(value.into());
        self
    }
    pub fn ipv4(mut self, value: i64) -> Self {
        self.input_mut().ipv4 = runtime::InputValue::Value(value);
        self
    }
    pub fn ipv4_private(mut self, value: i64) -> Self {
        self.input_mut().ipv4_private = runtime::InputValue::Value(value);
        self
    }
    pub fn ipv6(mut self, value: i64) -> Self {
        self.input_mut().ipv6 = runtime::InputValue::Value(value);
        self
    }
    pub fn location(mut self, value: i64) -> Self {
        self.input_mut().location = runtime::InputValue::Value(value);
        self
    }
    pub fn map_mode(mut self, value: impl Into<String>) -> Self {
        self.input_mut().map_mode = runtime::InputValue::Value(value.into());
        self
    }
    pub fn memory(mut self, value: i64) -> Self {
        self.input_mut().memory = runtime::InputValue::Value(value);
        self
    }
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
        self
    }
    pub fn onstartall(mut self, value: bool) -> Self {
        self.input_mut().onstartall = runtime::InputValue::Value(value);
        self
    }
    pub fn os_template(mut self, value: i64) -> Self {
        self.input_mut().os_template = runtime::InputValue::Value(value);
        self
    }
    pub fn start(mut self, value: bool) -> Self {
        self.input_mut().start = runtime::InputValue::Value(value);
        self
    }
    pub fn start_menu_timeout(mut self, value: i64) -> Self {
        self.input_mut().start_menu_timeout = runtime::InputValue::Value(value);
        self
    }
    pub fn swap(mut self, value: i64) -> Self {
        self.input_mut().swap = runtime::InputValue::Value(value);
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
    pub fn user_data_content(mut self, value: impl Into<String>) -> Self {
        self.input_mut().user_data_content = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user_data_format(mut self, value: impl Into<String>) -> Self {
        self.input_mut().user_data_format = runtime::InputValue::Value(value.into());
        self
    }
    pub fn user_namespace_map(mut self, value: i64) -> Self {
        self.input_mut().user_namespace_map = runtime::InputValue::Value(value);
        self
    }
    pub fn vps_user_data(mut self, value: i64) -> Self {
        self.input_mut().vps_user_data = runtime::InputValue::Value(value);
        self
    }
}
