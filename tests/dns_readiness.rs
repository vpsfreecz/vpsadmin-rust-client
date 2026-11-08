use serde_json::json;
use vpsadmin_client::resources::{
    dns_server_zone, dns_server_zone_primary_transfer_state, dns_server_zone_transfer_log,
    dns_zone_transfer,
};

#[test]
fn readiness_preserves_stale_positive_and_current_failure_evidence() {
    let stale: dns_server_zone_primary_transfer_state::index::Output =
        serde_json::from_value(json!({
            "status": "success", "evidence_current": false,
            "last_attempt_kind": "ixfr_probe", "primary_serial": 100, "secondary_serial": 100
        }))
        .unwrap();
    assert_eq!(stale.status.as_deref(), Some("success"));
    assert_eq!(stale.evidence_current, Some(false));
    assert_eq!(stale.last_attempt_kind.as_deref(), Some("ixfr_probe"));

    let failed: dns_server_zone_primary_transfer_state::index::Output =
        serde_json::from_value(json!({
            "status": "failed", "evidence_current": true,
            "failure_class": "network", "reason_code": "timeout"
        }))
        .unwrap();
    assert_eq!(failed.evidence_current, Some(true));
    assert_eq!(failed.failure_class.as_deref(), Some("network"));
    assert_eq!(failed.reason_code.as_deref(), Some("timeout"));
}

#[test]
fn historical_and_old_api_outputs_do_not_invent_readiness() {
    let log: dns_server_zone_transfer_log::show::Output = serde_json::from_value(json!({
        "attempt_kind": "legacy", "status": "failed",
        "dns_zone_transfer": null, "primary_serial": null
    }))
    .unwrap();
    assert_eq!(log.attempt_kind.as_deref(), Some("legacy"));
    assert!(log.dns_zone_transfer.is_none());
    assert!(log.primary_serial.is_none());

    let old: dns_zone_transfer::show::Output =
        serde_json::from_value(json!({"id": 20, "peer_type": "primary_type"})).unwrap();
    assert_eq!(old.id, Some(20));
    assert!(old.transfer_check_status.is_none());
    assert!(old.last_transfer_check_at.is_none());

    let serving: dns_server_zone::show::Output = serde_json::from_value(json!({
        "zone_status": "serving", "last_transfer_status": "failed"
    }))
    .unwrap();
    assert_eq!(serving.zone_status.as_deref(), Some("serving"));
    assert_eq!(serving.last_transfer_status.as_deref(), Some("failed"));
}
