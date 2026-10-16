# XAdES sidecar preflight/sign spec

Priority: P1

## Evidence

- CumpleFactura validates the XML against local VERI*FACTU XSD before preflight.
- Rust `services::xades_sidecar` checks local XML SHA256, local `/verifactu/sign-xml` endpoint, `vault://certs/...` certificate ref and `CUMPLEFACTURA_ENABLE_REAL_XADES_SIDECAR`.
- The UI calls `verifactu_xades_sidecar_preflight` and states that preflight does not call the sidecar.
- The UI calls `verifactu_xades_sidecar_sign` only after a ready preflight.
- Rust `services::xades_sidecar::sign_exact_xml_*` sends the exact XML to a local sidecar with HMAC, rejects non-real signature modes and writes a signed XML file.
- Sidecar evidence for the real signing boundary lives in `apps/contapro/services/sidecar/tests/test_fiscal_contracts.py::test_verifactu_sign_xml_*`.

## Limits

- XAdES signing requires explicit runtime flag, shared secret, ready preflight and sidecar real-XAdES mode.
- The CumpleFactura gate uses a fake local sidecar for HTTP/HMAC boundary coverage; certificate-backed signing is covered by the sidecar gate.
- No XML or certificate reference is sent to the sidecar during preflight.
- AEAT transport remains separately blocked until a real signed XML has `signature_status=signed_xades`.
