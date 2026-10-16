# CumpleFactura real company/customer forms

Goal: remove practical dependency on demo seed for local invoice creation.

Implemented:

- Real company form with create/edit by ID.
- Real customer form with create/edit by ID.
- `company_update` Tauri command.
- `customer_upsert` accepts optional `id` and uses `ON CONFLICT DO UPDATE`, avoiding delete/reinsert.
- Company/customer list rows now include address fields so selecting existing records refills forms.
- Demo button only fills fields; it no longer gates create/edit.

Verified:

- `cargo test`: 62 passed.
- `npm run build`: passed.

Still pending:

- Onboarding wizard that guides company, fiscal defaults, first series and export folder in one flow.
- Deletion/archive policy for companies/customers.
- Customer search/filter once lists grow.
