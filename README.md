# vpsAdmin Rust client

Rust client library for [vpsAdmin](https://github.com/vpsfreecz/vpsadmin)
API 7.0, with async and optional blocking clients. Requires Rust 1.95 or newer.
The crate version and API version are independent.

## Installation

Add the client and Tokio to your application:

```toml
[dependencies]
vpsadmin-client = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Building the TLS dependencies may require a C toolchain.

## Usage

Set `VPSADMIN_TOKEN` in your environment to an existing HaveAPI token. This
complete program reads the first page of up to 50 VPSes visible to that token
and prints their ID, hostname and running state:

```rust,no_run
use vpsadmin_client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("VPSADMIN_TOKEN")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or("Set VPSADMIN_TOKEN to a nonempty existing token")?;
    let api = Client::builder("https://api.vpsfree.cz")
        .token(token)
        .build()?;

    let response = api.vps().index().limit(50).send().await?;
    for vps in response.output {
        let id = vps.id.map(|id| id.to_string()).unwrap_or_else(|| "<not returned>".into());
        let hostname = vps.hostname.as_deref().unwrap_or("<not returned>");
        let state = match vps.is_running {
            Some(true) => "running",
            Some(false) => "stopped",
            None => "unknown",
        };
        println!("{id}\t{hostname}\t{state}");
    }
    Ok(())
}
```

Give the builder the server base URL without `/v7.0/`; generated routes already
select API 7.0. Public actions need no authentication. Server permissions still
apply to authenticated requests, and fields can be missing or null.

For another page, use `.from_id(id).limit(50)` on the index action. The response
metadata includes an optional `total_count`. A single list call does not fetch
all VPSes automatically.

## Working with VPSes

The following snippets use the authenticated `api` from the usage example.
Choose a location and OS template, create a VPS, then use its returned ID to
inspect and manage it. Choose one creation example and only the power action
you need.

### Choose a location and OS template

List locations where you can create a VPS:

```rust,ignore
let locations = api.location().index().has_hypervisor(true).send().await?;
for location in locations.output {
    let id = location.id.map(|id| id.to_string()).unwrap_or_else(|| "<not returned>".into());
    println!("{id}\t{}", location.label.as_deref().unwrap_or("<not returned>"));
}
```

Choose a location with a returned ID and supply it as `location_id: i64`.
Then list templates compatible with that location:

```rust,ignore
let templates = api.os_template().index().location(location_id).send().await?;
for template in templates.output {
    let id = template.id.map(|id| id.to_string()).unwrap_or_else(|| "<not returned>".into());
    println!("{id}\t{}", template.label.as_deref().unwrap_or("<not returned>"));
}
```

Choose a template with a returned ID and supply it as `template_id: i64`.
Use those selected IDs in one of the following creation examples.

### Create a VPS

To create a VPS, choose a hostname, location and OS template. The minimal
request creates `my-vps`:

```rust,ignore
let created = api.vps()
    .create()
    .hostname("my-vps")
    .location(location_id)
    .os_template(template_id)
    .send()
    .await?;
let vps_id = created.output.id.ok_or("Create response did not return a VPS ID")?;
```

The VPS starts by default. Omitted resource sizes use the API's defaults. By
default it receives one IPv4 and one IPv6 allocation where IPv6 is available.

To choose resource sizes and public IP allocation counts, use this alternative
creation request:

```rust,ignore
let created = api.vps()
    .create()
    .hostname("my-vps")
    .location(location_id)
    .os_template(template_id)
    .cpu(2)
    .memory(2048)
    .diskspace(10240)
    .ipv4(1)
    .ipv6(1)
    .send()
    .await?;
let vps_id = created.output.id.ok_or("Create response did not return a VPS ID")?;
```

Memory and disk space use MiB: `2048` is 2 GiB and `10240` is 10 GiB. `cpu` sets
CPU capacity. Requested resources must fit your quota and be available at the
chosen location.

The `ipv4` and `ipv6` values are allocation counts, not addresses. An allocation
can be an individual address or a routed prefix. IPv6 requires support at the
chosen location.

Creating a VPS starts background work. Before continuing, check that creation
has completed successfully. A returned VPS ID or an unfinished operation does
not establish completion. Check completion again before each dependent action;
the same rule applies to power controls and deletion. The SDK does not wait for
background completion automatically.

### Inspect and rename the VPS

Use the `vps_id` returned by the create alternative you chose:

```rust,ignore
let response = api.vps().show(vps_id).send().await?;
let vps = response.output;
println!("{}", vps.hostname.as_deref().unwrap_or("<not returned>"));
```

To rename it, send a separate update that selects only `hostname`:

```rust,ignore
api.vps()
    .update(vps_id)
    .hostname("new-hostname")
    .send()
    .await?;
