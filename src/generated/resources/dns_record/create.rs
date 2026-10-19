use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "comment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub comment: runtime::InputValue<String>,
    #[serde(
        rename = "content",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub content: runtime::InputValue<String>,
    #[serde(
        rename = "dns_zone",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dns_zone: runtime::InputValue<i64>,
    #[serde(
        rename = "dynamic_update_enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub dynamic_update_enabled: runtime::InputValue<bool>,
    #[serde(
        rename = "enabled",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub enabled: runtime::InputValue<bool>,
    #[serde(rename = "name", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub name: runtime::InputValue<String>,
    #[serde(
        rename = "priority",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub priority: runtime::InputValue<i64>,
    #[serde(rename = "ttl", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub ttl: runtime::InputValue<i64>,
    #[serde(rename = "type", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub type_: runtime::InputValue<String>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.comment)?;
        runtime::Validate::validate(&self.content)?;
        runtime::Validate::validate(&self.dns_zone)?;
        runtime::Validate::validate(&self.dynamic_update_enabled)?;
        runtime::Validate::validate(&self.enabled)?;
        runtime::Validate::validate(&self.name)?;
        runtime::Validate::validate(&self.priority)?;
        runtime::Validate::validate(&self.ttl)?;
        runtime::Validate::validate(&self.type_)?;
        runtime::Validate::validate(&self.user)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "comment")]
    pub comment: Option<String>,
    #[serde(default, rename = "content")]
    pub content: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[doc = "Association keys: ID=id, label=name"]
    #[serde(default, rename = "dns_zone")]
    pub dns_zone: Option<runtime::Association>,
    #[serde(default, rename = "dynamic_update_enabled")]
    pub dynamic_update_enabled: Option<bool>,
    #[serde(default, rename = "dynamic_update_url")]
    pub dynamic_update_url: Option<String>,
    #[serde(default, rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "managed")]
    pub managed: Option<bool>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "priority")]
    pub priority: Option<i64>,
    #[serde(default, rename = "ttl")]
    pub ttl: Option<i64>,
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "user")]
    pub user: Option<runtime::Association>,
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
    name: "dns_record/create",
    method: runtime::Method::Post,
    path: "v7.0/dns_records",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("dns_record"),
        parameters: &[
            runtime::ParameterSpec {
                name: "comment",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "content",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "dns_zone",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "dynamic_update_enabled",
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
                name: "name",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "priority",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ttl",
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
            runtime::ParameterSpec {
                name: "user",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: true,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "dns_record",
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
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.input_mut().comment = runtime::InputValue::Value(value.into());
        self
    }
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.input_mut().content = runtime::InputValue::Value(value.into());
        self
    }
    pub fn dns_zone(mut self, value: i64) -> Self {
        self.input_mut().dns_zone = runtime::InputValue::Value(value);
        self
    }
    pub fn dynamic_update_enabled(mut self, value: bool) -> Self {
        self.input_mut().dynamic_update_enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn enabled(mut self, value: bool) -> Self {
        self.input_mut().enabled = runtime::InputValue::Value(value);
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn priority(mut self, value: i64) -> Self {
        self.input_mut().priority = runtime::InputValue::Value(value);
        self
    }
    pub fn priority_null(mut self) -> Self {
        self.input_mut().priority = runtime::InputValue::Null;
        self
    }
    pub fn ttl(mut self, value: i64) -> Self {
        self.input_mut().ttl = runtime::InputValue::Value(value);
        self
    }
    pub fn ttl_null(mut self) -> Self {
        self.input_mut().ttl = runtime::InputValue::Null;
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
    pub fn user_null(mut self) -> Self {
        self.input_mut().user = runtime::InputValue::Null;
        self
    }
}
