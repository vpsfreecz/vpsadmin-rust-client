//! Acceptance against the disposable real vpsAdmin fixture, never production.

use serde::Deserialize;
use serde_json::json;
use vpsadmin_client::TokenStep;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Deserialize)]
struct Fixture {
    vps_id: i64,
    dns_record_id: i64,
    setting_namespace: String,
    setting_key: String,
}

fn fixture() -> Result<(String, Fixture), Box<dyn std::error::Error>> {
    if std::env::var("VPSADMIN_REAL_INTEGRATION").as_deref() != Ok("1") {
        return Err("set VPSADMIN_REAL_INTEGRATION=1 to run disposable server mutations".into());
    }
    let base = std::env::var("VPSADMIN_TEST_BASE_URL")?;
    let url = url::Url::parse(&base)?;
    let loopback = matches!(url.host(), Some(url::Host::Ipv4(ip)) if ip.is_loopback())
        || matches!(url.host(), Some(url::Host::Ipv6(ip)) if ip.is_loopback());
    if url.scheme() != "http"
        || !loopback
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("fixture tests require an explicit loopback HTTP URL".into());
    }
    let fixture = serde_json::from_str(&std::env::var("VPSADMIN_FIXTURE_JSON")?)?;
    Ok((base, fixture))
}

// The same source exercises both transports against real server behavior.
// Blocking tests use a normal test thread and own the entire client lifecycle.
macro_rules! exercise_contract {
    ($client:ty $(, $await:ident)?) => {{
        let (base, f) = fixture()?;
        let api = <$client>::builder(&base)
            .basic_auth("admin", "secret")
            .build().map_err(|_| "build Basic fixture client failed")?;

        let shown = api.vps().show(f.vps_id).send()$(.$await)?
            .map_err(|_| "VPS initial show failed")?.output;
        assert_eq!(shown.id, Some(f.vps_id));
        let hostname = shown.hostname;
        api.vps().update(f.vps_id)
            .info("Rust client disposable fixture")
            .send()$(.$await)?.map_err(|_| "VPS info update failed")?;
        let updated = api.vps().show(f.vps_id).send()$(.$await)?
            .map_err(|_| "VPS show after update failed")?.output;
        assert_eq!(updated.info.as_deref(), Some("Rust client disposable fixture"));
        assert_eq!(updated.hostname, hostname);

        let first_page = api.vps().index().limit(1).send()$(.$await)?
            .map_err(|_| "VPS first page failed")?.output;
        assert_eq!(first_page.len(), 1);
        let cursor = first_page[0].id.ok_or("fixture list item has no ID")?;
        let next_page = api.vps().index().limit(1).from_id(cursor)
            .send()$(.$await)?.map_err(|_| "VPS cursor page failed")?.output;
        assert!(next_page.iter().all(|vps| vps.id.is_some_and(|id| id > cursor)));

        api.dns_record().update(f.dns_record_id).ttl(120).send()$(.$await)?
            .map_err(|_| "DNS TTL value update failed")?;
        assert_eq!(api.dns_record().show(f.dns_record_id).send()$(.$await)?
            .map_err(|_| "DNS show after TTL value failed")?.output.ttl, Some(120));
        api.dns_record().update(f.dns_record_id).ttl_null().send()$(.$await)?
            .map_err(|_| "DNS TTL null update failed")?;
        assert_eq!(api.dns_record().show(f.dns_record_id).send()$(.$await)?
            .map_err(|_| "DNS show after TTL null failed")?.output.ttl, None);

        let custom = json!({"nested": [false, 0, "", null, {"key": "value"}]});
        let setting = api.webui_user_setting().set(&f.setting_namespace, &f.setting_key)
            .value(custom.clone()).send()$(.$await)?
            .map_err(|_| "WebUI Custom setting write failed")?.output;
        assert_eq!(setting.value, Some(custom.clone()));
        let saved = api.webui_user_setting().show(&f.setting_namespace, &f.setting_key)
            .send()$(.$await)?.map_err(|_| "WebUI Custom setting read failed")?.output;
        assert_eq!(saved.value, Some(custom));

        let unauthenticated = <$client>::builder(&base).build()
            .map_err(|_| "build unauthenticated fixture client failed")?;
        let login = unauthenticated.authentication__token__token().request()
            .user("admin").password("secret")
            .lifetime("renewable_manual").interval(120)
            .send()$(.$await)?.map_err(|error| match error.http_status() {
                Some(status) => format!("token request failed (HTTP {status})"),
                None => "token request failed (no HTTP response)".to_owned(),
            })?.output;
        let token = match login.token_step().map_err(|_| "token request step conversion failed")? {
            TokenStep::Complete { token } => token,
            _ => return Err("local admin fixture unexpectedly requires an auth challenge".into()),
        };
        let token_api = <$client>::builder(&base).token(token.expose_secret()).build()
            .map_err(|_| "build token fixture client failed")?;
        assert_eq!(token_api.vps().show(f.vps_id).send()$(.$await)?
            .map_err(|_| "VPS show with issued token failed")?.output.id, Some(f.vps_id));
        let renewed = token_api.authentication__token__token().renew().send()$(.$await)?
            .map_err(|_| "token renewal failed")?.output;
        assert!(renewed.valid_to.is_some());
        token_api.authentication__token__token().revoke().send()$(.$await)?
            .map_err(|_| "token revocation failed")?;
        assert!(token_api.vps().show(f.vps_id).send()$(.$await)?.is_err());

        let invalid = <$client>::builder(&base).basic_auth("admin", "invalid-fixture-password").build()
            .map_err(|_| "build invalid-credential fixture client failed")?;
        assert!(invalid.vps().show(f.vps_id).send()$(.$await)?.is_err());
        Ok(())
    }};
}

#[tokio::test]
#[ignore = "requires the disposable real vpsAdmin fixture"]
async fn async_vpsadmin_contract() -> TestResult {
    exercise_contract!(vpsadmin_client::Client, await)
}

#[cfg(feature = "blocking")]
#[test]
#[ignore = "requires the disposable real vpsAdmin fixture"]
fn blocking_vpsadmin_contract() -> TestResult {
    exercise_contract!(vpsadmin_client::blocking::Client)
}
