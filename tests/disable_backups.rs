use serde_json::json;
use vpsadmin_client::{
    Client,
    resources::dataset::disable_backups,
    runtime::{self, InputValue, RawResponse, RequestParts},
};

fn body(input: disable_backups::Input) -> serde_json::Value {
    let client = Client::builder("https://api.example.test").build().unwrap();
    let prepared = runtime::prepare(
        client.config(),
        &disable_backups::SPEC,
        RequestParts {
            path_params: [("dataset_id".into(), "7".into())].into_iter().collect(),
            input: Some(serde_json::to_value(input).unwrap()),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(prepared.url.path(), "/v7.0/datasets/7/disable_backups");
    serde_json::from_slice(&prepared.body.unwrap()).unwrap()
}

#[test]
fn default_preservation_and_opt_out_keep_wire_intent() {
    assert_eq!(
        body(disable_backups::Input::default()),
        json!({"dataset": {}})
    );
    assert_eq!(
        body(disable_backups::Input {
            recursive: InputValue::Value(true),
            ..Default::default()
        }),
        json!({"dataset": {"recursive": true}})
    );
    assert_eq!(
        body(disable_backups::Input {
            backup_first: InputValue::Value(false),
            recursive: InputValue::Value(false),
            ..Default::default()
        }),
        json!({"dataset": {"backup_first": false, "recursive": false}})
    );
    let client = Client::builder("https://api.example.test").build().unwrap();
    let _ = client.dataset().disable_backups(7).recursive(true);
    let _ = client.dataset().disable_backups(7).backup_first(false);
}

#[test]
fn ordinary_acknowledgement_exposes_the_chain_receipt() {
    let decoded = runtime::decode::<disable_backups::Output, disable_backups::Meta>(
        &disable_backups::SPEC,
        RawResponse {
            status: 200,
            body: br#"{"status":true,"response":{"dataset":{},"_meta":{"action_state_id":41}}}"#
                .to_vec(),
        },
    )
    .unwrap();
    assert!(decoded.output.extra.is_empty());
    assert_eq!(decoded.meta.action_state_id, Some(41));
}
