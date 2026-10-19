use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "arch", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub arch: runtime::InputValue<String>,
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
    #[serde(
        rename = "distribution",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub distribution: runtime::InputValue<String>,
    #[serde(
        rename = "enable_cloud_init",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_cloud_init: runtime::InputValue<bool>,
    #[serde(
        rename = "enable_script",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enable_script: runtime::InputValue<bool>,
    #[serde(
        rename = "enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "hypervisor_type",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub hypervisor_type: runtime::InputValue<String>,
    #[serde(rename = "info", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub info: runtime::InputValue<String>,
    #[serde(
        rename = "label",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub label: runtime::InputValue<String>,
    #[serde(
        rename = "manage_dns_resolver",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub manage_dns_resolver: runtime::InputValue<bool>,
    #[serde(
        rename = "manage_hostname",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub manage_hostname: runtime::InputValue<bool>,
    #[serde(
        rename = "order",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub order: runtime::InputValue<i64>,
    #[serde(
        rename = "os_family",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub os_family: runtime::InputValue<i64>,
    #[serde(
        rename = "supported",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub supported: runtime::InputValue<bool>,
    #[serde(
        rename = "variant",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub variant: runtime::InputValue<String>,
    #[serde(
        rename = "vendor",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub vendor: runtime::InputValue<String>,
    #[serde(
        rename = "version",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub version: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.arch)?;
        runtime::Validate::validate(&self.cgroup_version)?;
        runtime::Validate::validate(&self.config)?;
        runtime::Validate::validate(&self.distribution)?;
        runtime::Validate::validate(&self.enable_cloud_init)?;
        runtime::Validate::validate(&self.enable_script)?;
        runtime::Validate::validate(&self.enabled)?;
        runtime::Validate::validate(&self.hypervisor_type)?;
        runtime::Validate::validate(&self.info)?;
        runtime::Validate::validate(&self.label)?;
        runtime::Validate::validate(&self.manage_dns_resolver)?;
        runtime::Validate::validate(&self.manage_hostname)?;
        runtime::Validate::validate(&self.order)?;
        runtime::Validate::validate(&self.os_family)?;
        runtime::Validate::validate(&self.supported)?;
        runtime::Validate::validate(&self.variant)?;
        runtime::Validate::validate(&self.vendor)?;
        runtime::Validate::validate(&self.version)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "arch")]
    pub arch: Option<String>,
    #[serde(default, rename = "cgroup_version")]
    pub cgroup_version: Option<String>,
    #[serde(default, rename = "config")]
    pub config: Option<String>,
    #[serde(default, rename = "distribution")]
    pub distribution: Option<String>,
    #[serde(default, rename = "enable_cloud_init")]
    pub enable_cloud_init: Option<bool>,
    #[serde(default, rename = "enable_script")]
    pub enable_script: Option<bool>,
    #[serde(default, rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "hypervisor_type")]
    pub hypervisor_type: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "info")]
    pub info: Option<String>,
    #[serde(default, rename = "label")]
    pub label: Option<String>,
    #[serde(default, rename = "manage_dns_resolver")]
    pub manage_dns_resolver: Option<bool>,
    #[serde(default, rename = "manage_hostname")]
    pub manage_hostname: Option<bool>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "order")]
    pub order: Option<i64>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "os_family")]
    pub os_family: Option<runtime::Association>,
    #[serde(default, rename = "supported")]
    pub supported: Option<bool>,
    #[serde(default, rename = "variant")]
    pub variant: Option<String>,
    #[serde(default, rename = "vendor")]
    pub vendor: Option<String>,
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
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "os_template/update",
    method: runtime::Method::Put,
    path: "v7.0/os_templates/{os_template_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("os_template"),
        parameters: &[
            runtime::ParameterSpec {
                name: "arch",
                kind: runtime::ParameterKind::String,
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
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "distribution",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_cloud_init",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enable_script",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "enabled",
                kind: runtime::ParameterKind::Boolean,
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
                name: "info",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "label",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "manage_dns_resolver",
                kind: runtime::ParameterKind::Boolean,
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
                name: "order",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "os_family",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "supported",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "variant",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "vendor",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "version",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "os_template",
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
    pub fn arch(mut self, value: impl Into<String>) -> Self {
        self.input_mut().arch = runtime::InputValue::Value(value.into());
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
    pub fn distribution(mut self, value: impl Into<String>) -> Self {
        self.input_mut().distribution = runtime::InputValue::Value(value.into());
        self
    }
    pub fn enable_cloud_init(mut self, value: bool) -> Self {
        self.input_mut().enable_cloud_init = runtime::InputValue::Value(value);
        self
    }
    pub fn enable_script(mut self, value: bool) -> Self {
        self.input_mut().enable_script = runtime::InputValue::Value(value);
        self
    }
    pub fn enabled(mut self, value: bool) -> Self {
        self.input_mut().enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn hypervisor_type(mut self, value: impl Into<String>) -> Self {
        self.input_mut().hypervisor_type = runtime::InputValue::Value(value.into());
        self
    }
    pub fn info(mut self, value: impl Into<String>) -> Self {
        self.input_mut().info = runtime::InputValue::Value(value.into());
        self
    }
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.input_mut().label = runtime::InputValue::Value(value.into());
        self
    }
    pub fn manage_dns_resolver(mut self, value: bool) -> Self {
        self.input_mut().manage_dns_resolver = runtime::InputValue::Value(value);
        self
    }
    pub fn manage_hostname(mut self, value: bool) -> Self {
        self.input_mut().manage_hostname = runtime::InputValue::Value(value);
        self
    }
    pub fn order(mut self, value: i64) -> Self {
        self.input_mut().order = runtime::InputValue::Value(value);
        self
    }
    pub fn os_family(mut self, value: i64) -> Self {
        self.input_mut().os_family = runtime::InputValue::Value(value);
        self
    }
    pub fn supported(mut self, value: bool) -> Self {
        self.input_mut().supported = runtime::InputValue::Value(value);
        self
    }
    pub fn variant(mut self, value: impl Into<String>) -> Self {
        self.input_mut().variant = runtime::InputValue::Value(value.into());
        self
    }
    pub fn vendor(mut self, value: impl Into<String>) -> Self {
        self.input_mut().vendor = runtime::InputValue::Value(value.into());
        self
    }
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.input_mut().version = runtime::InputValue::Value(value.into());
        self
    }
}
