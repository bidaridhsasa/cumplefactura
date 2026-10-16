# Spec - append-only issued invoice

## Given
- Invoice in issued state.

## Must
- Changing customer, date, base, tax, total, or lines is blocked.
- Correction creates new rectifying invoice/event.
- Void/anulacion creates explicit event, never delete.
- Audit log stores actor, timestamp, operation, previous_hash, current_hash.
- Fiscal register stores each `Alta`/`Anulacion` as append-only row with current hash and previous fiscal hash.

## Reject
- UPDATE over issued invoice mutable fields.
- DELETE issued invoice.
- Recompute issued payload without audit event.
- UPDATE/DELETE over `fiscal_records`.

## Current executable coverage
- Rust DB tests prove `audit_events` and `fiscal_records` reject update/delete by SQLite trigger.
- Rust DB tests prove `insert_anulacion_record` appends fiscal record instead of mutating/removing the prior `Alta`.
