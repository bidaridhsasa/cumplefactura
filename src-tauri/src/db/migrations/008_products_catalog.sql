CREATE TABLE IF NOT EXISTS products (
  id TEXT PRIMARY KEY,
  company_id TEXT NOT NULL,
  sku TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  kind TEXT NOT NULL,
  unit_price_cents INTEGER NOT NULL,
  tax_rate_basis_points INTEGER NOT NULL,
  withholding_rate_basis_points INTEGER,
  discount_rate_basis_points INTEGER,
  active INTEGER NOT NULL DEFAULT 1,
  updated_at TEXT NOT NULL,
  UNIQUE(company_id, sku),
  FOREIGN KEY (company_id) REFERENCES companies(id)
);

CREATE INDEX IF NOT EXISTS products_company_active_idx
ON products(company_id, active, updated_at);
