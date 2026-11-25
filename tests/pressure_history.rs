use serde_json::json;
use vpsadmin_client::{
    Client,
    resources::{vps_evidence::index as fleet, vps_pressure_history::index as history},
    runtime::{self, InputValue, RawResponse, RequestParts},
};

#[test]
fn history_and_filters_keep_routes_query_namespaces_and_required_vps() {
    let client = Client::builder("https://api.example.test/mount")
        .build()
        .unwrap();
    let _ = client.vps_pressure_history().index().vps(12).days(30);
    let _ = client
        .vps_evidence()
        .index()
        .user(1)
        .node(2)
        .location(3)
        .environment(4);
    for (spec, input, expected_path, expected_query) in [
        (
            &history::SPEC,
            json!({"vps":12}),
            "/mount/v7.0/vps_pressure_history",
            vec![("vps_pressure_history[vps]", "12")],
        ),
        (
            &history::SPEC,
            json!({"vps":12,"days":30}),
            "/mount/v7.0/vps_pressure_history",
            vec![
                ("vps_pressure_history[days]", "30"),
                ("vps_pressure_history[vps]", "12"),
            ],
        ),
        (
            &fleet::SPEC,
            json!({"user":1,"node":2,"location":3,"environment":4}),
            "/mount/v7.0/vps_evidence",
            vec![
                ("vps_evidence[environment]", "4"),
                ("vps_evidence[location]", "3"),
                ("vps_evidence[node]", "2"),
                ("vps_evidence[user]", "1"),
            ],
        ),
    ] {
        let request = runtime::prepare(
            client.config(),
            spec,
            RequestParts {
                input: Some(input),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(request.url.path(), expected_path);
        assert!(request.body.is_none());
        assert_eq!(
            request
                .url
                .query_pairs()
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect::<std::collections::BTreeMap<String, String>>(),
            expected_query
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<std::collections::BTreeMap<String, String>>()
        );
    }
    let missing = runtime::prepare(
        client.config(),
        &history::SPEC,
        RequestParts {
            input: Some(json!({})),
            ..Default::default()
        },
    );
    assert!(missing.is_err());
    let input = history::Input {
        vps: InputValue::Value(12),
        ..Default::default()
    };
    assert_eq!(serde_json::to_value(input).unwrap(), json!({"vps":12}));
}

#[test]
fn history_calendar_nulls_microseconds_and_metadata_survive_decode() {
    for value in [None, Some(125)] {
        let body = json!({"status":true,"response":{"vps_pressure_history":[
            {"id":"2026-11-27","from_at":"2026-11-27T00:00:00.000000Z","to_at":"2026-11-28T00:00:00.000000Z","recorded_us":null,"partial_day":false},
            {"id":"2026-11-28","from_at":"2026-11-28T00:00:00.000000Z","to_at":"2026-11-29T00:00:00.000000Z","recorded_us":value,"partial_day":true}
        ],"_meta":{"available":true,"accounting_since":null,"ownership_limited":false,"timezone":"UTC","read_at":"2026-11-28T12:00:00.123456Z","range_from":"2026-11-22T00:00:00.000000Z","range_to":"2026-11-29T00:00:00.000000Z","first_recorded_at":null,"total_recorded_us":value}}});
        let page = runtime::decode::<Vec<history::Output>, history::Meta>(
            &history::SPEC,
            RawResponse {
                status: 200,
                body: serde_json::to_vec(&body).unwrap(),
            },
        )
        .unwrap();
        assert_eq!(page.output[0].recorded_us, None);
        assert_eq!(page.output[0].partial_day, Some(false));
        assert_eq!(page.output[1].recorded_us, value);
        assert_eq!(page.output[1].partial_day, Some(true));
        assert_eq!(
            page.output[1].to_at.as_deref(),
            Some("2026-11-29T00:00:00.000000Z")
        );
        assert_eq!(page.meta.total_recorded_us, value);
        assert_eq!(page.meta.accounting_since, None);
        assert_eq!(page.meta.first_recorded_at, None);
        assert_eq!(page.meta.available, Some(true));
        assert_eq!(page.meta.ownership_limited, Some(false));
        assert_eq!(
            page.meta.read_at.as_deref(),
            Some("2026-11-28T12:00:00.123456Z")
        );
    }
}
