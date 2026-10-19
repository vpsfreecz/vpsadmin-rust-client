use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "accounted_by",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub accounted_by: runtime::InputValue<i64>,
    #[serde(
        rename = "created_from",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub created_from: runtime::InputValue<String>,
    #[serde(
        rename = "created_to",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub created_to: runtime::InputValue<String>,
    #[serde(
        rename = "from_id",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub from_id: runtime::InputValue<i64>,
    #[serde(
        rename = "limit",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub limit: runtime::InputValue<i64>,
    #[serde(rename = "user", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub user: runtime::InputValue<i64>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.accounted_by)?;
        runtime::Validate::validate(&self.created_from)?;
        runtime::Validate::validate(&self.created_to)?;
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.user)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[doc = "Association keys: ID=id, label=login"]
    #[serde(default, rename = "accounted_by")]
    pub accounted_by: Option<runtime::Association>,
    #[serde(default, rename = "amount")]
    pub amount: Option<i64>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "from_date")]
    pub from_date: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=transaction_id"]
    #[serde(default, rename = "incoming_payment")]
    pub incoming_payment: Option<runtime::Association>,
    #[serde(default, rename = "to_date")]
    pub to_date: Option<String>,
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
    #[serde(default, rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub extra: runtime::JsonMap,
}

pub static SPEC: runtime::ActionSpec = runtime::ActionSpec {
    name: "user_payment/index",
    method: runtime::Method::Get,
    path: "v7.0/user_payments",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("user_payment"),
        parameters: &[
            runtime::ParameterSpec {
                name: "accounted_by",
                kind: runtime::ParameterKind::Resource,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "created_from",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "created_to",
                kind: runtime::ParameterKind::Datetime,
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
                name: "limit",
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
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "user_payments",
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
    pub fn accounted_by(mut self, value: i64) -> Self {
        self.input_mut().accounted_by = runtime::InputValue::Value(value);
        self
    }
    pub fn created_from(mut self, value: impl Into<String>) -> Self {
        self.input_mut().created_from = runtime::InputValue::Value(value.into());
        self
    }
    pub fn created_to(mut self, value: impl Into<String>) -> Self {
        self.input_mut().created_to = runtime::InputValue::Value(value.into());
        self
    }
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn user(mut self, value: i64) -> Self {
        self.input_mut().user = runtime::InputValue::Value(value);
        self
    }
}
