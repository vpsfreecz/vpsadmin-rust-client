use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "monthly_payment",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub monthly_payment: runtime::InputValue<i64>,
    #[serde(
        rename = "paid_until",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub paid_until: runtime::InputValue<String>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.monthly_payment)?;
        runtime::Validate::validate(&self.paid_until)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "monthly_payment")]
    pub monthly_payment: Option<i64>,
    #[serde(default, rename = "paid_until")]
    pub paid_until: Option<String>,
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
    name: "user_account/update",
    method: runtime::Method::Put,
    path: "v7.0/user_accounts/{user_account_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("user_account"),
        parameters: &[
            runtime::ParameterSpec {
                name: "monthly_payment",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "paid_until",
                kind: runtime::ParameterKind::Datetime,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "user_account",
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
    pub fn monthly_payment(mut self, value: i64) -> Self {
        self.input_mut().monthly_payment = runtime::InputValue::Value(value);
        self
    }
    pub fn paid_until(mut self, value: impl Into<String>) -> Self {
        self.input_mut().paid_until = runtime::InputValue::Value(value.into());
        self
    }
}
