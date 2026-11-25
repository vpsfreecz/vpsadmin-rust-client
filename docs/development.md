# Development and regeneration

`src/lib.rs`, package files, examples and tests are handwritten.
`src/generated/**` is owned by the HaveAPI Rust generator, including the runtime
copied from `clients/rust/runtime/src/runtime/**`. Fix maintained HaveAPI source
and regenerate when generated behavior needs a change. A runtime fix in HaveAPI
reaches crate consumers only after regeneration and a new client release.

## Tools

Enter the committed Nix environment with `nix develop`. The client retains its
own pinned nixpkgs revision:
`fe13242281a67641c0102cdb1087f0ebe586c7df`. Generation requires the exact formatter
output `rustfmt 1.9.0` recorded in `generator.toml`, using edition 2024.
The pinned compiler is newer than the separate Rust 1.95 minimum.

The crate depends on reqwest, Serde, serde_json and url. The optional `blocking`
feature enables reqwest's blocking transport. Tokio is a development/example
dependency. Retain `Cargo.lock` for repeatable verification.

## Saved schema

`schema/api-7.0.json` is the canonical sanitized API 7.0 schema exported from a
clean, committed vpsAdmin checkout by `tools/export-api-schema`. The exporter
mounts the API with all six bundled plugins and English fixture data in a new
temporary database. Profile `vpsadmin-full-fixture` describes declared operations
and parameters. It does not describe a user's permissions or certify a deployed
installation's capabilities. Exporting does not contact a deployed API.

`schema/source-provenance.json` retains the exporter's source revision, plugin
and fixture/configuration hashes, dependency identities and schema digest.
`schema/tooling.Gemfile.lock` retains its exact dependency graph.
`schema/provenance.json` binds that receipt and schema to this client's generator,
copied runtime, generator dependency lock and toolchain. `schema/inventory.json`
describes the exported resources, actions and parameters.

