CREATE TABLE IF NOT EXISTS fiscal_records (
  id TEXT PRIMARY KEY,
  company_id TEXT NOT NULL,
  invoice_id TEXT NOT NULL,
  record_type TEXT NOT NULL CHECK (record_type IN ('Alta', 'Anulacion')),
  generated_at TEXT NOT NULL,
  fiscal_hash TEXT NOT NULL UNIQUE,
  previous_fiscal_hash TEXT,
  xml_path TEXT,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  FOREIGN KEY (company_id) REFERENCES companies(id),
  FOREIGN KEY (invoice_id) REFERENCES invoices(id)
);

CREATE INDEX IF NOT EXISTS fiscal_records_company_chain_idx
ON fiscal_records(company_id, generated_at);

CREATE INDEX IF NOT EXISTS fiscal_records_invoice_type_idx
ON fiscal_records(invoice_id, record_type);

CREATE TRIGGER IF NOT EXISTS fiscal_records_no_update
BEFORE UPDATE ON fiscal_records
BEGIN
  SELECT RAISE(ABORT, 'fiscal_records is append-only');
END;

CREATE TRIGGER IF NOT EXISTS fiscal_records_no_delete
BEFORE DELETE ON fiscal_records
BEGIN
  SELECT RAISE(ABORT, 'fiscal_records is append-only');
END;
