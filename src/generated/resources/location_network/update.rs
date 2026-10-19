use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(
        rename = "autopick",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub autopick: runtime::InputValue<bool>,
    #[serde(
        rename = "primary",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub primary: runtime::InputValue<bool>,
    #[serde(
        rename = "priority",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub priority: runtime::InputValue<i64>,
    #[serde(
        rename = "userpick",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub userpick: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.autopick)?;
        runtime::Validate::validate(&self.primary)?;
        runtime::Validate::validate(&self.priority)?;
        runtime::Validate::validate(&self.userpick)?;
        Ok(())
    }
}
#[derive(Default, serde::Deserialize)]
#[non_exhaustive]
pub struct Output {
    #[serde(default, rename = "autopick")]
    pub autopick: Option<bool>,
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[doc = "Association keys: ID=id, label=label"]
    #[serde(default, rename = "location")]
    pub location: Option<runtime::Association>,
    #[doc = "Association keys: ID=id, label=address"]
    #[serde(default, rename = "network")]
    pub network: Option<runtime::Association>,
    #[serde(default, rename = "primary")]
    pub primary: Option<bool>,
    #[serde(default, rename = "priority")]
    pub priority: Option<i64>,
    #[serde(default, rename = "userpick")]
    pub userpick: Option<bool>,
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
    name: "location_network/update",
    method: runtime::Method::Put,
    path: "v7.0/location_networks/{location_network_id}",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("location_network"),
        parameters: &[
            runtime::ParameterSpec {
                name: "autopick",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "primary",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "priority",
                kind: runtime::ParameterKind::Integer,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "userpick",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "location_network",
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
    pub fn autopick(mut self, value: bool) -> Self {
        self.input_mut().autopick = runtime::InputValue::Value(value);
        self
    }
    pub fn primary(mut self, value: bool) -> Self {
        self.input_mut().primary = runtime::InputValue::Value(value);
        self
    }
    pub fn priority(mut self, value: i64) -> Self {
        self.input_mut().priority = runtime::InputValue::Value(value);
        self
    }
    pub fn userpick(mut self, value: bool) -> Self {
        self.input_mut().userpick = runtime::InputValue::Value(value);
        self
    }
}
