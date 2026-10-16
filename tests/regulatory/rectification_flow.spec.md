# Spec - rectification flow

## Must
- Rectification references original invoice id/number/hash.
- Original invoice remains immutable.
- Rectifying invoice has own number/hash/audit event.
- Totals and tax impact exported separately.
- VERI*FACTU rectifying Alta includes `TipoFactura`, `TipoRectificativa` and `FacturasRectificadas`.
- Rectifying invoices require explicit cause `R1-R5` and method `I/S`.
- Substitutive rectification `S` includes `ImporteRectificacion`.
- Anulacion uses `RegistroAnulacion`, references original invoice number/date/hash and has its own AEAT SHA-256 hash.
- Anulacion XML must pass local VERI*FACTU XSD before export success.
- Anulacion persists a new fiscal record and marks the invoice as `Void`.

## Reject
- Editing original invoice to simulate correction.
- Deleting original invoice.

## Current executable coverage
- Rust tests cover official AEAT anulacion hash vector:
  `177547C0D57AC74748561D054A9CEC14B4C4EA23D1BEFD6F2E69E3A388F90C68`.
- Rust tests cover XSD-valid generated `RegistroAnulacion` XML.
- Rust DB tests cover `insert_anulacion_record`: append fiscal record + invoice `Void`.
- Rust tests cover basic incremental rectifying invoice:
  - negative adjustment line allowed only for `Rectifying`.
  - source invoice is marked `Rectified`.
  - generated XML contains `R4`, `TipoRectificativa=I` and `FacturasRectificadas`.
  - generated rectifying XML passes local VERI*FACTU XSD.
- Rust tests cover substitutive rectifying XML shape:
  - generated XML contains `R1`, `TipoRectificativa=S` and `ImporteRectificacion`.
  - `BaseRectificada` and `CuotaRectificada` come from the source invoice.
- Rust DB tests cover persisted rectification cause/method metadata.

## Current limit
- Cause selector `R1-R5` and methods `I/S` are implemented technically. A guided legal/business wizard for choosing the correct cause is still pending.
