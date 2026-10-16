# CumpleFactura - List Pagination Sprint

Date: 2026-06-04
Status: implemented

## Goal

Avoid loading unbounded local history in the main MVP lists before adding deeper fiscal audit screens and larger datasets.

## Scope

- Add shared Rust `ListPagination` with safe default `limit=50`, `offset=0` and hard cap `limit<=500`.
- Page `invoice_list`, `audit_list` and `fiscal_record_list` through SQLite `LIMIT/OFFSET`.
- Keep invoice filters compatible with pagination.
- Add UI pager controls for invoices, audit and fiscal records.
- Keep browser smoke mock aligned with the Tauri command shape.

## Evidence

- `db::tests::paginates_invoice_audit_and_fiscal_lists`
- `npm run validate:ui-contract`
- `npm run validate:browser-smoke`
- `npm run build`
- `cargo test`

## Limits

- No total row counts yet.
- No saved views yet.
- No custom page-size selector yet.
- Browser smoke still uses local Tauri invoke mock; Rust command path is covered by Rust tests and core E2E.

## Next

Recommended next sprint: fiscal audit screen with chain verifier UI and export report, because pagination now makes larger audit/fiscal histories practical.
