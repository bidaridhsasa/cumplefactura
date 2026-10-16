# CumpleFactura invoice search filters

Goal: make local invoice lists usable as data grows.

Implemented:

- `InvoiceListFilters` for optional company, customer, fiscal year, series, status, kind and text search.
- `invoice_list(filters)` Tauri command remains backward compatible with no filters.
- SQLite query filters invoices and searches series, number, kind, status, fiscal hash, company and customer fields.
- UI panel `Buscar facturas` with text/select filters and `Limpiar filtros`.
- Browser smoke covers issued invoice search by status/text.

Verified:

- `cargo test`: 65 passed.
- `npm run build`: passed.
- `npm run validate:ui-contract`: passed.
- `npm run validate:browser-smoke`: passed.

Still pending:

- Pagination.
- Saved views.
- CSV export applying same filters.
