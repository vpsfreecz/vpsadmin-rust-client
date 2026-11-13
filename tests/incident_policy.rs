use serde::Serialize;
use serde_json::json;
use vpsadmin_client::{
    Client,
    resources::vps::{index, show, update},
    runtime::{self, ActionSpec, Error, InputValue, RawResponse, RequestParts},
};

fn request(spec: &ActionSpec, input: &impl Serialize) -> Result<runtime::PreparedRequest, Error> {
    let client = Client::builder("https://api.example.test").build()?;
    let mut parts = RequestParts {
        input: Some(serde_json::to_value(input).unwrap()),
        ..Default::default()
    };
    if spec.path.contains("{vps_id}") {
        parts.path_params.insert("vps_id".into(), "1".into());
    }
    runtime::prepare(client.config(), spec, parts)
}

#[test]
fn incident_read_defaults_are_not_injected() {
    for prepared in [
        request(&index::SPEC, &index::Input::default()).unwrap(),
        request(&show::SPEC, &show::Input::default()).unwrap(),
    ] {
        assert_eq!(prepared.url.query(), None);
    }
    let selected = index::Input {
        incident_policy: InputValue::Value("oomd".into()),
        incident_status: InputValue::Value("near_limit".into()),
        incident_enforcement_enabled: InputValue::Value(false),
        ..Default::default()
    };
    let prepared = request(&index::SPEC, &selected).unwrap();
    let query: std::collections::BTreeMap<_, _> = prepared.url.query_pairs().collect();
    assert_eq!(
        query.get("vps[incident_policy]").map(|s| s.as_ref()),
        Some("oomd")
    );
    assert_eq!(
        query.get("vps[incident_status]").map(|s| s.as_ref()),
        Some("near_limit")
    );
    assert_eq!(
        query
            .get("vps[incident_enforcement_enabled]")
            .map(|s| s.as_ref()),
        Some("false")
    );
    let invalid = show::Input {
        incident_policy: InputValue::Null,
    };
    assert!(matches!(
        request(&show::SPEC, &invalid),
        Err(Error::Validation { .. })
    ));
}

#[test]
fn incident_settings_preserve_explicit_null_and_false() {
    let client = Client::builder("https://api.example.test").build().unwrap();
    let _ = client
        .vps()
        .index()
        .incident_policy("oomd")
        .incident_status("near_limit")
        .incident_enforcement_enabled(false);
    let _ = client.vps().show(1).incident_policy("oomd");
    let _ = client
        .vps()
        .update(1)
        .incident_policy("oomd")
        .incident_warning_limit_override_null()
        .incident_stop_limit_override(12)
        .incident_enforcement_enabled(false);
    let _ = client
        .vps()
        .update(1)
        .incident_policy("oomd")
        .incident_stop_limit_override_null();
    let mut input = update::Input {
        incident_policy: InputValue::Value("oomd".into()),
        incident_warning_limit_override: InputValue::Null,
        incident_stop_limit_override: InputValue::Value(12),
        incident_enforcement_enabled: InputValue::Value(false),
        ..Default::default()
    };
    let prepared = request(&update::SPEC, &input).unwrap();
    let body: serde_json::Value = serde_json::from_slice(&prepared.body.unwrap()).unwrap();
    assert_eq!(
        body,
        json!({"vps": {"incident_policy":"oomd", "incident_warning_limit_override":null, "incident_stop_limit_override":12, "incident_enforcement_enabled":false}})
    );
    input.incident_stop_limit_override = InputValue::Null;
    let prepared = request(&update::SPEC, &input).unwrap();
    let body: serde_json::Value = serde_json::from_slice(&prepared.body.unwrap()).unwrap();
    assert_eq!(
        body["vps"]["incident_stop_limit_override"],
        serde_json::Value::Null
    );
    input.incident_enforcement_enabled = InputValue::Null;
    assert!(matches!(
        request(&update::SPEC, &input),
        Err(Error::Validation { .. })
    ));
    let plain = update::Input {
        hostname: InputValue::Value("ordinary-update".into()),
        ..Default::default()
    };
    let prepared = request(&update::SPEC, &plain).unwrap();
    let body: serde_json::Value = serde_json::from_slice(&prepared.body.unwrap()).unwrap();
    assert_eq!(body, json!({"vps": {"hostname":"ordinary-update"}}));
    let synchronous = runtime::decode::<update::Output, update::Meta>(
        &update::SPEC,
        RawResponse {
            status: 200,
            body: br#"{"status":true,"response":{}}"#.to_vec(),
        },
    )
    .unwrap();
    assert_eq!(synchronous.meta.action_state_id, None);
}

