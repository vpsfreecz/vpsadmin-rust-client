use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "cleanup_data",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub cleanup_data: runtime::InputValue<bool>,
    #[serde(
        rename = "finish_minutes",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub finish_minutes: runtime::InputValue<i64>,
    #[serde(
        rename = "finish_weekday",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub finish_weekday: runtime::InputValue<i64>,
    #[serde(
        rename = "maintenance_window",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub maintenance_window: runtime::InputValue<bool>,
    #[serde(
        rename = "no_start",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub no_start: runtime::InputValue<bool>,
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
    #[serde(
        rename = "reason",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reason: runtime::InputValue<String>,
    #[serde(
        rename = "replace_ip_addresses",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub replace_ip_addresses: runtime::InputValue<bool>,
    #[serde(
        rename = "send_mail",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub send_mail: runtime::InputValue<bool>,
    #[serde(
        rename = "skip_start",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub skip_start: runtime::InputValue<bool>,
    #[serde(rename = "swap", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub swap: runtime::InputValue<String>,
    #[serde(
        rename = "transfer_ip_addresses",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub transfer_ip_addresses: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.cleanup_data)?;
        runtime::Validate::validate(&self.finish_minutes)?;
        runtime::Validate::validate(&self.finish_weekday)?;
        runtime::Validate::validate(&self.maintenance_window)?;
        runtime::Validate::validate(&self.no_start)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.reason)?;
        runtime::Validate::validate(&self.replace_ip_addresses)?;
        runtime::Validate::validate(&self.send_mail)?;
        runtime::Validate::validate(&self.skip_start)?;
        runtime::Validate::validate(&self.swap)?;
        runtime::Validate::validate(&self.transfer_ip_addresses)?;
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
    name: "vps/migrate",
    method: runtime::Method::Post,
    path: "v7.0/vpses/{vps_id}/migrate",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("vps"),
        parameters: &[
            runtime::ParameterSpec {
                name: "cleanup_data",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "finish_minutes",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "finish_weekday",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "maintenance_window",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "no_start",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "node",
                kind: runtime::ParameterKind::Resource,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "reason",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "replace_ip_addresses",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "send_mail",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "skip_start",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "swap",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "transfer_ip_addresses",
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
    pub fn cleanup_data(mut self, value: bool) -> Self {
        self.input_mut().cleanup_data = runtime::InputValue::Value(value);
        self
    }
    pub fn finish_minutes(mut self, value: i64) -> Self {
        self.input_mut().finish_minutes = runtime::InputValue::Value(value);
        self
    }
    pub fn finish_weekday(mut self, value: i64) -> Self {
        self.input_mut().finish_weekday = runtime::InputValue::Value(value);
        self
    }
    pub fn maintenance_window(mut self, value: bool) -> Self {
        self.input_mut().maintenance_window = runtime::InputValue::Value(value);
        self
    }
    pub fn no_start(mut self, value: bool) -> Self {
        self.input_mut().no_start = runtime::InputValue::Value(value);
        self
    }
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
        self
    }
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reason = runtime::InputValue::Value(value.into());
        self
    }
    pub fn replace_ip_addresses(mut self, value: bool) -> Self {
        self.input_mut().replace_ip_addresses = runtime::InputValue::Value(value);
        self
    }
    pub fn send_mail(mut self, value: bool) -> Self {
        self.input_mut().send_mail = runtime::InputValue::Value(value);
        self
    }
    pub fn skip_start(mut self, value: bool) -> Self {
        self.input_mut().skip_start = runtime::InputValue::Value(value);
        self
    }
    pub fn swap(mut self, value: impl Into<String>) -> Self {
        self.input_mut().swap = runtime::InputValue::Value(value.into());
        self
    }
    pub fn transfer_ip_addresses(mut self, value: bool) -> Self {
        self.input_mut().transfer_ip_addresses = runtime::InputValue::Value(value);
        self
    }
}
