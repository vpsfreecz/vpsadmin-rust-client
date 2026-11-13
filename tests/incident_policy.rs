use serde::Serialize;
use serde_json::json;
use vpsadmin_client::{
    Client,
    resources::{
        incident_policy, incident_report__policy_evaluation as history,
        vps::{index, show, update},
        vps__incident_policy as policy,
    },
    runtime::{self, ActionSpec, Error, InputValue, RawResponse, RequestParts},
};

fn request(spec: &ActionSpec, input: &impl Serialize) -> Result<runtime::PreparedRequest, Error> {
    let client = Client::builder("https://api.example.test").build()?;
    let mut parts = RequestParts {
        input: Some(serde_json::to_value(input).unwrap()),
        ..Default::default()
    };
    for (name, value) in [
        ("vps_id", "1"),
        ("key", "memory_pressure2"),
        ("incident_report_id", "12"),
    ] {
        if spec.path.contains(&format!("{{{name}}}")) {
            parts.path_params.insert(name.into(), value.into());
        }
    }
    runtime::prepare(client.config(), spec, parts)
}

#[test]
fn generic_registry_and_explicit_filters_keep_runtime_string_keys() {
    for prepared in [
        request(&index::SPEC, &index::Input::default()).unwrap(),
        request(&show::SPEC, &show::Input::default()).unwrap(),
    ] {
        assert_eq!(prepared.url.query(), None);
    }
    let selected = index::Input {
        incident_policy: InputValue::Value("memory_pressure2".into()),
        incident_status: InputValue::Value("near_limit".into()),
        incident_enforcement_enabled: InputValue::Value(false),
        ..Default::default()
    };
    let prepared = request(&index::SPEC, &selected).unwrap();
    let query: std::collections::BTreeMap<_, _> = prepared.url.query_pairs().collect();
    assert_eq!(
        query.get("vps[incident_policy]").map(|s| s.as_ref()),
        Some("memory_pressure2")
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
    let registry = runtime::decode::<
        Vec<incident_policy::index::Output>,
        incident_policy::index::Meta,
    >(
        &incident_policy::index::SPEC,
        RawResponse {
            status: 200,
            body:
                br#"{"status":true,"response":{"incident_policies":[{"key":"memory_pressure2"}]}}"#
                    .to_vec(),
        },
    )
    .unwrap();
    assert_eq!(registry.output[0].key.as_deref(), Some("memory_pressure2"));
    let client = Client::builder("https://api.example.test").build().unwrap();
    let _ = client.incident_policy().show("memory_pressure2");
    let _ = client.vps__incident_policy(1).index();
    let _ = client.vps__incident_policy(1).show("memory_pressure2");
}

#[test]
fn policy_update_preserves_null_and_returns_authoritative_row() {
    let client = Client::builder("https://api.example.test").build().unwrap();
    let _ = client
        .vps__incident_policy(1)
        .update("memory_pressure2")
        .warning_limit_override_null()
        .stop_limit_override(12);
    let input = policy::update::Input {
        warning_limit_override: InputValue::Null,
        stop_limit_override: InputValue::Value(12),
    };
    let prepared = request(&policy::update::SPEC, &input).unwrap();
    assert_eq!(
        prepared.url.path(),
        "/v7.0/vpses/1/incident_policies/memory_pressure2"
    );
    let body: serde_json::Value = serde_json::from_slice(&prepared.body.unwrap()).unwrap();
    assert_eq!(
        body,
        json!({"incident_policy":{"warning_limit_override":null,"stop_limit_override":12}})
    );
    let response=runtime::decode::<policy::update::Output,policy::update::Meta>(&policy::update::SPEC,RawResponse{status:200,body:br#"{"status":true,"response":{"incident_policy":{"key":"memory_pressure2","configuration_valid":true,"warning_limit":5,"stop_limit":12,"stop_limit_override":12}}}"#.to_vec()}).unwrap();
    assert_eq!(response.output.key.as_deref(), Some("memory_pressure2"));
    assert_eq!(response.output.configuration_valid, Some(true));
    assert_eq!(response.output.warning_limit, Some(5));
    assert_eq!(response.output.stop_limit, Some(12));
    let reset = policy::update::Input {
        stop_limit_override: InputValue::Null,
        ..Default::default()
    };
    let body: serde_json::Value = serde_json::from_slice(
        &request(&policy::update::SPEC, &reset)
            .unwrap()
            .body
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        body,
        json!({"incident_policy":{"stop_limit_override":null}})
    );
}

#[test]
fn shared_vps_permission_and_counters_stay_separate() {
    let input = update::Input {
        incident_enforcement_enabled: InputValue::Value(false),
        ..Default::default()
    };
    let body: serde_json::Value =
        serde_json::from_slice(&request(&update::SPEC, &input).unwrap().body.unwrap()).unwrap();
    assert_eq!(body, json!({"vps":{"incident_enforcement_enabled":false}}));
    let invalid = update::Input {
        incident_enforcement_enabled: InputValue::Null,
        ..Default::default()
    };
    assert!(matches!(
        request(&update::SPEC, &invalid),
        Err(Error::Validation { .. })
    ));
    let ack = runtime::decode::<update::Output, update::Meta>(
        &update::SPEC,
        RawResponse {
            status: 200,
            body: br#"{"status":true,"response":{}}"#.to_vec(),
        },
    )
    .unwrap();
    assert_eq!(ack.meta.action_state_id, None);
    let out:show::Output=serde_json::from_value(json!({"incident_enforcement_enabled":false,"incident_warning_count":2,"incident_stop_threshold_count":1})).unwrap();
    assert_eq!(out.incident_enforcement_enabled, Some(false));
    assert_eq!(out.incident_warning_count, Some(2));
    assert_eq!(out.incident_stop_threshold_count, Some(1));
}

#[test]
fn invalid_current_configuration_and_saved_evaluations_preserve_nullable_facts() {
    let row = json!({"key":"memory_pressure2","configuration_valid":false,"warning_limit":null,"stop_limit":null,"status":null,"count_7d":2,"count_30d":3,"evaluation_report_id":12,"evaluated_at":"2026-11-18T12:00:00Z","evaluation_state":"recorded"});
    let out: policy::show::Output = serde_json::from_value(row.clone()).unwrap();
    assert_eq!(out.configuration_valid, Some(false));
    assert_eq!(
        (out.warning_limit, out.stop_limit, out.status),
        (None, None, None)
    );
    assert_eq!((out.count_7d, out.count_30d), (Some(2), Some(3)));
    assert_eq!(out.evaluation_report_id, Some(12));
    assert_eq!(out.evaluated_at.as_deref(), Some("2026-11-18T12:00:00Z"));
    let page = runtime::decode::<Vec<policy::index::Output>, policy::index::Meta>(
        &policy::index::SPEC,
        RawResponse {
            status: 200,
            body: serde_json::to_vec(
                &json!({"status":true,"response":{"incident_policies":[row]}}),
            )
            .unwrap(),
        },
    )
    .unwrap();
    assert_eq!(page.output[0].warning_limit, None);
    let saved=runtime::decode::<Vec<history::index::Output>,history::index::Meta>(&history::index::SPEC,RawResponse{status:200,body:br#"{"status":true,"response":{"policy_evaluations":[{"policy_key":"memory_pressure2","state":"recorded","count_7d":5,"warning_limit":5,"stop_limit":10}]}}"#.to_vec()}).unwrap();
    assert_eq!(
        saved.output[0].policy_key.as_deref(),
        Some("memory_pressure2")
    );
    assert_eq!(
        (saved.output[0].warning_limit, saved.output[0].stop_limit),
        (Some(5), Some(10))
    );
    let client = Client::builder("https://api.example.test").build().unwrap();
    let _ = client.incident_report__policy_evaluation(12).index();
    let refused = runtime::decode::<policy::show::Output, policy::show::Meta>(
        &policy::show::SPEC,
        RawResponse {
            status: 404,
            body: br#"{"status":false,"message":"Unknown policy"}"#.to_vec(),
        },
    );
    assert!(matches!(refused, Err(Error::Api { status: 404, .. })));
}

#[test]
fn nullable_notice_metadata_preserves_classification_and_transport_states() {
    use vpsadmin_client::resources::incident_report;
    let ordinary = json!({"notice_kind":null,"trigger_incident_report_id":null,"notice_mail_state":null,"notice_policy_key":null,"notice_policy_label":null});
    let out: incident_report::show::Output = serde_json::from_value(ordinary.clone()).unwrap();
    assert_eq!(
        (
            out.notice_kind,
            out.trigger_incident_report_id,
            out.notice_mail_state
        ),
        (None, None, None)
    );
    let page =
        runtime::decode::<Vec<incident_report::index::Output>, incident_report::index::Meta>(
            &incident_report::index::SPEC,
            RawResponse {
                status: 200,
                body: serde_json::to_vec(
                    &json!({"status":true,"response":{"incident_reports":[ordinary]}}),
                )
                .unwrap(),
            },
        )
        .unwrap();
    assert_eq!(page.output[0].notice_kind, None);
    for kind in ["warning", "stop_completed"] {
        for state in [
            "pending",
            "queued",
            "sent",
            "failed",
            "uncertain",
            "suppressed",
        ] {
            let out = runtime::decode::<incident_report::show::Output,incident_report::show::Meta>(&incident_report::show::SPEC,RawResponse{status:200,body:serde_json::to_vec(&json!({"status":true,"response":{"incident_report":{"notice_kind":kind,"trigger_incident_report_id":100,"notice_mail_state":state,"notice_policy_key":"retired_policy","notice_policy_label":"Saved label"}}})).unwrap()}).unwrap().output;
            assert_eq!(out.notice_kind.as_deref(), Some(kind));
            assert_eq!(out.trigger_incident_report_id, Some(100));
            assert_eq!(out.notice_mail_state.as_deref(), Some(state));
            assert_eq!(out.notice_policy_key.as_deref(), Some("retired_policy"));
            assert_eq!(out.notice_policy_label.as_deref(), Some("Saved label"));
        }
    }
}

#[test]
fn nullable_custom_global_override_reset_remains_null() {
    use vpsadmin_client::resources::system_config;
    let client = Client::builder("https://api.example.test").build().unwrap();
    let input = system_config::update::Input {
        value: InputValue::Null,
    };
    let parts = RequestParts {
        path_params: [
            ("category".into(), "test".into()),
            ("name".into(), "limit".into()),
        ]
        .into_iter()
        .collect(),
        input: Some(serde_json::to_value(input).unwrap()),
        ..Default::default()
    };
    let prepared = runtime::prepare(client.config(), &system_config::update::SPEC, parts).unwrap();
    let body: serde_json::Value = serde_json::from_slice(&prepared.body.unwrap()).unwrap();
    assert_eq!(body, json!({"system_config":{"value":null}}));
}

#[test]
fn saved_membership_preserves_true_false_null_missing_and_periods() {
    for matched in [Some(true), Some(false), None] {
        let out: history::index::Output = serde_json::from_value(json!({
            "matched": matched, "warning_period_days": 7, "stop_period_days": 30
        }))
        .unwrap();
        assert_eq!(out.matched, matched);
        assert_eq!(
            (out.warning_period_days, out.stop_period_days),
            (Some(7), Some(30))
        );
    }
    let missing: history::index::Output = serde_json::from_value(json!({})).unwrap();
    assert_eq!(missing.matched, None);
    assert_eq!(
        (missing.warning_period_days, missing.stop_period_days),
        (None, None)
    );
    let ordinary: vpsadmin_client::resources::incident_report::show::Output =
        serde_json::from_value(json!({})).unwrap();
    assert_eq!(
        (ordinary.notice_policy_key, ordinary.notice_policy_label),
        (None, None)
    );
}
