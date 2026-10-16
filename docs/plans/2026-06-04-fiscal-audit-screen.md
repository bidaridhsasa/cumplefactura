# CumpleFactura - Fiscal Audit Screen Sprint

Date: 2026-06-04
Status: implemented

## Goal

Make local fiscal traceability visible before adding deeper external AEAT work.

## Scope

- Add a dedicated local fiscal audit screen.
- Show chain status, verified fiscal record count, issue count and final hash by company.
- Add a manual `fiscal_chain_verify` action from the UI.
- Move fiscal audit export into the audit screen.
- Include `chain_verification` inside fiscal audit JSON exports.

## Evidence

- `services::export::tests::writes_fiscal_audit_json_with_chain_verification`
- `services::fiscal_chain::*` tests
- `npm run validate:ui-contract`
- `npm run validate:browser-smoke`
- `cargo test`

## Limits

- Local evidence only.
- Not certification, homologation or AEAT acceptance.
- Browser smoke uses a local Tauri invoke mock.

## Next

Recommended next sprint: safe native file picker for export/backup paths, then PDF template settings.
