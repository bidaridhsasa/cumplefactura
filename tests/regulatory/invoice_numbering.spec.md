# Spec - invoice numbering

## Given
- Company with fiscal year 2026.
- Series A configured.
- No issued invoices.

## Must
- First issued invoice gets A-2026-000001.
- Next issued invoice gets A-2026-000002.
- Drafts do not consume final number.
- Failed issue operation does not consume final number.
- Parallel issue attempts cannot produce duplicates.

## Reject
- Manual number lower than last issued.
- Duplicate number in same series/year.
- Gap without audit event.

## Evidence
- Persist series, year, sequence, issued_at, audit_hash.
