# CumpleFactura - Safe File Picker Sprint

Date: 2026-06-04
Status: implemented

## Goal

Make local path selection safer before deeper export, backup and restore flows.

## Scope

- Add Tauri dialog plugin in Rust and TypeScript.
- Grant only `dialog:open` and `dialog:save` in the default desktop capability.
- Use native directory picker for export folder.
- Use native save dialogs for backup, PDF, JSON, CSV, XML, signed XML and audit report paths.
- Use native open dialog for restore source.
- Keep path fields read-only in the UI.

## Evidence

- `npm run validate:ui-contract`
- `npm run validate:browser-smoke`
- `npm run build`
- `cargo test`
- `npm run validate:regulatory`

## Limits

- Browser smoke mocks dialog IPC; it does not render the OS modal.
- Command-side validation remains the final write/restore guard.
- Restore remains preflight-only.

## Next

Recommended next sprint: PDF template settings.
