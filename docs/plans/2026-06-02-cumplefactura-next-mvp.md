# CumpleFactura Next MVP Implementation Plan

> For Codex: REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

Goal: move CumpleFactura from fiscal pre-MVP prototype to safer local MVP before external AEAT work.

Architecture: keep fiscal-critical behavior in Rust/Tauri commands, keep external AEAT/XAdES fail-closed, and use local evidence gates as acceptance. UI becomes a local workflow surface; external readiness remains blocked until vault/cert/manifest evidence exists.

Tech Stack: Tauri 2, Rust, SQLite, React, TypeScript, Playwright, local PowerShell/Node validation tools.

---

## Batch 1: Safety Foundation

- [x] Replace hardcoded user export paths with `app_defaults_get`.
- [x] Enable strict CSP in Tauri config.
- [x] Allocate invoice numbers in Rust from `invoice_sequences`; UI number is preview-only.
- [x] Add secret scan and desktop E2E validation hooks.

## Batch 2: Local MVP Usability

- [x] Add JSON/CSV/audit export commands.
- [x] Add safe backup restore preflight.
- [x] Add status/onboarding/backlog docs and visible pre-MVP limits.
- [x] Keep SII/foral and AEAT real calls unpromised.

## Batch 3: Evidence And Verification

- [x] Update README, module contract, regulatory plan and evidence matrix.
- [x] Add ledger entries for new tools.
- [x] Run full `npm run validate:regulatory` after the new Tauri build step completes.

## Implemented In This Sprint

- Rust command `app_defaults_get` resolves app/export directories from Tauri path APIs.
- UI no longer seeds export paths with a hardcoded Windows user profile.
- Tauri CSP is explicit instead of `null`.
- `invoice_issue` ignores UI numbers and reserves the next number in Rust from `invoice_sequences`.
- Demo seed is behind the explicit `Activar datos demo` control.
- Local exports now include PDF, JSON, CSV and fiscal audit JSON.
- Restore is deliberately `validated-only`; no destructive database replacement in MVP.
- Secret scan and desktop E2E evidence are part of the gate/ledger.

## Still Deferred

- Real desktop WebDriver interaction is opt-in with `CUMPLEFACTURA_TAURI_DESKTOP_E2E_REAL=1`; default gate mode is preflight.
- CSV/audit filters are accepted in the public command contract but MVP export still includes all records.
- Invoice editor and per-invoice series selector advanced on 2026-06-04; catalog, search/filter UI, restore execution and gestoria export pack remain feature work.
- AEAT/XAdES real path remains blocked until vault, cert, flags and redacted manifest evidence exist.

## Acceptance

- `cargo test` passes.
- `npm run build` passes.
- `npm run validate:secret-scan` passes.
- `npm run validate:tauri-desktop-e2e` emits evidence.
- `npm run validate:regulatory` passes or reports only documented external blockers.
