# CumpleFactura - ContaPro sidecar import

Fecha: 2026-06-22

## Objetivo

Rescatar de ContaPro sidecar solo lo que tiene evidencia local aceptada para MVP y exponerlo en CumpleFactura sin abrir red ni importar secretos.

## Importado

- Readiness redacted de VERI*FACTU, SII, FACe y TicketBAI.
- Clasificacion `mvp_importable` solo para VERI*FACTU y SII con manifiesto `accepted/passed`.
- Gate/evidence ledger de ContaPro sidecar como fuente local.
- Fronteras fail-closed para SII `/sii/submit-preflight`.
- UI `Canales ContaPro sidecar` con TicketBAI/FACe diferidos.

## Limites

- Sin llamada AEAT/SII/FACe/foral desde CumpleFactura.
- Sin certificados, passwords, claves privadas ni XML bruto.
- FACe sigue bloqueado por alta externa del certificado como SP.
- TicketBAI sigue bloqueado por licencia/politica/homologacion foral.
- TicketBAI queda visible como diferido, no importado al MVP.

## Validacion

- `cargo test`
- `npm run build`
- `npm run validate:ui-contract`
- `npm run validate:browser-smoke`
- `npm run validate:contapro-sidecar-import`
- `npm run validate:regulatory`
