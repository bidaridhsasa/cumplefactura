ALTER TABLE invoice_lines
ADD COLUMN discount_rate_basis_points INTEGER NOT NULL DEFAULT 0;
