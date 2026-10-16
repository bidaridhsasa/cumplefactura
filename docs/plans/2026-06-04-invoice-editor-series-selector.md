# CumpleFactura invoice editor + series selector

Goal: make the next local MVP step usable for real invoice drafting before catalog/onboarding work.

Implemented:

- UI invoice editor for editable lines: description, integer quantity, unit price, discount %, IVA % and IRPF %.
- Per-invoice series selector using configured company series.
- Rust domain support for `discount_rate_basis_points`; totals are calculated after discount before IVA/IRPF.
- SQLite migration `006_invoice_line_discounts.sql` persists line discount rate.
- PDF/JSON invoice data include unit price and discount rate.
- Browser smoke edits a real line with discount/IRPF before draft/issue.
- Vite ignores `src-tauri/target` to avoid Windows watcher locks during smoke.

Verified:

- `cargo test`: 60 passed.
- `npm run build`: passed.
- `npm run validate:ui-contract`: passed.
- `npm run validate:browser-smoke`: passed.

Follow-up completed 2026-06-04:

- Real company/customer forms beyond demo seed.

Still pending:

- Product/service catalog.
- Decimal quantities if required commercially.
- Search/filter/pagination.
- Full desktop WebDriver E2E in real mode.
