use hmac::{Hmac, Mac};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::Duration;

const ENABLE_SII_PREFLIGHT_ENV: &str = "CUMPLEFACTURA_ENABLE_SII_SUBMIT_SIDECAR_PREFLIGHT";
const ENABLE_TICKETBAI_PREFLIGHT_ENV: &str =
    "CUMPLEFACTURA_ENABLE_TICKETBAI_SUBMIT_SIDECAR_PREFLIGHT";
const SIDECAR_SHARED_SECRET_ENV: &str = "CUMPLEFACTURA_SIDECAR_SHARED_SECRET";
const SIDECAR_SHARED_SECRET_FALLBACK_ENV: &str = "SIDECAR_SHARED_SECRET";
const DEFAULT_SII_PREFLIGHT_URL: &str = "http://127.0.0.1:8000/sii/submit-preflight";
const DEFAULT_TICKETBAI_PREFLIGHT_URL: &str = "http://127.0.0.1:8000/ticketbai/submit-preflight";
const SII_PREFLIGHT_PATH: &str = "/sii/submit-preflight";
const TICKETBAI_PREFLIGHT_PATH: &str = "/ticketbai/submit-preflight";
const CERT_REF_PREFIX: &str = "vault://certs/";
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportedSidecarChannel {
    pub channel: String,
    pub status: String,
    pub source: String,
    pub blockers: Vec<String>,
    pub mvp_importable: bool,
    pub acceptance_status: Option<String>,
    pub acceptance_validation_status: Option<String>,
    pub acceptance_manifest: Option<String>,
    pub deferred_reason: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportedSidecarCapabilities {
    pub source_project: String,
    pub source_readiness_status: String,
    pub source_gate_status: String,
    pub source_generated_at: Option<String>,
    pub channels: Vec<ImportedSidecarChannel>,
    pub mvp_channels: Vec<String>,
    pub deferred_channels: Vec<String>,
    pub external_call_performed_by_importer: bool,
    pub source_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiiSidecarPreflightRequest {
    pub book: String,
    pub endpoint_name: String,
    pub certificate_secret_ref: String,
    pub sidecar_url: Option<String>,
    pub require_soap_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SiiSidecarPreflight {
    pub ready: bool,
    pub status: String,
    pub sidecar_url: String,
    pub sidecar_endpoint: String,
    pub book: String,
    pub endpoint_name: String,
    pub wsdl_name: Option<String>,
    pub wsdl_sha256: Option<String>,
    pub operation_available: bool,
    pub endpoint_allowed: bool,
    pub transport_enabled: bool,
    pub soap_calls_enabled: bool,
    pub certificate_loaded: bool,
    pub certificate_source: Option<String>,
    pub certificate_days_remaining: Option<i64>,
    pub sidecar_call_performed: bool,
    pub external_call_performed: bool,
    pub blockers: Vec<String>,
    pub warnings: Vec<String>,
    pub source_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketBaiSidecarPreflightRequest {
    pub territorio: String,
    pub endpoint: String,
    pub certificate_secret_ref: String,
    pub sidecar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketBaiSidecarPreflight {
    pub ready: bool,
    pub status: String,
    pub sidecar_url: String,
    pub sidecar_endpoint: String,
    pub territorio: String,
    pub endpoint: String,
    pub xsd_name: Option<String>,
    pub xsd_sha256: Option<String>,
    pub xsd_available: bool,
    pub endpoint_allowed: bool,
    pub submission_enabled: bool,
    pub certificate_loaded: bool,
    pub certificate_source: Option<String>,
    pub certificate_days_remaining: Option<i64>,
    pub sidecar_call_performed: bool,
    pub external_call_performed: bool,
    pub blockers: Vec<String>,
    pub warnings: Vec<String>,
    pub source_refs: Vec<String>,
}

pub fn imported_sidecar_capabilities() -> ImportedSidecarCapabilities {
    let root = sidecar_root();
    let readiness_path = root
        .join("tests")
        .join("regulatory")
        .join("evidence")
        .join("latest-external-readiness.json");
    let gate_path = root
        .join("tests")
        .join("regulatory")
        .join("evidence")
        .join("latest-regulatory-gate.json");

    let readiness = read_json(&readiness_path);
    let gate = read_json(&gate_path);

    let source_readiness_status = readiness
        .as_ref()
        .and_then(|value| value.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("missing")
        .to_string();
    let source_gate_status = gate
        .as_ref()
        .and_then(|value| value.get("gate_status"))
        .and_then(Value::as_str)
        .unwrap_or("missing")
        .to_string();
    let source_generated_at = readiness
        .as_ref()
        .and_then(|value| value.get("generated_at"))
        .and_then(Value::as_str)
        .map(ToString::to_string);

    let mut channels = Vec::new();
    for channel in ["verifactu", "sii", "face", "ticketbai"] {
        channels.push(import_channel(readiness.as_ref(), channel, &root));
    }
    let mvp_channels = channels
        .iter()
        .filter(|channel| channel.mvp_importable)
        .map(|channel| channel.channel.clone())
        .collect::<Vec<_>>();
    let deferred_channels = channels
        .iter()
        .filter(|channel| !channel.mvp_importable)
        .map(|channel| channel.channel.clone())
        .collect::<Vec<_>>();
    let mut source_refs = vec![
        readiness_path.to_string_lossy().to_string(),
        gate_path.to_string_lossy().to_string(),
        "C:\\07 DESARROLLO\\apps\\contapro\\services\\sidecar\\REGULATORY_EVIDENCE_MATRIX.md"
            .to_string(),
    ];
    for channel in &channels {
        if let Some(manifest) = &channel.acceptance_manifest {
            source_refs.push(manifest.clone());
        }
    }

    ImportedSidecarCapabilities {
        source_project: "contapro-sidecar".to_string(),
        source_readiness_status,
        source_gate_status,
        source_generated_at,
        channels,
        mvp_channels,
        deferred_channels,
        external_call_performed_by_importer: false,
        source_refs,
        notes: vec![
            "MVP import imports only channels with ready status and accepted redacted manifest: VERI*FACTU and SII.".to_string(),
            "FACe and TicketBAI are tracked as deferred evidence only, not MVP runtime scope.".to_string(),
            "Imported as local evidence only; CumpleFactura performs no AEAT/SII/FACe/foral network call during import.".to_string(),
            "Private certificates, passwords and raw fiscal payloads are not imported.".to_string(),
        ],
    }
}

pub fn sii_preflight(request: SiiSidecarPreflightRequest) -> Result<SiiSidecarPreflight, String> {
    require("book", &request.book)?;
    require("endpoint_name", &request.endpoint_name)?;

    let sidecar_url = request
        .sidecar_url
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_SII_PREFLIGHT_URL.to_string());
    let enabled = std::env::var(ENABLE_SII_PREFLIGHT_ENV).unwrap_or_default() == "1";
    let secret = sidecar_secret();
    let mut result = base_sii_result(&request, &sidecar_url);

    if !valid_certificate_ref(&request.certificate_secret_ref) {
        result
            .blockers
            .push("certificate_secret_ref_must_use_vault_certs_prefix".to_string());
    }
    if !allowed_url(&sidecar_url, SII_PREFLIGHT_PATH) {
        result
            .blockers
            .push("sidecar_url_must_be_local_sii_submit_preflight".to_string());
    }
    if !enabled {
        result
            .blockers
            .push(format!("{ENABLE_SII_PREFLIGHT_ENV}_not_enabled"));
    }
    if secret.trim().is_empty() {
        result
            .blockers
            .push("sidecar_shared_secret_required".to_string());
    }
    if !result.blockers.is_empty() {
        return Ok(result);
    }

    let payload = SiiSidecarPayload {
        book: request.book.clone(),
        endpoint_name: request.endpoint_name.clone(),
        certificate_secret_ref: request.certificate_secret_ref.clone(),
        require_soap_enabled: request.require_soap_enabled.unwrap_or(true),
    };
    let body =
        serde_json::to_vec(&payload).map_err(|err| format!("sidecar payload error: {err}"))?;
    let response: SiiSidecarResponse =
        post_json_local_sidecar(&sidecar_url, SII_PREFLIGHT_PATH, &secret, &body)?;
    if response.external_call_performed {
        return Err("sidecar_reported_external_call".to_string());
    }

    result.sidecar_call_performed = true;
    result.status = response.status;
    result.ready = result.status == "ready" && response.blockers.is_empty();
    result.book = response.book;
    result.endpoint_name = response.endpoint_name;
    result.wsdl_name = response.wsdl_name;
    result.wsdl_sha256 = response.wsdl_sha256;
    result.operation_available = response.operation_available;
    result.endpoint_allowed = response.endpoint_allowed;
    result.transport_enabled = response.transport_enabled;
    result.soap_calls_enabled = response.soap_calls_enabled;
    result.certificate_loaded = response.certificate_loaded;
    result.certificate_source = response.certificate_source;
    result.certificate_days_remaining = response.certificate_days_remaining;
    result.blockers = response.blockers;
    result.warnings.extend(response.warnings);
    Ok(result)
}

pub fn ticketbai_preflight(
    request: TicketBaiSidecarPreflightRequest,
) -> Result<TicketBaiSidecarPreflight, String> {
    require("territorio", &request.territorio)?;
    require("endpoint", &request.endpoint)?;

    let sidecar_url = request
        .sidecar_url
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_TICKETBAI_PREFLIGHT_URL.to_string());
    let enabled = std::env::var(ENABLE_TICKETBAI_PREFLIGHT_ENV).unwrap_or_default() == "1";
    let secret = sidecar_secret();
    let mut result = base_ticketbai_result(&request, &sidecar_url);

    if !valid_certificate_ref(&request.certificate_secret_ref) {
        result
            .blockers
            .push("certificate_secret_ref_must_use_vault_certs_prefix".to_string());
    }
    if !request.endpoint.trim().starts_with("https://") {
        result
            .blockers
            .push("ticketbai_https_endpoint_required".to_string());
    }
    if !allowed_url(&sidecar_url, TICKETBAI_PREFLIGHT_PATH) {
        result
            .blockers
            .push("sidecar_url_must_be_local_ticketbai_submit_preflight".to_string());
    }
    if !enabled {
        result
            .blockers
            .push(format!("{ENABLE_TICKETBAI_PREFLIGHT_ENV}_not_enabled"));
    }
    if secret.trim().is_empty() {
        result
            .blockers
            .push("sidecar_shared_secret_required".to_string());
    }
    if !result.blockers.is_empty() {
        return Ok(result);
    }

    let payload = TicketBaiSidecarPayload {
        territorio: request.territorio.clone(),
        endpoint: request.endpoint.clone(),
        certificate_secret_ref: request.certificate_secret_ref.clone(),
    };
    let body =
        serde_json::to_vec(&payload).map_err(|err| format!("sidecar payload error: {err}"))?;
    let response: TicketBaiSidecarResponse =
        post_json_local_sidecar(&sidecar_url, TICKETBAI_PREFLIGHT_PATH, &secret, &body)?;
    if response.external_call_performed {
        return Err("sidecar_reported_external_call".to_string());
    }

    result.sidecar_call_performed = true;
    result.status = response.status;
    result.ready = result.status == "ready" && response.blockers.is_empty();
    result.territorio = response.territorio;
    result.endpoint = response.endpoint;
    result.xsd_name = response.xsd_name;
    result.xsd_sha256 = response.xsd_sha256;
    result.xsd_available = response.xsd_available;
    result.endpoint_allowed = response.endpoint_allowed;
    result.submission_enabled = response.submission_enabled;
    result.certificate_loaded = response.certificate_loaded;
    result.certificate_source = response.certificate_source;
    result.certificate_days_remaining = response.certificate_days_remaining;
    result.blockers = response.blockers;
    result.warnings.extend(response.warnings);
    Ok(result)
}

fn import_channel(
    readiness: Option<&Value>,
    channel: &str,
    root: &PathBuf,
) -> ImportedSidecarChannel {
    let section = readiness
        .and_then(|value| value.get("sections"))
        .and_then(Value::as_array)
        .and_then(|sections| {
            sections
                .iter()
                .find(|section| section.get("id").and_then(Value::as_str) == Some(channel))
        });

    let status = section
        .and_then(|value| value.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("missing")
        .to_string();
    let blockers = section
        .and_then(|value| value.get("blockers"))
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    item.get("id")
                        .and_then(Value::as_str)
                        .or_else(|| item.get("detail").and_then(Value::as_str))
                        .map(ToString::to_string)
                })
                .collect()
        })
        .unwrap_or_default();
    let manifest_path = acceptance_manifest_path(root, channel);
    let manifest = manifest_path.as_ref().and_then(read_json);
    let acceptance_status = manifest
        .as_ref()
        .and_then(|value| value.get("status"))
        .and_then(Value::as_str)
        .map(ToString::to_string);
    let acceptance_validation_status = manifest
        .as_ref()
        .and_then(|value| value.get("validation_status"))
        .and_then(Value::as_str)
        .map(ToString::to_string);
    let services_include_channel = manifest
        .as_ref()
        .and_then(|value| value.get("services"))
        .and_then(Value::as_array)
        .map(|services| {
            services
                .iter()
                .any(|service| service.as_str() == Some(channel))
        })
        .unwrap_or(false);
    let accepted = acceptance_status.as_deref() == Some("accepted")
        && acceptance_validation_status.as_deref() == Some("passed")
        && services_include_channel;
    let mvp_importable = matches!(channel, "verifactu" | "sii") && status == "ready" && accepted;

    ImportedSidecarChannel {
        channel: channel.to_string(),
        status,
        source: "contapro-sidecar latest-external-readiness".to_string(),
        blockers,
        mvp_importable,
        acceptance_status,
        acceptance_validation_status,
        acceptance_manifest: manifest_path.map(|path| path.to_string_lossy().to_string()),
        deferred_reason: if mvp_importable {
            None
        } else {
            Some(deferred_reason(channel).to_string())
        },
        notes: channel_notes(channel),
    }
}

fn acceptance_manifest_path(root: &PathBuf, channel: &str) -> Option<PathBuf> {
    match channel {
        "verifactu" | "sii" => Some(
            root.join("tests")
                .join("regulatory")
                .join("evidence")
                .join(channel)
                .join("acceptance-manifest.json"),
        ),
        _ => None,
    }
}

fn deferred_reason(channel: &str) -> &'static str {
    match channel {
        "face" => "blocked_by_external_sp_certificate_registration",
        "ticketbai" => "blocked_by_foral_signature_policy_software_registration_and_acceptance",
        "verifactu" | "sii" => "ready_status_or_accepted_manifest_missing",
        _ => "not_in_mvp_scope",
    }
}

