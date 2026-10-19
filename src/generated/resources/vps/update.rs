use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "admin_lock_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub admin_lock_type: runtime::InputValue<String>,
    #[serde(
        rename = "admin_override",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub admin_override: runtime::InputValue<bool>,
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
        rename = "change_reason",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub change_reason: runtime::InputValue<String>,
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
        rename = "expiration_date",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub expiration_date: runtime::InputValue<String>,
    #[serde(
        rename = "hostname",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub hostname: runtime::InputValue<String>,
    #[serde(rename = "info", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub info: runtime::InputValue<String>,
    #[serde(
        rename = "manage_hostname",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub manage_hostname: runtime::InputValue<bool>,
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
        rename = "object_state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub object_state: runtime::InputValue<String>,
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
        rename = "remind_after_date",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub remind_after_date: runtime::InputValue<String>,
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
        rename = "user_namespace_map",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub user_namespace_map: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.admin_lock_type)?;
        runtime::Validate::validate(&self.admin_override)?;
        runtime::Validate::validate(&self.allow_admin_modifications)?;
        runtime::Validate::validate(&self.autostart_enable)?;
        runtime::Validate::validate(&self.autostart_priority)?;
        runtime::Validate::validate(&self.cgroup_version)?;
        runtime::Validate::validate(&self.change_reason)?;
        runtime::Validate::validate(&self.config)?;
        runtime::Validate::validate(&self.cpu)?;
        runtime::Validate::validate(&self.cpu_limit)?;
        runtime::Validate::validate(&self.dns_resolver)?;
        runtime::Validate::validate(&self.enable_network)?;
        runtime::Validate::validate(&self.enable_os_template_auto_update)?;
        runtime::Validate::validate(&self.expiration_date)?;
        runtime::Validate::validate(&self.hostname)?;
        runtime::Validate::validate(&self.info)?;
        runtime::Validate::validate(&self.manage_hostname)?;
        runtime::Validate::validate(&self.map_mode)?;
        runtime::Validate::validate(&self.memory)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.object_state)?;
        runtime::Validate::validate(&self.onstartall)?;
        runtime::Validate::validate(&self.os_template)?;
        runtime::Validate::validate(&self.remind_after_date)?;
        runtime::Validate::validate(&self.start_menu_timeout)?;
        runtime::Validate::validate(&self.swap)?;
        runtime::Validate::validate(&self.user)?;
        runtime::Validate::validate(&self.user_namespace_map)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
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
    name: "vps/update",
    method: runtime::Method::Put,
    path: "v7.0/vpses/{vps_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("vps"),
        parameters: &[
            runtime::ParameterSpec {
                name: "admin_lock_type",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "admin_override",
                kind: runtime::ParameterKind::Boolean,
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
                name: "change_reason",
                kind: runtime::ParameterKind::Text,
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
                name: "expiration_date",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "hostname",
                kind: runtime::ParameterKind::String,
                required: false,
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
                name: "manage_hostname",
                kind: runtime::ParameterKind::Boolean,
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
                name: "object_state",
                kind: runtime::ParameterKind::String,
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
                name: "remind_after_date",
                kind: runtime::ParameterKind::Datetime,
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
                name: "user_namespace_map",
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
        allow_empty_response: true,
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
    pub fn admin_lock_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().admin_lock_type = runtime::InputValue::Value(value.into());
        self
    }
    pub fn admin_override(mut self, value: bool) -> Self {
        self.input_mut().admin_override = runtime::InputValue::Value(value);
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
    pub fn change_reason(mut self, value: impl Into<String>) -> Self {
        self.input_mut().change_reason = runtime::InputValue::Value(value.into());
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
    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.input_mut().expiration_date = runtime::InputValue::Value(value.into());
        self
    }
    pub fn expiration_date_null(mut self) -> Self {
        self.input_mut().expiration_date = runtime::InputValue::Null;
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
    pub fn manage_hostname(mut self, value: bool) -> Self {
        self.input_mut().manage_hostname = runtime::InputValue::Value(value);
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
    pub fn object_state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().object_state = runtime::InputValue::Value(value.into());
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
    pub fn remind_after_date(mut self, value: impl Into<String>) -> Self {
        self.input_mut().remind_after_date = runtime::InputValue::Value(value.into());
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
    pub fn user_namespace_map(mut self, value: i64) -> Self {
        self.input_mut().user_namespace_map = runtime::InputValue::Value(value);
        self
    }
}
