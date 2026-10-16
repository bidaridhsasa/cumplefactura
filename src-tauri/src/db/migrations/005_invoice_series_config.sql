CREATE TABLE IF NOT EXISTS invoice_series_configs (
  id TEXT PRIMARY KEY,
  company_id TEXT NOT NULL,
  label TEXT NOT NULL,
  series_code TEXT NOT NULL,
  fiscal_year INTEGER NOT NULL,
  include_year INTEGER NOT NULL DEFAULT 0,
  separator TEXT NOT NULL DEFAULT '',
  padding INTEGER NOT NULL DEFAULT 3,
  next_number INTEGER NOT NULL DEFAULT 1,
  is_default INTEGER NOT NULL DEFAULT 0,
  active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  UNIQUE (company_id, fiscal_year, series_code),
  FOREIGN KEY (company_id) REFERENCES companies(id)
);

CREATE INDEX IF NOT EXISTS idx_invoice_series_configs_company_year
ON invoice_series_configs(company_id, fiscal_year, active, is_default);
