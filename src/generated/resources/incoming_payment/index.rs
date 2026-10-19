use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
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
    #[serde(
        rename = "state",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub state: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.from_id)?;
        runtime::Validate::validate(&self.limit)?;
        runtime::Validate::validate(&self.state)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "account_name")]
    pub account_name: Option<String>,
    #[serde(default, rename = "amount")]
    pub amount: Option<i64>,
    #[serde(default, rename = "comment")]
    pub comment: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "currency")]
    pub currency: Option<String>,
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "ks")]
    pub ks: Option<String>,
    #[serde(default, rename = "src_amount")]
    pub src_amount: Option<i64>,
    #[serde(default, rename = "src_currency")]
    pub src_currency: Option<String>,
    #[serde(default, rename = "ss")]
    pub ss: Option<String>,
    #[serde(default, rename = "state")]
    pub state: Option<String>,
    #[serde(default, rename = "transaction_id")]
    pub transaction_id: Option<String>,
    #[serde(default, rename = "transaction_type")]
    pub transaction_type: Option<String>,
    #[serde(default, rename = "user_ident")]
    pub user_ident: Option<String>,
    #[serde(default, rename = "user_message")]
    pub user_message: Option<String>,
    #[serde(default, rename = "vs")]
    pub vs: Option<String>,
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
    name: "incoming_payment/index",
    method: runtime::Method::Get,
    path: "v7.0/incoming_payments",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("incoming_payment"),
        parameters: &[
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
                name: "state",
                kind: runtime::ParameterKind::String,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::ObjectList,
        namespace: "incoming_payments",
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
    pub fn from_id(mut self, value: i64) -> Self {
        self.input_mut().from_id = runtime::InputValue::Value(value);
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.input_mut().limit = runtime::InputValue::Value(value);
        self
    }
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.input_mut().state = runtime::InputValue::Value(value.into());
        self
    }
}
