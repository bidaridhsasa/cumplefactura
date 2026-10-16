# Spec - audit export

## Must
- Export issued invoices, audit events, hashes and configuration by period.
- Export is deterministic: same DB state gives same package hash.
- Export includes app version and schema version.
- Export excludes secrets/private keys.
- VERI*FACTU submission preparation validates XML, computes SHA256, records `fiscal_exports` and marks `prepared_for_signature`.
- Prepared package states explicitly that XAdES/signature and network submission are pending.

## Reject
- Non-deterministic timestamps during export.
- Missing audit events for issued invoices.
- Claiming signed/submitted status without certificate-backed signature or AEAT transport evidence.

## Current executable coverage
- Rust `services::submission` test prepares unsigned package with SHA256 and `pending_xades` / `not_submitted`.