fn channel_notes(channel: &str) -> Vec<String> {
    match channel {
        "verifactu" => vec!["ContaPro sidecar has real VERI*FACTU readiness evidence when vault, flags and accepted manifest are present.".to_string()],
        "sii" => vec!["ContaPro sidecar has SII WSDL/transport/readiness and accepted preproduction evidence recorded locally.".to_string()],
        "face" => vec!["FACe code exists but remains blocked by external certificate/provider registration evidence.".to_string()],
        "ticketbai" => vec!["TicketBAI transport boundary exists; homologation remains blocked by foral acceptance/licensing evidence.".to_string()],
        _ => Vec::new(),
    }
}

fn base_sii_result(request: &SiiSidecarPreflightRequest, sidecar_url: &str) -> SiiSidecarPreflight {
    SiiSidecarPreflight {
        ready: false,
        status: "blocked".to_string(),
        sidecar_url: sidecar_url.to_string(),
        sidecar_endpoint: SII_PREFLIGHT_PATH.to_string(),
        book: request.book.clone(),
        endpoint_name: request.endpoint_name.clone(),
        wsdl_name: None,
        wsdl_sha256: None,
        operation_available: false,
        endpoint_allowed: false,
        transport_enabled: false,
        soap_calls_enabled: false,
        certificate_loaded: false,
        certificate_source: None,
        certificate_days_remaining: None,
        sidecar_call_performed: false,
        external_call_performed: false,
        blockers: Vec::new(),
        warnings: vec![
            "SII preflight only; no AEAT SII SOAP request is performed by CumpleFactura.".to_string(),
            "The local sidecar checks WSDL, flags and vault readiness when explicitly enabled.".to_string(),
        ],
        source_refs: vec![
            "C:\\07 DESARROLLO\\apps\\contapro\\services\\sidecar\\app\\sii\\router.py::/sii/submit-preflight".to_string(),
            "C:\\07 DESARROLLO\\apps\\contapro\\services\\sidecar\\app\\sii\\preflight.py".to_string(),
        ],
    }
}

