use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cgroup_version",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cgroup_version: runtime::InputValue<String>,
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
        rename = "os_family",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub os_family: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cgroup_version)?;
        runtime::Validate::validate(&self.enable_cloud_init)?;
        runtime::Validate::validate(&self.enable_script)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.hypervisor_type)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.os_family)?;
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "os_template/index",
    method: runtime::Method::Get,
    path: "v7.0/os_templates",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("os_template"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cgroup_version",
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
                name: "os_family",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "os_templates",
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
    pub fn cgroup_version(mut self, value: impl Into<String>) -> Self {
        self.input_mut().cgroup_version = runtime::InputValue::Value(value.into());
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
    pub fn os_family(mut self, value: i64) -> Self {
        self.input_mut().os_family = runtime::InputValue::Value(value);
        self
    }
}
