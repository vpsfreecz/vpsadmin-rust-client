# vpsAdmin Rust client

Typed async and optional blocking clients for **vpsAdmin API 7.0**, generated
from the public unauthenticated description at
<https://api.vpsfree.cz/v7.0/>. The package version is independent of the API
version. The saved schema and `SCHEMA_SHA256` identify the generated surface.

The capture describes 104 top-level resources, 152 resources including nested
and authentication resources, and 524 actions. Permission-dependent metadata
does not establish complete API or plugin coverage. See
[schema provenance](schema/provenance.json) and
[the inventory](schema/inventory.json) for the exact capture.

The crate contains its protocol runtime and generated source. Using it needs
neither Ruby, HaveAPI source nor a running API during compilation. Rust 1.95 or
newer is required. HTTPS uses reqwest's Rustls platform verifier and aws-lc
backend, which can require a C toolchain to build.

## Async use

The crate has not been published to crates.io. Use a local checkout as a path
dependency and add Tokio to your application:

```toml
[dependencies]
vpsadmin-client = { path = "../vpsadmin-rust-client" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Use a URL for the server mount root, without `/v7.0/`.

```rust,no_run
use vpsadmin_client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = Client::builder("https://api.vpsfree.cz")
        .token(std::env::var("VPSADMIN_TOKEN")?)
        .build()?;
    let vps = api.vps().show(123).send().await?.output;
    println!("{}", vps.hostname.as_deref().unwrap_or("<not returned>"));
    Ok(())
}
```

The [async example](examples/async_vps.rs) reads a VPS without changing it:

```sh
cargo run --locked --example async_vps -- 123
```

Set `VPSADMIN_TOKEN` in the environment before running the example. Set
`VPSADMIN_URL` to use another server offering API 7.0. Credentials are supplied
explicitly through `.token(...)`, `.oauth2_token(...)` or
`.basic_auth(username, password)`; choose one method. Tokens use the header
described by the captured API.

## Blocking use

Enable the `blocking` feature and use `vpsadmin_client::blocking::Client` with
the same builders and models. Calls end in `.send()?`.

```sh
cargo run --locked --features blocking --example blocking_vps -- 123
```

Construct, use and drop the blocking client outside an async runtime. Async
applications should use the async client, or put the entire blocking-client
lifecycle inside `tokio::task::spawn_blocking`. Aborting a started blocking
task does not cancel its HTTP request.

## Inputs, responses and failures

Setters send only the fields you supply. Unset fields stay omitted; `false`,
zero and empty strings are sent as values. Nullable fields have an explicit
`_null()` setter. For example, these calls change server state:

```rust,ignore
api.vps().update(vps_id).info("Maintenance note").send().await?;
api.dns_record().update(record_id).ttl_null().send().await?;
```

Custom parameters accept `serde_json::Value`. Ordinary response fields are
optional because permissions and server responses can omit them. Associations
preserve partial JSON maps; they do not fetch related objects. A `Response`
contains typed `output`, action metadata in `meta`, and `http_status`.

Action models live under modules such as `resources::vps::show`. Nested resource
accessors join resource components with double underscores and take parent path
arguments, for example `api.dataset__plan(dataset_id).show(plan_id)`. Token
actions use `api.authentication__token__token()`.

GET actions use bracketed scalar query parameters. Query null is encoded as an
empty string, so GET cannot distinguish every null/empty-string case. Nested
Custom JSON and list input in GET queries are unsupported. DELETE, POST, PUT and
PATCH actions use JSON request bodies, preserving explicit null, nested Custom
values and supported named list input.

The client validates input and response envelopes. Errors distinguish local
validation, encoding, transport, HTTP, API, protocol and typed decoding failures.
Default deadlines are 30 seconds per request and 10 seconds to connect, with a
16 MiB response limit. Builders can change these limits. Redirects and automatic
retries are disabled. A timeout may follow a completed server mutation; decide
how to recover before repeating it.

Pagination and token actions are explicit. Call `index().limit(...).from_id(...)`
for subsequent pages. Token login, challenges, renewal and revocation are
generated actions; the client does not install or refresh a token automatically.
Browser OAuth2 orchestration, automatic paging and ActionState polling are not
included. A successful action response can describe work still running on the
server.

## Development and releases

See [development and regeneration](docs/development.md) for pinned tools,
offline generation, local-server acceptance and release checks. Generated code
and copied runtime files must be refreshed through the HaveAPI generator.

MIT licensed; see [LICENSE](LICENSE).
