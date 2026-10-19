use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "expiration_date",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub expiration_date: runtime::InputValue<String>,
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
    #[serde(
        rename = "preserve_backup_history",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub preserve_backup_history: runtime::InputValue<bool>,
    #[serde(
        rename = "preserve_backups",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub preserve_backups: runtime::InputValue<bool>,
    #[serde(
        rename = "reason",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reason: runtime::InputValue<String>,
    #[serde(
        rename = "start",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub start: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.expiration_date)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.preserve_backup_history)?;
        runtime::Validate::validate(&self.preserve_backups)?;
        runtime::Validate::validate(&self.reason)?;
        runtime::Validate::validate(&self.start)?;
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
    name: "vps/replace",
    method: runtime::Method::Post,
    path: "v7.0/vpses/{vps_id}/replace",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("vps"),
        parameters: &[
            runtime::ParameterSpec {
                name: "expiration_date",
                kind: runtime::ParameterKind::Datetime,
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
                name: "preserve_backup_history",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "preserve_backups",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "reason",
                kind: runtime::ParameterKind::Text,
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
    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.input_mut().expiration_date = runtime::InputValue::Value(value.into());
        self
    }
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
        self
    }
    pub fn preserve_backup_history(mut self, value: bool) -> Self {
        self.input_mut().preserve_backup_history = runtime::InputValue::Value(value);
        self
    }
    pub fn preserve_backups(mut self, value: bool) -> Self {
        self.input_mut().preserve_backups = runtime::InputValue::Value(value);
        self
    }
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reason = runtime::InputValue::Value(value.into());
        self
    }
    pub fn start(mut self, value: bool) -> Self {
        self.input_mut().start = runtime::InputValue::Value(value);
        self
    }
}
