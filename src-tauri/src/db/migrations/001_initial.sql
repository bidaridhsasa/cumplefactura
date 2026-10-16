PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS schema_migrations (
  version INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS companies (
  id TEXT PRIMARY KEY,
  legal_name TEXT NOT NULL,
  tax_id TEXT NOT NULL UNIQUE,
  address TEXT NOT NULL,
  postal_code TEXT NOT NULL,
  municipality TEXT NOT NULL,
  province TEXT NOT NULL,
  country_code TEXT NOT NULL DEFAULT 'ES',
  default_series TEXT NOT NULL DEFAULT 'A',
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS customers (
  id TEXT PRIMARY KEY,
  display_name TEXT NOT NULL,
  tax_id TEXT NOT NULL,
  kind TEXT NOT NULL,
  address TEXT NOT NULL,
  postal_code TEXT NOT NULL,
  municipality TEXT NOT NULL,
  province TEXT NOT NULL,
  country_code TEXT NOT NULL DEFAULT 'ES',
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS invoice_sequences (
  company_id TEXT NOT NULL,
  fiscal_year INTEGER NOT NULL,
  series TEXT NOT NULL,
  next_number INTEGER NOT NULL DEFAULT 1,
  PRIMARY KEY (company_id, fiscal_year, series),
  FOREIGN KEY (company_id) REFERENCES companies(id)
);

CREATE TABLE IF NOT EXISTS invoices (
  id TEXT PRIMARY KEY,
  company_id TEXT NOT NULL,
  customer_id TEXT NOT NULL,
  fiscal_year INTEGER NOT NULL,
  series TEXT NOT NULL,
  number INTEGER,
  kind TEXT NOT NULL,
  status TEXT NOT NULL,
  rectifies_invoice_id TEXT,
  taxable_base_cents INTEGER NOT NULL,
  tax_cents INTEGER NOT NULL,
  withholding_cents INTEGER NOT NULL,
  total_cents INTEGER NOT NULL,
  fiscal_hash TEXT,
  created_at TEXT NOT NULL,
  issued_at TEXT,
  UNIQUE (company_id, fiscal_year, series, number),
  FOREIGN KEY (company_id) REFERENCES companies(id),
  FOREIGN KEY (customer_id) REFERENCES customers(id),
  FOREIGN KEY (rectifies_invoice_id) REFERENCES invoices(id)
);

CREATE TABLE IF NOT EXISTS invoice_lines (
  id TEXT PRIMARY KEY,
  invoice_id TEXT NOT NULL,
  position INTEGER NOT NULL,
  description TEXT NOT NULL,
  quantity INTEGER NOT NULL,
  unit_price_cents INTEGER NOT NULL,
  taxable_base_cents INTEGER NOT NULL,
  tax_rate_basis_points INTEGER NOT NULL,
  tax_cents INTEGER NOT NULL,
  withholding_rate_basis_points INTEGER NOT NULL DEFAULT 0,
  withholding_cents INTEGER NOT NULL DEFAULT 0,
  FOREIGN KEY (invoice_id) REFERENCES invoices(id)
);

CREATE TABLE IF NOT EXISTS invoice_territory_flags (
  invoice_id TEXT NOT NULL,
  flag TEXT NOT NULL,
  PRIMARY KEY (invoice_id, flag),
  FOREIGN KEY (invoice_id) REFERENCES invoices(id)
);

CREATE TABLE IF NOT EXISTS audit_events (
  id TEXT PRIMARY KEY,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  action TEXT NOT NULL,
  actor TEXT NOT NULL,
  occurred_at TEXT NOT NULL,
  payload_hash TEXT NOT NULL,
  previous_hash TEXT,
  hash TEXT NOT NULL UNIQUE
);

CREATE TRIGGER IF NOT EXISTS audit_events_no_update
BEFORE UPDATE ON audit_events
BEGIN
  SELECT RAISE(ABORT, 'audit_events is append-only');
END;

CREATE TRIGGER IF NOT EXISTS audit_events_no_delete
BEFORE DELETE ON audit_events
BEGIN
  SELECT RAISE(ABORT, 'audit_events is append-only');
END;

CREATE TABLE IF NOT EXISTS fiscal_exports (
  id TEXT PRIMARY KEY,
  invoice_id TEXT NOT NULL,
  provider TEXT NOT NULL,
  environment TEXT NOT NULL,
  payload_version TEXT NOT NULL,
  payload_hash TEXT NOT NULL,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  FOREIGN KEY (invoice_id) REFERENCES invoices(id)
);
