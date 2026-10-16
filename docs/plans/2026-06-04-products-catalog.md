# CumpleFactura products catalog

Goal: make invoice creation faster with reusable company products/services.

Implemented:

- `products` SQLite table with per-company SKU uniqueness.
- `ProductInput`, `Product`, `ProductKind` domain types.
- `product_upsert` and `product_list` Tauri commands.
- Catalog UI for SKU, name, description, kind, price, IVA, IRPF, discount and active state.
- `Usar en linea` fills the first editable invoice line from catalog defaults.
- Browser smoke covers save product and apply product to invoice editor.

Verified:

- `cargo test`: 64 passed.
- `npm run build`: passed.
- `npm run validate:ui-contract`: passed.
- `npm run validate:browser-smoke`: passed.
- `npm run validate:regulatory`: passed.

Still pending:

- Catalog search/filter.
- Price history.
- Stock/unit metadata.
- Import/export catalog.