```

Only fields set on an action builder are sent. Setters preserve `false`, zero
and empty strings. Where the schema permits null, use the field's `_null()`
setter, such as `.ttl_null()` when updating a DNS record. Custom parameters
accept `serde_json::Value`. Nested resources take the parent ID and join names
with double underscores, for example `api.dataset__plan(dataset_id).show(plan_id)`.

### Start, stop or restart

Each action uses the same `vps_id`. Run the operation you need as a separate
request, then check its completion before depending on the resulting state.

Start a stopped VPS. Creation already starts the VPS by default:

```rust,ignore
api.vps().start(vps_id).send().await?;
```

Stop the VPS gracefully:

```rust,ignore
api.vps().stop(vps_id).send().await?;
```

Or restart it gracefully:

```rust,ignore
api.vps().restart(vps_id).send().await?;
```

### Delete the VPS

When the VPS is no longer needed:

```rust,ignore
api.vps().delete(vps_id).send().await?;
```

Deletion schedules the VPS to stop and its IP allocations to be released.
Disk data is removed later.

### Blocking client

Enable the `blocking` feature:

```toml
[dependencies]
vpsadmin-client = { version = "0.1", features = ["blocking"] }
```

This lists the same first page using synchronous calls:

```rust,no_run
use vpsadmin_client::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("VPSADMIN_TOKEN")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or("Set VPSADMIN_TOKEN to a nonempty existing token")?;
    let api = Client::builder("https://api.vpsfree.cz")
        .token(token)
        .build()?;

    for vps in api.vps().index().limit(50).send()?.output {
        let id = vps.id.map(|id| id.to_string()).unwrap_or_else(|| "<not returned>".into());
        let hostname = vps.hostname.as_deref().unwrap_or("<not returned>");
        let state = match vps.is_running {
            Some(true) => "running",
            Some(false) => "stopped",
            None => "unknown",
        };
        println!("{id}\t{hostname}\t{state}");
    }
    Ok(())
}
```

Construct, use and drop the blocking client outside an async runtime. In an
async application, use the async client or put the entire blocking-client
lifecycle inside `tokio::task::spawn_blocking`.

## Authentication

Configure one authentication method on the client builder. An existing HaveAPI
token uses `.token(token)`, HTTP Basic uses `.basic_auth(username, password)`,
and an OAuth2 access token uses `.oauth2_token(token)`. Keep credentials outside
source code and logs.

Token login, challenges, renewal and revocation are explicit actions under
`api.authentication__token__token()`. The client does not acquire or refresh
tokens automatically.

## Responses, errors and timeouts

Successful calls return typed `output`, action metadata in `meta` and
`http_status`. Output fields use `Option`: a missing or null field becomes
`None`. Related objects can be partial; the client does not fetch them
automatically. The generated client covers a public API snapshot; its plugin
completeness is unknown.

Requests return errors for invalid inputs, transport failures, unsuccessful
HTTP or API responses, and invalid response data. The examples propagate these
errors with `?`. Redirects and automatic retries are disabled.

The default request timeout is 30 seconds and the connection timeout is 10
seconds. Set `.timeout(std::time::Duration::from_secs(60))` or
`.connect_timeout(std::time::Duration::from_secs(10))` on the builder to change
them. A timeout can occur after a write was applied; check the server's state
before repeating it. Some successful actions start background work, so a
successful response does not mean that work has finished. The client does not
wait for background completion automatically.

## Development and regeneration

See the [API reference](https://docs.rs/vpsadmin-client) for resources, actions
and response types. The [async](examples/async_vps.rs) and
[blocking](examples/blocking_vps.rs) repository examples inspect one VPS using
`VPSADMIN_TOKEN`:

```sh
cargo run --locked --example async_vps -- 123
cargo run --locked --features blocking --example blocking_vps -- 123
```

Set `VPSADMIN_URL` for those repository examples to use another server offering
API 7.0.

The client is generated with [HaveAPI](https://github.com/vpsfreecz/haveapi).
See [development and regeneration](docs/development.md) for source maintenance,
saved schema provenance, verification and release checks. Fix generated behavior
in the maintained HaveAPI generator, then regenerate; do not edit
`src/generated` manually.

MIT licensed; see [LICENSE](LICENSE).
