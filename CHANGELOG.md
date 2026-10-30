# Changelog

## Unreleased

- Retain the Rust generator dependency lock and verify source, shared-schema and
  runtime identities before offline regeneration.
- Refresh HaveAPI 0.32.0 provenance. Generated API and runtime files are
  unchanged, so no crate release is prepared.

## 0.1.0

- Generate the captured public vpsAdmin API 7.0 surface with typed builders,
  partial output models and metadata.
- Include async and optional blocking transports, explicit authentication,
  nullable inputs and Custom JSON values.
- Retain the sanitized snapshot, provenance and offline regeneration tools.
