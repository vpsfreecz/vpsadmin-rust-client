use crate::runtime;

#[derive(Default, serde::Serialize)]
pub struct Input {
    #[serde(rename = "fuse", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub fuse: runtime::InputValue<bool>,
    #[serde(
        rename = "impermanence",
        skip_serializing_if = "runtime::InputValue::is_unset"
    )]
    pub impermanence: runtime::InputValue<bool>,
    #[serde(rename = "kvm", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub kvm: runtime::InputValue<bool>,
    #[serde(rename = "lxc", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub lxc: runtime::InputValue<bool>,
    #[serde(rename = "ppp", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub ppp: runtime::InputValue<bool>,
    #[serde(rename = "tun", skip_serializing_if = "runtime::InputValue::is_unset")]
    pub tun: runtime::InputValue<bool>,
}
impl runtime::Validate for Input {
    fn validate(&self) -> Result<(), runtime::Error> {
        runtime::Validate::validate(&self.fuse)?;
        runtime::Validate::validate(&self.impermanence)?;
        runtime::Validate::validate(&self.kvm)?;
        runtime::Validate::validate(&self.lxc)?;
        runtime::Validate::validate(&self.ppp)?;
        runtime::Validate::validate(&self.tun)?;
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
    name: "vps/feature/update_all",
    method: runtime::Method::Post,
    path: "v7.0/vpses/{vps_id}/features/update_all",
    input: Some(runtime::InputSpec {
        layout: runtime::Layout::Object,
        namespace: Some("feature"),
        parameters: &[
            runtime::ParameterSpec {
                name: "fuse",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "impermanence",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "kvm",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "lxc",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "ppp",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
            runtime::ParameterSpec {
                name: "tun",
                kind: runtime::ParameterKind::Boolean,
                required: false,
                nullable: false,
                protected: false,
            },
        ],
    }),
    output: Some(runtime::OutputSpec {
        layout: runtime::Layout::Object,
        namespace: "feature",
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
    pub fn fuse(mut self, value: bool) -> Self {
        self.input_mut().fuse = runtime::InputValue::Value(value);
        self
    }
    pub fn impermanence(mut self, value: bool) -> Self {
        self.input_mut().impermanence = runtime::InputValue::Value(value);
        self
    }
    pub fn kvm(mut self, value: bool) -> Self {
        self.input_mut().kvm = runtime::InputValue::Value(value);
        self
    }
    pub fn lxc(mut self, value: bool) -> Self {
        self.input_mut().lxc = runtime::InputValue::Value(value);
        self
    }
    pub fn ppp(mut self, value: bool) -> Self {
        self.input_mut().ppp = runtime::InputValue::Value(value);
        self
    }
    pub fn tun(mut self, value: bool) -> Self {
        self.input_mut().tun = runtime::InputValue::Value(value);
        self
    }
}