fn base_ticketbai_result(
    request: &TicketBaiSidecarPreflightRequest,
    sidecar_url: &str,
) -> TicketBaiSidecarPreflight {
    TicketBaiSidecarPreflight {
        ready: false,
        status: "blocked".to_string(),
        sidecar_url: sidecar_url.to_string(),
        sidecar_endpoint: TICKETBAI_PREFLIGHT_PATH.to_string(),
        territorio: request.territorio.clone(),
        endpoint: request.endpoint.clone(),
        xsd_name: None,
        xsd_sha256: None,
        xsd_available: false,
        endpoint_allowed: false,
        submission_enabled: false,
        certificate_loaded: false,
        certificate_source: None,
        certificate_days_remaining: None,
        sidecar_call_performed: false,
        external_call_performed: false,
        blockers: Vec::new(),
        warnings: vec![
            "TicketBAI preflight only; no foral submission is performed by CumpleFactura.".to_string(),
            "ContaPro sidecar still marks TicketBAI externally blocked until foral evidence is supplied.".to_string(),
        ],
        source_refs: vec![
            "C:\\07 DESARROLLO\\apps\\contapro\\services\\sidecar\\app\\ticketbai\\router.py::/ticketbai/submit-preflight".to_string(),
            "C:\\07 DESARROLLO\\apps\\contapro\\services\\sidecar\\app\\ticketbai\\preflight.py".to_string(),
        ],
    }
}

