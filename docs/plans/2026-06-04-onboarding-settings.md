# CumpleFactura onboarding settings

Goal: close the first-run local setup loop before deeper MVP features.

Implemented:

- `app_settings` SQLite singleton for onboarding status, export folder and fiscal flags.
- `app_settings_get` and `app_settings_update` Tauri commands.
- Onboarding panel in the UI for company, fiscal defaults, active series and export folder.
- Persisted export folder drives default backup/PDF/XML/JSON/CSV/audit paths.
- Draft, issue and rectifying flows use persisted default fiscal flags.
- Browser smoke and UI contract cover the onboarding controls.

Verified:

- `cargo fmt --all --check`: passed after formatting.
- `cargo test`: 63 passed.
- `npm run build`: passed.
- `npm run validate:ui-contract`: passed.
- `npm run validate:browser-smoke`: passed.
- `npm run validate:core-e2e`: passed.
- `npm run validate:secret-scan`: passed.
- `npm run validate:evidence-matrix`: passed.
- `npm run validate:evidence-ledger`: passed.
- `npm run validate:external-acceptance`: passed as `not_provided`.
- `npm run validate:external-readiness`: passed as safely `blocked`.
- `npm run validate:tauri-desktop-e2e`: passed preflight.
- `npm run validate:verifactu:xsd`: passed.
- Full `npm run validate:regulatory` was not rerun in this sprint because escalation was rejected by the usage limiter.

Still pending:

- Native safe file picker.
- Step-by-step wizard polish.
- Per-company fiscal defaults if multiple companies need different flags.
