# CumpleFactura Invoice Series Config

Goal: each company can continue an old invoice numbering sequence or start a new one before issuing invoices.

## Implemented

- Company-level series configs with code, fiscal year, optional year suffix, separator, padding and next number.
- Examples covered:
  - `VENDI` + `2026` + `01` => `VENDI202601`
  - `AL` + `-` + `001` => `AL-001`
  - `C` + `-` + `001` => `C-001`
  - `I` + `-` + `001` => `I-001`
- Saving a series seeds `invoice_sequences`.
- Saving rejects `next_number` if it is not greater than existing issued invoices for that company/year/series.
- UI uses selected configured series for draft/issue.

## Follow-up completed 2026-06-04

- Invoice editor with editable lines, discounts, IVA and IRPF.
- Per-invoice selector for configured active series.

## Deferred

- Import from historical invoices.
- Product/service catalog.
- Real company/customer forms beyond demo seed.
