CREATE INDEX IF NOT EXISTS invoices_company_year_series_status_idx
ON invoices(company_id, fiscal_year, series, status);

CREATE INDEX IF NOT EXISTS invoices_created_at_idx
ON invoices(created_at);

CREATE INDEX IF NOT EXISTS audit_events_entity_idx
ON audit_events(entity_type, entity_id, occurred_at);

CREATE INDEX IF NOT EXISTS fiscal_exports_invoice_status_idx
ON fiscal_exports(invoice_id, status, created_at);