fn sidecar_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("contapro")
        .join("services")
        .join("sidecar")
}

fn read_json(path: &PathBuf) -> Option<Value> {
    fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
}

fn valid_certificate_ref(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.starts_with(CERT_REF_PREFIX) && trimmed.len() > CERT_REF_PREFIX.len()
}

fn sidecar_secret() -> String {
    std::env::var(SIDECAR_SHARED_SECRET_ENV)
        .or_else(|_| std::env::var(SIDECAR_SHARED_SECRET_FALLBACK_ENV))
        .unwrap_or_default()
}

fn allowed_url(value: &str, expected_path: &str) -> bool {
    parse_local_http_url(value, expected_path).is_ok()
}

#[derive(Serialize)]
struct SiiSidecarPayload {
    book: String,
    endpoint_name: String,
    certificate_secret_ref: String,
    require_soap_enabled: bool,
}

#[derive(Deserialize)]
struct SiiSidecarResponse {
    status: String,
    #[allow(dead_code)]
    profile: String,
    book: String,
    wsdl_name: Option<String>,
    wsdl_sha256: Option<String>,
    endpoint_name: String,
    operation_available: bool,
    endpoint_allowed: bool,
    transport_enabled: bool,
    soap_calls_enabled: bool,
    certificate_loaded: bool,
    certificate_source: Option<String>,
    #[allow(dead_code)]
    certificate_sha256: Option<String>,
    #[allow(dead_code)]
    certificate_not_after: Option<String>,
    certificate_days_remaining: Option<i64>,
    blockers: Vec<String>,
    warnings: Vec<String>,
    external_call_performed: bool,
}

