use vpsadmin_client::resources::{node__kernel_history, node_kernel_event};

#[test]
fn livepatch_id_outputs_accept_missing_null_and_string_values() {
    let history_missing: node__kernel_history::index::Output = serde_json::from_str("{}").unwrap();
    let history_null: node__kernel_history::index::Output =
        serde_json::from_str(r#"{"livepatch_id":null}"#).unwrap();
    let history_value: node__kernel_history::index::Output =
        serde_json::from_str(r#"{"livepatch_id":"lp_history"}"#).unwrap();
    assert_eq!(history_missing.livepatch_id, None);
    assert_eq!(history_null.livepatch_id, None);
    assert_eq!(history_value.livepatch_id.as_deref(), Some("lp_history"));

    let event_missing: node_kernel_event::index::Output = serde_json::from_str("{}").unwrap();
    let event_null: node_kernel_event::index::Output =
        serde_json::from_str(r#"{"livepatch_id":null}"#).unwrap();
    let event_value: node_kernel_event::index::Output =
        serde_json::from_str(r#"{"livepatch_id":"lp_event"}"#).unwrap();
    assert_eq!(event_missing.livepatch_id, None);
    assert_eq!(event_null.livepatch_id, None);
    assert_eq!(event_value.livepatch_id.as_deref(), Some("lp_event"));

    let show_missing: node_kernel_event::show::Output = serde_json::from_str("{}").unwrap();
    let show_null: node_kernel_event::show::Output =
        serde_json::from_str(r#"{"livepatch_id":null}"#).unwrap();
    let show_value: node_kernel_event::show::Output =
        serde_json::from_str(r#"{"livepatch_id":"lp_show"}"#).unwrap();
    assert_eq!(show_missing.livepatch_id, None);
    assert_eq!(show_null.livepatch_id, None);
    assert_eq!(show_value.livepatch_id.as_deref(), Some("lp_show"));
}