#[test]
fn incident_summaries_decode_optional_members_and_server_refusals() {
    let summary = json!({
        "incident_policy":"oomd", "incident_policy_label":"VPS memory-pressure incidents",
        "incident_count_7d":5, "incident_count_30d":10,
        "incident_warning_limit":5, "incident_stop_limit":10,
        "incident_warning_limit_override":null, "incident_stop_limit_override":12,
        "incident_enforcement_enabled":false, "incident_mode":"enforce",
        "incident_stop_enforced":false, "incident_status":"stop_threshold",
        "incident_last_incident_at":"2026-11-14T10:00:00Z", "incident_last_warning_at":null,
        "incident_last_stop_at":null, "incident_next_stop_allowed_at":"2026-11-15T10:00:00Z",
        "incident_accounting_since":"2026-11-01T00:00:00Z",
        "incident_action_state":"pending", "incident_action_chain_id":42
    });
    let decoded = runtime::decode::<show::Output, show::Meta>(
        &show::SPEC,
        RawResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"status":true,"response":{"vps":summary}})).unwrap(),
        },
    )
    .unwrap()
    .output;
    assert_eq!(
        (
            decoded.incident_count_7d,
            decoded.incident_count_30d,
            decoded.incident_warning_limit,
            decoded.incident_stop_limit
        ),
        (Some(5), Some(10), Some(5), Some(10))
    );
    assert_eq!(
        (
            decoded.incident_warning_limit_override,
            decoded.incident_stop_limit_override
        ),
        (None, Some(12))
    );
    assert_eq!(
        (
            decoded.incident_enforcement_enabled,
            decoded.incident_stop_enforced
        ),
        (Some(false), Some(false))
    );
    assert_eq!(decoded.incident_policy.as_deref(), Some("oomd"));
    assert_eq!(
        decoded.incident_policy_label.as_deref(),
        Some("VPS memory-pressure incidents")
    );
    assert_eq!(decoded.incident_mode.as_deref(), Some("enforce"));
    assert_eq!(decoded.incident_status.as_deref(), Some("stop_threshold"));
    assert_eq!(decoded.incident_action_state.as_deref(), Some("pending"));
    assert_eq!(decoded.incident_action_chain_id, Some(42));
    assert_eq!(
        decoded.incident_last_incident_at.as_deref(),
        Some("2026-11-14T10:00:00Z")
    );
    assert_eq!(
        decoded.incident_next_stop_allowed_at.as_deref(),
        Some("2026-11-15T10:00:00Z")
    );
    assert_eq!(
        decoded.incident_accounting_since.as_deref(),
        Some("2026-11-01T00:00:00Z")
    );
    assert_eq!(
        (
            decoded.incident_last_warning_at,
            decoded.incident_last_stop_at
        ),
        (None, None)
    );
    let page = runtime::decode::<Vec<index::Output>, index::Meta>(
        &index::SPEC,
        RawResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"status":true,"response":{"vpses":[summary]}}))
                .unwrap(),
        },
    )
    .unwrap();
    assert_eq!(page.output[0].incident_count_7d, Some(5));
    let old_api: show::Output = serde_json::from_value(json!({"id":1})).unwrap();
    assert_eq!(old_api.incident_policy, None);
    assert_eq!(old_api.incident_count_7d, None);
    let unknown = show::Input {
        incident_policy: InputValue::Value("unregistered".into()),
    };
    assert!(
        request(&show::SPEC, &unknown).is_ok(),
        "registry validation remains on the server"
    );
    let refused = runtime::decode::<show::Output, show::Meta>(&show::SPEC, RawResponse { status:400, body:br#"{"status":false,"message":"Invalid incident policy","errors":{"incident_policy":["is not included"]}}"#.to_vec() });
    match refused {
        Err(Error::Api {
            status: 400,
            message,
            ..
        }) => assert_eq!(message.as_deref(), Some("Invalid incident policy")),
        _ => panic!("server refusal was lost"),
    }
}
