use serde_json::json;
use vpsadmin_client::{
    Client,
    resources::{
        incident_report__evidence_interval::index as saved, vps__evidence::index as child,
        vps_evidence::index as fleet,
    },
    runtime::{self, Error, InputValue, RawResponse, RequestParts},
};

#[test]
fn evidence_routes_and_explicit_pagination_use_final_namespaces() {
    let client = Client::builder("https://api.example.test/mount")
        .build()
        .unwrap();
    let _ = client
        .incident_report__evidence_interval(12)
        .index()
        .offset(0)
        .limit(100);
    let _ = client.vps__evidence(12).index();
    let _ = client
        .vps_evidence()
        .index()
        .offset(0)
        .limit(100)
        .signal("rtnl_latency")
        .vps(12);
    for (spec, input, path_key, path, namespace) in [
        (
            &saved::SPEC,
            json!({"offset":0,"limit":100}),
            Some("incident_report_id"),
            "/mount/v7.0/incident_reports/12/evidence_intervals",
            "evidence_interval",
        ),
        (
            &child::SPEC,
            json!({}),
            Some("vps_id"),
            "/mount/v7.0/vpses/12/evidence",
            "evidence",
        ),
        (
            &fleet::SPEC,
            json!({"offset":0,"limit":100}),
            None,
            "/mount/v7.0/vps_evidence",
            "vps_evidence",
        ),
    ] {
        let mut parts = RequestParts {
            input: Some(input),
            ..Default::default()
        };
        if let Some(key) = path_key {
            parts.path_params.insert(key.into(), "12".into());
        }
        let request = runtime::prepare(client.config(), spec, parts).unwrap();
        assert_eq!(request.url.path(), path);
        assert!(matches!(spec.method, runtime::Method::Get));
        assert!(matches!(spec.auth, runtime::RequestAuth::Configured));
        let query: std::collections::BTreeMap<_, _> = request.url.query_pairs().collect();
        if path_key == Some("vps_id") {
            assert!(query.is_empty());
        } else {
            assert_eq!(query.len(), 2);
            assert_eq!(query[&*format!("{namespace}[offset]")], "0");
            assert_eq!(query[&*format!("{namespace}[limit]")], "100");
        }
    }
    let selected = fleet::Input {
        offset: InputValue::Value(0),
        limit: InputValue::Value(100),
        signal: InputValue::Value("rtnl_latency".into()),
        vps: InputValue::Value(12),
        ..Default::default()
    };
    let request = runtime::prepare(
        client.config(),
        &fleet::SPEC,
        RequestParts {
            input: Some(serde_json::to_value(selected).unwrap()),
            ..Default::default()
        },
    )
    .unwrap();
    let query: std::collections::BTreeMap<_, _> = request.url.query_pairs().collect();
    assert_eq!(query["vps_evidence[vps]"], "12");
    assert_eq!(query["vps_evidence[signal]"], "rtnl_latency");
}