#[derive(Serialize)]
struct TicketBaiSidecarPayload {
    territorio: String,
    endpoint: String,
    certificate_secret_ref: String,
}

#[derive(Deserialize)]
struct TicketBaiSidecarResponse {
    status: String,
    territorio: String,
    endpoint: String,
    xsd_name: Option<String>,
    xsd_sha256: Option<String>,
    xsd_available: bool,
    endpoint_allowed: bool,
    submission_enabled: bool,
    certificate_loaded: bool,
    certificate_source: Option<String>,
    #[allow(dead_code)]
    certificate_sha256: Option<String>,
    #[allow(dead_code)]
    certificate_not_after: Option<String>,
    certificate_days_remaining: Option<i64>,
    blockers: Vec<String>,
    warnings: Vec<String>,
    external_call_performed: bool,
}

struct LocalHttpTarget {
    host: String,
    port: u16,
    path: String,
}

fn post_json_local_sidecar<T: DeserializeOwned>(
    url: &str,
    expected_path: &str,
    shared_secret: &str,
    body: &[u8],
) -> Result<T, String> {
    let target = parse_local_http_url(url, expected_path)?;
    let signature = hmac_sha256_hex(shared_secret.as_bytes(), body)?;
    let mut stream = TcpStream::connect((target.host.as_str(), target.port))
        .map_err(|err| format!("sidecar connection error: {err}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(20)))
        .map_err(|err| format!("sidecar read timeout error: {err}"))?;
    stream
        .set_write_timeout(Some(Duration::from_secs(20)))
        .map_err(|err| format!("sidecar write timeout error: {err}"))?;

    let request = format!(
        "POST {} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nx-correlation-id: {}\r\nx-idempotency-key: {}\r\nx-signature: {}\r\nConnection: close\r\n\r\n",
        target.path,
        target.host,
        target.port,
        body.len(),
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4(),
        signature
    );
    stream
        .write_all(request.as_bytes())
        .and_then(|_| stream.write_all(body))
        .map_err(|err| format!("sidecar request error: {err}"))?;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|err| format!("sidecar response read error: {err}"))?;
    parse_http_json_response(&response)
}

fn parse_local_http_url(url: &str, expected_path: &str) -> Result<LocalHttpTarget, String> {
    let rest = url
        .strip_prefix("http://")
        .ok_or_else(|| "sidecar_url_must_use_http_localhost".to_string())?;
    let (host_port, path) = rest
        .split_once('/')
        .ok_or_else(|| "sidecar_url_missing_path".to_string())?;
    let (host, port_text) = host_port
        .split_once(':')
        .ok_or_else(|| "sidecar_url_missing_port".to_string())?;
    if host != "127.0.0.1" && host != "localhost" {
        return Err("sidecar_url_host_must_be_local".to_string());
    }
    let path = format!("/{path}");
    if path != expected_path {
        return Err(format!(
            "sidecar_url_path_must_be_{}",
            expected_path.trim_start_matches('/').replace('/', "_")
        ));
    }
    let port = port_text
        .parse::<u16>()
        .map_err(|_| "sidecar_url_invalid_port".to_string())?;
    Ok(LocalHttpTarget {
        host: host.to_string(),
        port,
        path,
    })
}

fn parse_http_json_response<T: DeserializeOwned>(response: &[u8]) -> Result<T, String> {
    let raw = String::from_utf8_lossy(response);
    let (headers, body) = raw
        .split_once("\r\n\r\n")
        .ok_or_else(|| "sidecar response missing body".to_string())?;
    let status_code = headers
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .ok_or_else(|| "sidecar_missing_status_code".to_string())?
        .parse::<u16>()
        .map_err(|_| "sidecar_invalid_status_code".to_string())?;
    if !(200..300).contains(&status_code) {
        return Err(format!("sidecar_http_status_{status_code}"));
    }
    serde_json::from_str(body).map_err(|err| format!("sidecar response JSON error: {err}"))
}

fn hmac_sha256_hex(key: &[u8], body: &[u8]) -> Result<String, String> {
    let mut mac =
        HmacSha256::new_from_slice(key).map_err(|_| "sidecar_hmac_key_invalid".to_string())?;
    mac.update(body);
    Ok(hex::encode(mac.finalize().into_bytes()))
}

fn require(field: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{field} is required"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn imports_channel_statuses_from_contapro_readiness_shape() {
        let readiness = json!({
            "sections": [
                {"id": "verifactu", "status": "ready", "blockers": []},
                {"id": "sii", "status": "ready", "blockers": []},
                {"id": "face", "status": "blocked", "blockers": [{"id": "face_acceptance_manifest_validated"}]},
                {"id": "ticketbai", "status": "blocked", "blockers": [{"id": "ticketbai_acceptance_manifest_validated"}]}
            ]
        });
        let root = PathBuf::from("missing-contapro-sidecar");

        assert_eq!(
            import_channel(Some(&readiness), "sii", &root).status,
            "ready"
        );
        assert_eq!(
            import_channel(Some(&readiness), "ticketbai", &root).blockers,
            vec!["ticketbai_acceptance_manifest_validated".to_string()]
        );
        assert!(!import_channel(Some(&readiness), "ticketbai", &root).mvp_importable);
    }

    #[test]
    fn sii_preflight_blocks_before_any_sidecar_call_without_flag_or_secret() {
        let result = sii_preflight(SiiSidecarPreflightRequest {
            book: "issued".to_string(),
            endpoint_name: "SuministroFactEmitidasPruebas".to_string(),
            certificate_secret_ref: "vault://certs/prod".to_string(),
            sidecar_url: Some(DEFAULT_SII_PREFLIGHT_URL.to_string()),
            require_soap_enabled: Some(true),
        })
        .expect("preflight");

        assert_eq!(result.status, "blocked");
        assert!(!result.sidecar_call_performed);
        assert!(!result.external_call_performed);
        assert!(result
            .blockers
            .contains(&format!("{ENABLE_SII_PREFLIGHT_ENV}_not_enabled")));
    }

    #[test]
    fn ticketbai_preflight_blocks_non_https_endpoint_before_sidecar_call() {
        let result = ticketbai_preflight(TicketBaiSidecarPreflightRequest {
            territorio: "bizkaia".to_string(),
            endpoint: "http://foral.example.test/ticketbai".to_string(),
            certificate_secret_ref: "vault://certs/prod".to_string(),
            sidecar_url: Some(DEFAULT_TICKETBAI_PREFLIGHT_URL.to_string()),
        })
        .expect("preflight");

        assert_eq!(result.status, "blocked");
        assert!(!result.sidecar_call_performed);
        assert!(result
            .blockers
            .contains(&"ticketbai_https_endpoint_required".to_string()));
    }
}
