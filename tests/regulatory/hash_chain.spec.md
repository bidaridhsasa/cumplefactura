# Spec - hash chain

## Must
- Same canonical invoice payload gives same hash.
- Any relevant field change gives different hash.
- Invoice N stores previous_hash of invoice N-1 in same company/series/year.
- Chain verification detects missing, reordered, duplicated or modified records.
- VERI*FACTU RegistroAlta hash follows AEAT official field order:
  `IDEmisorFactura`, `NumSerieFactura`, `FechaExpedicionFactura`, `TipoFactura`, `CuotaTotal`, `ImporteTotal`, previous `Huella`, `FechaHoraHusoGenRegistro`.
- Output is SHA-256, uppercase hexadecimal, 64 chars.
- VERI*FACTU RegistroAnulacion hash follows AEAT official field order:
  `IDEmisorFacturaAnulada`, `NumSerieFacturaAnulada`, `FechaExpedicionFacturaAnulada`, previous `Huella`, `FechaHoraHusoGenRegistro`.

## Canonicalization
- Stable field order.
- Decimal normalization.
- UTF-8.
- No locale-dependent formatting.

## Current executable coverage
- Rust tests cover AEAT official examples:
  - first `RegistroAlta`: `3C464DAF61ACB827C65FDA19F352A4E3BDC2C640E9E9FC4CC058073F38F12F60`.
  - chained `RegistroAlta`: `F7B94CFD8924EDFF273501B01EE5153E4CE8F259766F88CF6ACB8935802A2B97`.
  - `RegistroAnulacion`: `177547C0D57AC74748561D054A9CEC14B4C4EA23D1BEFD6F2E69E3A388F90C68`.
- Rust DB tests prove `Alta` and `Anulacion` persist in `fiscal_records` with `previous_fiscal_hash`.
- Rust fiscal-chain tests prove:
  - valid `Alta` + `Anulacion` chain returns OK.
  - invoice tampering after fiscal record is detected as `HASH_MISMATCH`.
  - broken `previous_fiscal_hash` is detected as `BROKEN_CHAIN`.