#[test]
fn live_evidence_keeps_valid_config_unavailable_distinct_from_recorded_zero() {
    for credit in [None, Some(0), Some(125)] {
        let peak = if credit.is_some() {
            json!(0)
        } else {
            json!(null)
        };
        let lower = if credit.is_some() {
            json!(false)
        } else {
            json!(null)
        };
        let body = json!({"status":true,"response":{"evidence":[{"configuration_valid":true,"credit_us":credit,"peak_value":peak,"peak_is_lower_bound":lower,"condition_value":null}],"_meta":{"configuration_valid":true,"unavailable_signals":[]}}});
        let page = runtime::decode::<Vec<child::Output>, child::Meta>(
            &child::SPEC,
            RawResponse {
                status: 200,
                body: serde_json::to_vec(&body).unwrap(),
            },
        )
        .unwrap();
        assert_eq!(page.output[0].credit_us, credit);
        assert_eq!(page.output[0].configuration_valid, Some(true));
        assert_eq!(page.output[0].peak_value, credit.map(|_| 0));
        assert_eq!(page.output[0].peak_is_lower_bound, credit.map(|_| false));
        assert_eq!(page.output[0].condition_value, None);
        assert_eq!(page.meta.unavailable_signals, Some(json!([])));
    }
    let page=runtime::decode::<Vec<fleet::Output>,fleet::Meta>(&fleet::SPEC,RawResponse{status:200,body:serde_json::to_vec(&json!({"status":true,"response":{"vps_evidence":[],"_meta":{"configuration_valid":false,"unavailable_signals":["rtnl_latency"],"total_count":0,"next_offset":null}}})).unwrap()}).unwrap();
    let signals = page
        .meta
        .unavailable_signals
        .as_ref()
        .unwrap()
        .as_array()
        .unwrap();
    assert!(signals.iter().all(serde_json::Value::is_string));
    assert_eq!(signals, &vec![json!("rtnl_latency")]);
    assert_eq!(page.meta.next_offset, None);
    assert_eq!(page.meta.configuration_valid, Some(false));
    let page=runtime::decode::<Vec<fleet::Output>,fleet::Meta>(&fleet::SPEC,RawResponse{status:200,body:serde_json::to_vec(&json!({"status":true,"response":{"vps_evidence":[{"credit_us":125}],"_meta":{"configuration_valid":true,"unavailable_signals":[],"total_count":2,"next_offset":1}}})).unwrap()}).unwrap();
    assert_eq!(page.output[0].credit_us, Some(125));
    assert_eq!(page.meta.total_count, Some(2));
    assert_eq!(page.meta.next_offset, Some(1));
}

#[test]
fn saved_generic_intervals_preserve_precision_nullable_peaks_and_condition() {
    let stamp = "2026-11-25T12:00:00.123456Z";
    let body = json!({"status":true,"response":{"evidence_intervals":[{"id":3,"credit_us":1530865,"from_at":stamp,"to_at":"2026-11-25T12:00:01.654321Z","measurement":null,"peak_unit":null,"peak_value":null,"peak_is_lower_bound":null}],"_meta":{"available":true,"window_complete":false,"condition_measurement":"rtnl_latency","condition_operator":"gt","condition_value":1000000,"condition_unit":"microseconds","next_offset":1,"total_count":2}}});
    let page = runtime::decode::<Vec<saved::Output>, saved::Meta>(
        &saved::SPEC,
        RawResponse {
            status: 200,
            body: serde_json::to_vec(&body).unwrap(),
        },
    )
    .unwrap();
    let row = &page.output[0];
    assert_eq!(row.from_at.as_deref(), Some(stamp));
    assert_eq!(row.to_at.as_deref(), Some("2026-11-25T12:00:01.654321Z"));
    assert_eq!(row.credit_us, Some(1530865));
    assert_eq!(
        (
            &row.measurement,
            &row.peak_unit,
            row.peak_value,
            row.peak_is_lower_bound
        ),
        (&None, &None, None, None)
    );
    assert_eq!(page.meta.condition_value, Some(1000000));
    assert_eq!(page.meta.condition_operator.as_deref(), Some("gt"));
    assert_eq!(page.meta.window_complete, Some(false));
    let empty=runtime::decode::<Vec<saved::Output>,saved::Meta>(&saved::SPEC,RawResponse{status:200,body:br#"{"status":true,"response":{"evidence_intervals":[],"_meta":{"available":false,"condition_value":null,"next_offset":null}}}"#.to_vec()}).unwrap();
    assert!(empty.output.is_empty());
    assert_eq!(empty.meta.available, Some(false));
    assert_eq!(empty.meta.condition_value, None);
    let denied = runtime::decode::<Vec<child::Output>, child::Meta>(
        &child::SPEC,
        RawResponse {
            status: 403,
            body: br#"{"status":false,"message":"denied"}"#.to_vec(),
        },
    );
    assert!(matches!(denied, Err(Error::Api { status: 403, .. })));
}
