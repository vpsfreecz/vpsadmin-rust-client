use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "action",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub action: runtime::InputValue<String>,
    #[serde(
        rename = "activate",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub activate: runtime::InputValue<bool>,
    #[serde(
        rename = "address",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub address: runtime::InputValue<String>,
    #[serde(
        rename = "create_vps",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub create_vps: runtime::InputValue<bool>,
    #[serde(
        rename = "currency",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub currency: runtime::InputValue<String>,
    #[serde(
        rename = "email",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub email: runtime::InputValue<String>,
    #[serde(
        rename = "full_name",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub full_name: runtime::InputValue<String>,
    #[serde(rename = "how", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub how: runtime::InputValue<String>,
    #[serde(
        rename = "language",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub language: runtime::InputValue<i64>,
    #[serde(
        rename = "location",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub location: runtime::InputValue<i64>,
    #[serde(
        rename = "login",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub login: runtime::InputValue<String>,
    #[serde(rename = "node", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub node: runtime::InputValue<i64>,
    #[serde(rename = "note", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub note: runtime::InputValue<String>,
    #[serde(
        rename = "org_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub org_id: runtime::InputValue<String>,
    #[serde(
        rename = "org_name",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub org_name: runtime::InputValue<String>,
    #[serde(
        rename = "os_template",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub os_template: runtime::InputValue<i64>,
    #[serde(
        rename = "reason",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub reason: runtime::InputValue<String>,
    #[serde(
        rename = "time_zone",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub time_zone: runtime::InputValue<String>,
    #[serde(
        rename = "year_of_birth",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub year_of_birth: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.action)?;
        runtime::Validate::validate(&self.activate)?;
        runtime::Validate::validate(&self.address)?;
        runtime::Validate::validate(&self.create_vps)?;
        runtime::Validate::validate(&self.currency)?;
        runtime::Validate::validate(&self.email)?;
        runtime::Validate::validate(&self.full_name)?;
        runtime::Validate::validate(&self.how)?;
        runtime::Validate::validate(&self.language)?;
        runtime::Validate::validate(&self.location)?;
        runtime::Validate::validate(&self.login)?;
        runtime::Validate::validate(&self.node)?;
        runtime::Validate::validate(&self.note)?;
        runtime::Validate::validate(&self.org_id)?;
        runtime::Validate::validate(&self.org_name)?;
        runtime::Validate::validate(&self.os_template)?;
        runtime::Validate::validate(&self.reason)?;
        runtime::Validate::validate(&self.time_zone)?;
        runtime::Validate::validate(&self.year_of_birth)?;
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
    #[serde(default, rename = "path_params")]
    pub path_params: Option<serde_json::Value>,
    #[serde(default, rename = "resolved")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "user_request/registration/resolve",
    method: runtime::Method::Post,
    path: "v7.0/user_request/registrations/{registration_id}/resolve",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("registration"),
        parameters: &[
            runtime::ParameterSpec {
                name: "action",
                kind: runtime::ParameterKind::String,
                required: true,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "activate",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "address",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "create_vps",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "currency",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "email",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "full_name",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "how",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "language",
                kind: runtime::ParameterKind::Resource,
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
                name: "login",
                kind: runtime::ParameterKind::String,
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
                name: "note",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "org_id",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "org_name",
                kind: runtime::ParameterKind::String,
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
                name: "reason",
                kind: runtime::ParameterKind::Text,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "time_zone",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: true,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "year_of_birth",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "registration",
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
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.input_mut().action = runtime::InputValue::Value(value.into());
        self
    }
    pub fn activate(mut self, value: bool) -> Self {
        self.input_mut().activate = runtime::InputValue::Value(value);
        self
    }
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.input_mut().address = runtime::InputValue::Value(value.into());
        self
    }
    pub fn create_vps(mut self, value: bool) -> Self {
        self.input_mut().create_vps = runtime::InputValue::Value(value);
        self
    }
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.input_mut().currency = runtime::InputValue::Value(value.into());
        self
    }
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.input_mut().email = runtime::InputValue::Value(value.into());
        self
    }
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().full_name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn how(mut self, value: impl Into<String>) -> Self {
        self.input_mut().how = runtime::InputValue::Value(value.into());
        self
    }
    pub fn language(mut self, value: i64) -> Self {
        self.input_mut().language = runtime::InputValue::Value(value);
        self
    }
    pub fn location(mut self, value: i64) -> Self {
        self.input_mut().location = runtime::InputValue::Value(value);
        self
    }
    pub fn login(mut self, value: impl Into<String>) -> Self {
        self.input_mut().login = runtime::InputValue::Value(value.into());
        self
    }
    pub fn node(mut self, value: i64) -> Self {
        self.input_mut().node = runtime::InputValue::Value(value);
        self
    }
    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.input_mut().note = runtime::InputValue::Value(value.into());
        self
    }
    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.input_mut().org_id = runtime::InputValue::Value(value.into());
        self
    }
    pub fn org_name(mut self, value: impl Into<String>) -> Self {
        self.input_mut().org_name = runtime::InputValue::Value(value.into());
        self
    }
    pub fn os_template(mut self, value: i64) -> Self {
        self.input_mut().os_template = runtime::InputValue::Value(value);
        self
    }
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.input_mut().reason = runtime::InputValue::Value(value.into());
        self
    }
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.input_mut().time_zone = runtime::InputValue::Value(value.into());
        self
    }
    pub fn time_zone_null(mut self) -> Self {
        self.input_mut().time_zone = runtime::InputValue::Null;
        self
    }
    pub fn year_of_birth(mut self, value: i64) -> Self {
        self.input_mut().year_of_birth = runtime::InputValue::Value(value);
        self
    }
}
