# Changelog

## 0.2.0

- Regenerate API 7.0 from the vpsAdmin source fixture with all bundled plugins,
  retaining the exact source and exporter provenance.
- Add advisory retraction and deletion operations and `expected_state` inputs
  for advisory, CVE and node-status mutations.
- Direct struct literals for affected action `Input` types must include
  `expected_state` or use `..Default::default()`. Existing builder calls can omit
  this optional field.
- Retain the Rust generator dependency lock and verify source, shared-schema and
  runtime identities before offline regeneration.
- Refresh HaveAPI 0.32.2 generator provenance and its released dependency lock.

## 0.1.0

- Generate the captured public vpsAdmin API 7.0 surface with typed builders,
  partial output models and metadata.
- Include async and optional blocking transports, explicit authentication,
  nullable inputs and Custom JSON values.
- Retain the sanitized snapshot, provenance and offline regeneration tools.