To refresh the schema, follow vpsAdmin's
[schema export and generated-client synchronization procedure](https://github.com/vpsfreecz/vpsadmin/blob/master/docs/api-schema-export.md#generated-client-synchronization),
review the source export and retain its schema,
provenance and tooling lock together. Update client provenance and inventory,
then regenerate from the saved schema using the pinned generator. Never save
credentials, database URLs, login responses or transient tokens.

The sanitizer removes prose, examples, validator messages and protected defaults
while preserving wire names, paths, validators, namespaces and nullability.
Canonical JSON sorts object keys, uses two-space indentation and LF newlines,
and ends with a newline. Arrays keep their order.

Follow that procedure when API, shared-schema, generator or copied-runtime
changes affect this client. Record the API source revision, then update the
client default branch or record evidence against that revision that no generated
change is needed. A successful `tools/regenerate --check` proves consistency
with the saved inputs; it does not prove they match current API source.

Use `incident_policy().index()` to discover policies and
`vps__incident_policy(vps_id).index()` or `.show(key)` to read their current rows
for a VPS. Policy keys come from the registry at runtime; pass the returned
string key in policy paths. The server validates policy keys and status strings.
Initial VPS reads omit policy filters; only explicitly selected filters are sent.
Check server capabilities before enabling these calls.

Policy `.update(key)` preserves explicit null override resets and returns the
authoritative policy row. Change shared VPS shutdown permission through a
separate `vps().update(vps_id)`, then use `vps().show(vps_id)` to confirm its
current value. A shared-permission update can succeed without returning
action-state metadata.

Current rows include counts, effective limits, overrides and
`configuration_valid`. Limits reflect the declaration and current overrides.
Invalid configurations return null effective limits and status. Missing or null
output values decode to `None`; datetimes remain strings. Evaluation IDs,
timestamps, states and `incident_report__policy_evaluation(id)` describe saved
history. They do not supply current limits or predict whether a policy will
match a future event.

Saved evaluations include nullable `matched` and the count-window lengths
`warning_period_days` and `stop_period_days`. `Option<bool>` keeps `false`
distinct from unknown membership (`None` for null or a missing field).
A report's `notice_policy_key` and `notice_policy_label` identify its saved
originating policy and use `None` when unavailable. They do not select a current
policy. Active registry and VPS policy metadata have no `source_type` field.

Incident reports expose nullable notice kind, triggering report ID and mail
state. Ordinary reports return null for all three. Notice kind distinguishes a
warning from a confirmed stop. The triggering report ID is an integer and may
be unavailable. Mail state `pending` means the notice has not been queued.
`queued` does not establish SMTP success; `sent` means SMTP succeeded, without
proving delivery to the recipient's inbox.

## System configuration metadata

SystemConfig outputs expose `choices` as optional Custom JSON and
`value_nullable` as an optional boolean. Missing and null metadata decode to
`None`, allowing reads from older APIs. Consumers validate the ordered choice
values and labels before offering a select; labels already use the API request
language. Update retains its existing Custom value input, including explicit
null. The server enforces the setting's membership and nullability rules.

## Offline generation

Obtain a clean, separate HaveAPI checkout at the exact revision recorded in
provenance. Use a checkout you own: dependency preparation writes its component
lock and must not change another developer's checkout. HaveAPI ignores library
locks upstream. This repository retains the prepared Rust component lock in
`schema/generator.Gemfile.lock`; provenance and the generated ownership manifest
bind its SHA-256. Restore those exact bytes before installing dependencies, and
stop if an existing lock differs.

The generator and its Ruby dependencies are development tools. Crate consumers
do not need them. The generator uses Fiddle for atomic directory exchange, and
the Nix shell supplies libffi to build its native extension. Enter `nix develop`
and prepare an isolated Ruby 3.3 bundle frozen to the retained lock:

```sh
(
set -euo pipefail
export HAVEAPI_SOURCE=/path/to/recorded/haveapi
tools/check-provenance
revision=$(python3 -c 'import json; print(json.load(open("schema/provenance.json"))["generator"]["revision"])')
test "$(git -C "$HAVEAPI_SOURCE" rev-parse HEAD)" = "$revision"
lock="$HAVEAPI_SOURCE/clients/rust/Gemfile.lock"
test ! -L "$lock"
if [ -e "$lock" ]; then
  cmp schema/generator.Gemfile.lock "$lock"
else
  cp schema/generator.Gemfile.lock "$lock"
fi
export GEM_HOME="$PWD/target/generator-gems"
export GEM_PATH="$GEM_HOME:$(ruby -e 'puts Gem.default_dir')"
export PATH="$GEM_HOME/bin:$PATH"
export BUNDLE_GEMFILE="$HAVEAPI_SOURCE/clients/rust/Gemfile"
export BUNDLE_PATH="$PWD/target/generator-bundle"
export BUNDLE_USER_HOME="$PWD/target/bundler-user"
export BUNDLE_APP_CONFIG="$PWD/target/bundler-config"
export BUNDLE_FROZEN=true
gem install --no-document bundler --version 4.0.23
bundle install
bundle check
cmp schema/generator.Gemfile.lock "$lock"
python3 -B tools/test-regenerate.py
tools/regenerate
tools/regenerate --check
cmp schema/generator.Gemfile.lock "$lock"
)
```

Keep the recorded HaveAPI checkout and lock unchanged. Do not reuse its Ruby 3.4
native gems or write Bundler configuration into that checkout. Generation uses
this crate's exact `rustfmt 1.9.0`; the HaveAPI development formatter may be newer.
If frozen installation fails on a supported Ruby/platform, investigate the
recorded dependency graph. Preparing a different graph requires an explicit
provenance refresh.

Generation reads the saved snapshot without contacting the API. Both commands
verify the source revision and cleanliness, retained and installed locks, loaded
generator and shared-schema sources, and maintained runtime against provenance.
Neither installs dependencies, restores locks or resolves a graph. `--check`
reports drift without writing files. Regeneration replaces only `src/generated`
and preserves handwritten package files. Review manifest requirements when the
generator changes. Commit generated refreshes separately from handwritten
changes, and update source, lock and shared-schema identities in provenance for
each refresh.

## Verification

Known quick checks are `cargo fmt --all -- --check`, `bash -n tools/regenerate`,
`tools/check-provenance`, `python3 -B tools/test-provenance.py`, and deterministic
`tools/regenerate --check` once the generator is available. The provenance check
binds the source receipt, schema, inventory, exporter and generator locks,
generator configuration and copied runtime.
The preparation block also runs `python3 -B tools/test-regenerate.py` for
refusal checks in disposable source copies.

Compilation and acceptance commands are:

```sh
cargo test --locked --all-targets
cargo test --locked --all-targets --features blocking
cargo doc --locked --all-features --no-deps
cargo package --locked
```

Also verify the minimum supported compiler with Rust 1.95 and the committed
lockfile. CI runs offline-schema generation and the default/blocking matrices;
ordinary checks need no live API.

The real vpsAdmin acceptance test is ignored by default because it changes
disposable server state. Start HaveAPI's
`clients/rust/spec/integration/support/vpsadmin_server.rb` with an explicit
vpsAdmin fixture-source checkout, following that harness's instructions. It
owns its ephemeral listener and private database. Pass its readiness URL as
`VPSADMIN_TEST_BASE_URL` and its `VPSADMIN_FIXTURE_READY` JSON object as
`VPSADMIN_FIXTURE_JSON`. Set `VPSADMIN_REAL_INTEGRATION=1` to opt in, then run:

```sh
cargo test --locked --all-features --test real_vpsadmin -- --ignored --test-threads=1
```

The test accepts loopback HTTP URLs only, uses the fixture's local Basic
credentials, and exercises async/blocking VPS show and info update, pagination,
nullable DNS TTL, Custom JSON and token actions. Do not point it at production.
Stop the fixture through its owning harness when verification ends.

## Compatibility and release checks

API version 7.0 selects the generated contract. Crate SemVer versions the Rust
interface, including copied common types. A compatible server may add fields;
older clients tolerate unknown output fields. Removed actions, changed public
types, newly required inputs or a different API target need a breaking crate
version. During 0.x this means a new minor version. Document MSRV changes.

Ordinary source synchronization does not itself require a crate release.
Assess Rust interface compatibility under this crate's versioning policy when
preparing a release, and follow the linked synchronization procedure for task
completion and any dependent release requirements.

Before an authorized release, review the source and generator provenance,
regenerate, run the verification matrix and package check, and update
`CHANGELOG.md`. Inspect
`cargo package --list --locked` so the package contains the generated runtime
and has no HaveAPI path or Git dependency. Publishing to crates.io is a separate
explicit action through the manual workflow described below.

No server deployment, database migration or coordinated node update is needed.
Old and new client versions can coexist while the server offers their API
versions. Consumers can recover by pinning an older crate and lockfile. Rolling
back a client cannot undo server actions it already submitted.

## Publishing from GitHub Actions

Run `release.yml` manually against a tag matching
`v<Cargo.toml package.version>`. The workflow publishes `vpsadmin-client` from
this repository after the normal CI matrix passes for that commit. It checks
that the tag still points to the verified commit before publishing. Publication
runs execute one at a time; branch pushes and pull requests do not publish.

Both `release.yml` and `ci.yml` must exist on the default branch and at the
tagged revision. GitHub cannot dispatch a workflow that exists only on a feature
branch. A repository administrator must create the `release` environment,
require reviewers and restrict deployments to the intended version tags.
Configure these protections and registry credentials before running a release.

### First publication

[crates.io trusted publishing](https://crates.io/docs/trusted-publishing)
requires an existing crate. For its first version, create a short-expiry API
token with the minimum available publish scope for `vpsadmin-client`. Store it
as the `release` environment secret `CARGO_PUBLISH_BOOTSTRAP_TOKEN`. No local
Cargo login is needed.

After reviewing and pushing the version tag, select bootstrap mode. Replace
`v0.1.0` with the tag matching the manifest:

```sh
gh workflow run release.yml --repo vpsfreecz/vpsadmin-rust-client \
  --ref v0.1.0 -f authentication=bootstrap
```

Bootstrap requires a missing crate (registry HTTP 404) and a configured token.
An existing crate, missing secret or failed registry lookup stops publication.
The token is supplied only to `cargo publish --locked` after verification.

### Subsequent publications

After the first upload, a crate owner must register a trusted publisher in
crates.io Settings → Trusted Publishing with owner `vpsfreecz`, repository
`vpsadmin-rust-client`, workflow `release.yml` and environment `release`.
Revoke the bootstrap token and delete its environment secret. Dispatch later
version tags using the default OIDC mode:

```sh
gh workflow run release.yml --repo vpsfreecz/vpsadmin-rust-client \
  --ref v0.2.0 -f authentication=oidc
```

The workflow obtains OIDC credentials after verification and toolchain setup,
then passes them only to the publish step. OIDC failure never falls back to an API token.
Package checks and dry runs do not prove that registry authentication or upload
works; those require the owner prerequisites and an authorized publication.

If an upload fails, inspect registry state before retrying: a version may have
been accepted even if the workflow failed. Never move a release tag or try to
overwrite a published version. To stop future publication, disable the workflow
or remove its trusted publisher binding.
