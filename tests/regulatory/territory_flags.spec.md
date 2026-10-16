# Spec - territory flags

## Flags
- verifactu_estatal
- sii
- ticketbai_bizkaia
- ticketbai_araba
- ticketbai_gipuzkoa
- navarra_htn
- grupo_iva

## Must
- Exactly one fiscal territory mode active per company unless explicit advanced config.
- AEAT VERI*FACTU is not substitute for TicketBAI/Batuz.
- Navarra HTN is separate from AEAT and PV.
- SII optional, disabled by default.

## Reject
- Applying state VERI*FACTU to foral company automatically.
