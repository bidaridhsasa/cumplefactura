# CumpleFactura: inventario funcional y análisis competitivo

Fecha: 2026-10-21. Fuente: revisión del código local y documentación; referencias remotas sincronizadas y coincidentes con HEAD según docs/workflows/APP_SYNC_2026-10-20.md del workspace. Presencia de código no equivale a prueba funcional ni aceptación fiscal.

## CumpleFactura

Aplicación de escritorio local-first con Tauri 2, React/TypeScript, Rust y SQLite. No es un ERP contable completo. Evidencia principal: `apps/CumpleFactura/src/App.tsx`, `src-tauri/src/lib.rs`, `MODULE_CONTRACT.md`, `README_DEV.md` y `REGULATORY_EVIDENCE_MATRIX.md`.

### Facturación y configuración local

| Funcionalidad | Alcance detectado | Estado | Evidencia |
|---|---|---|---|
| Persistencia local | Base SQLite, migraciones, índices y reglas en Rust | Código | `src-tauri/src/db`, `MODULE_CONTRACT.md` |
| Onboarding | Empresa activa, fiscalidad, primera serie y carpeta de exportación | Código; wizard más completo pendiente | `src/App.tsx`, comandos `app_settings_*` |
| Configuración persistente | Lectura/guardado de ajustes y rutas de exportación | Código | `app_defaults_get`, `app_settings_get/update` |
| Empresas emisoras | Crear, editar y listar emisores | Código; multiempresa avanzada fuera del MVP | `company_create/update/list` |
| Clientes | Crear, editar por ID y listar | Código | `customer_upsert/list` |
| Productos y servicios | Catálogo por empresa; SKU, precio, IVA, IRPF, descuento y activo/inactivo | Código | `product_upsert/list` |
| Uso de producto en factura | Rellenar línea editable desde catálogo | Código | `src/App.tsx` |
| Series | Configurar por empresa/ejercicio: código, año opcional, separador, padding y siguiente número | Código | `company_series_configure/list` |
| Continuidad de numeración | Inicio desde número previo y prevención de colisiones | Código | `invoice_sequences`, `MODULE_CONTRACT.md` |
| Previsualización de número | Consulta del siguiente número sin reservarlo | Código | `invoice_next_number_preview` |
| Editor de factura | Líneas con descripción, cantidad entera, precio, descuento, IVA e IRPF | Código | `src/App.tsx`, dominio Rust |
| Borradores | Creación y persistencia con cálculo de totales en Rust | Código | `invoice_draft_create` |
| Emisión | Validaciones, reserva de número, huella y auditoría local | Código | `invoice_issue` |
| Numeración segura | Rust asigna el número; ignora el propuesto por la UI | Código | `MODULE_CONTRACT.md`, `invoice_issue` |
| Tipos fiscales | Alta F1/F2 y rectificativas R1-R5 en XML cubierto | Código; alcance técnico específico | `MODULE_CONTRACT.md`, servicio VERI*FACTU |
| Rectificativas | Referencia a factura original, causa R1-R5 y método I/S; líneas negativas | Código | `invoice_issue`, `src/App.tsx` |
| Asistente de rectificación | Orienta desde escenario declarado hacia causa/método | Código; no decide la causa jurídica | `rectification_advice`, `REGULATORY_EVIDENCE_MATRIX.md` |
| Estados de factura | Borrador, emitida, rectificada y anulada según flujo | Código | `MODULE_CONTRACT.md`, dominio/DB |
| Inmutabilidad fiscal | Emitidas se rectifican/anulan; registros fiscales append-only | Código | dominio Rust y triggers SQLite |
| Listado y buscador | Filtros por texto, empresa, cliente, año, serie, estado y tipo | Código | `invoice_list`, `src/App.tsx` |
| Paginación | Facturas, auditoría y registros fiscales con limit/offset | Código; sin totales ni vistas guardadas | `MODULE_CONTRACT.md` |
| Datos demo explícitos | Activación manual; rellena formularios sin desbloquear permisos | Código | `src/App.tsx` |

### Documentos, exportación y copias

| Funcionalidad | Alcance detectado | Estado | Evidencia |
|---|---|---|---|
| PDF de factura | Generación local de PDF básico | Código; sin editor avanzado de plantillas | `invoice_export_pdf` |
| JSON de factura | Exportación individual | Código | `invoice_export_json` |
| CSV de facturas | Exportación local | Código; MVP exporta todo aunque contrato acepte filtros | `invoice_export_csv`, `MODULE_CONTRACT.md` |
| Auditoría JSON | Facturas, eventos, registros y verificación de cadena | Código; no certifica cumplimiento | `fiscal_audit_export` |
| Diálogos nativos | Selección de carpetas y archivos PDF/JSON/CSV/XML/backup | Código | `src/App.tsx`, plugin dialog |
| Rutas de exportación | Defaults resueltos por Tauri, sin perfil de Windows hardcodeado | Código | `app_defaults_get` |
| Backup SQLite | Copia manual con VACUUM INTO | Código; no se acredita cifrado integral de backup | `backup_export` |
| Validación de restauración | Comprobación de origen y respuesta validated-only | **Preliminar: no restaura la base activa** | `backup_restore` |

### Auditoría y VERI*FACTU

| Funcionalidad | Alcance detectado | Estado | Evidencia |
|---|---|---|---|
| Auditoría encadenada | Eventos append-only con hash del evento anterior | Código | servicio audit y triggers SQLite |
| Registros fiscales | Altas y anulaciones persistidas, append-only | Código | `fiscal_records`, `MODULE_CONTRACT.md` |
| Huella fiscal | SHA-256 de campos cubiertos de Alta/Anulación | Código; evidencia técnica local | servicio `verifactu`, matriz de evidencias |
| XML de alta | Generación XML borrador desde factura emitida | Código; no implica firma/envío | `verifactu_xml_draft_export` |
| XML de anulación | XML, registro fiscal y cambio a Void | Código; no implica comunicación aceptada por AEAT | `verifactu_xml_anulacion_draft_export` |
| Validación XSD | Comprobación local antes de devolver exportación correcta | Código; estructura no equivale a suficiencia legal | recursos XSD y validador local |
| Encadenamiento XML | PrimerRegistro/RegistroAnterior según cadena fiscal | Código | servicio `verifactu` |
| Verificación de integridad | Recalcula huellas y detecta enlaces rotos, duplicados, alteraciones y XML ausente | Código | `fiscal_chain_verify` |
| Pantalla de auditoría | Incidencias, último hash por empresa y exportación | Código | `src/App.tsx` |
| Preparación de firma | Revalidación XML, SHA-256 y estado prepared_for_signature | Código; no firma por sí mismo | `verifactu_submission_prepare` |
| Preparación VERI*FACTU genérica | Payload inicial | **Preliminar/placeholder** | `fiscal_prepare_verifactu`, contrato |
| Preflight XAdES | Comprueba hash exacto, endpoint local, flag y referencia a certificado | Código; no equivale a firma | `verifactu_xades_sidecar_preflight` |
| Firma XAdES por sidecar | Solicitud local autenticada, XML firmado y estado signed_xades | Condicionado a HMAC, flags, vault y certificado real | `verifactu_xades_sidecar_sign` |
| Preflight de transporte | Allowlist de endpoint, huella, firma y configuración | Código; no realiza SOAP | `verifactu_transport_preflight` |
| Preflight de envío sidecar | Comprueba readiness de WSDL/vault/flags | Condicionado; no envía a AEAT | `verifactu_submit_sidecar_preflight` |
| Solicitud de envío sidecar | Entrega XML firmado a endpoint local invoke-submit | Condicionado; aceptación AEAT no acreditada | `verifactu_submit_sidecar_invoke` |

La matriz declara pruebas de fronteras con sidecar falso y ausencia de transmisión AEAT en el gate. No se presenta esta implementación como cumplimiento homologado ni envío externo validado.

### Otros canales y verificabilidad

| Funcionalidad | Alcance detectado | Estado | Evidencia |
|---|---|---|---|
| Panel de canales ContaPro | Estado importado de VERI*FACTU, SII, FACe y TicketBAI | Código; evidencia local redactada | `fiscal_sidecar_capabilities_get`, `src/App.tsx` |
| Importación de readiness | Solo marca MVP importable VERI*FACTU/SII con manifiestos accepted/passed | Condicionado; no importa certificados ni demuestra servicio activo | herramienta de importación y matriz |
| SII | Preflight local autenticado | **Preliminar: no envío SII completo** | `sii_submit_sidecar_preflight` |
| TicketBAI | Canal diferido y preflight local | **Pendiente/bloqueado para alcance foral real** | `ticketbai_submit_sidecar_preflight` |
| FACe | Visibilidad del estado importado | **Pendiente/bloqueado por alta externa del certificado como proveedor** | matriz y README_DEV |
| Matriz de evidencias | Relaciona promesa, fuente, prueba y límite | Código/documentación | `REGULATORY_EVIDENCE_MATRIX.md` |
| Ledger SHA-256 | Trazabilidad de fuentes y artefactos locales | Código; no es certificación | herramientas `verify-evidence-ledger` |
| Evidencia de aceptación externa | Validación de manifiestos redactados y hashes | Código; sin manifiesto queda not_provided | herramienta `verify-external-acceptance-evidence` |
| Readiness externo | Informe de bloqueos de sidecar, certificados, flags y entorno | Código; puede quedar bloqueado por diseño | herramienta `verify-external-readiness` |
| Pruebas disponibles | Rust, core E2E, navegador con mock Tauri, contrato UI y Tauri desktop | Infraestructura presente; no ejecutada en esta revisión | `README_DEV.md`, `REGULATORY_EVIDENCE_MATRIX.md` |
| Seguridad local | CSP Tauri y escaneo de secretos con salida redactada | Código; no acredita ausencia universal de fallos | `README_DEV.md` |

### Pendientes y fuera de alcance de CumpleFactura

- Restauración que reemplace efectivamente la base activa; el comando actual solo valida.
- Firma y aceptación AEAT reales con certificado autorizado y evidencia externa verificable.
- Envíos SII/forales completos; FACe operativo.
- Conteos totales, tamaños de página configurables y vistas guardadas.
- Mejora del wizard de incorporación y validaciones de negocio del asistente de rectificaciones.
- Facturas recurrentes, importación visual/CSV/Facturae, presupuestos/albaranes, plantillas PDF editables, correo integrado y backup cloud: aparecen como mejoras del brief, no se acreditan como flujo implementado actual.
- Cifrado integral de SQLite/backup: el brief lo propone; la implementación inventariada acredita persistencia/copia SQLite, no cifrado.
- ERP completo, contabilidad completa, RRHH/nóminas, portal cliente, SaaS multiusuario, inventario avanzado, BI complejo y marketplace: fuera del alcance v1 del brief.

## Comparación competitiva actualizada

Fuentes oficiales consultadas el 21/10/2026. Se comparan flujos de usuario, no solo número de botones. Las páginas de los rivales expresan sus promesas comerciales; no se auditó su funcionamiento. «Sin cobertura acreditada» significa que el código revisado no demuestra un flujo terminado.

| Alternativa y cliente típico | Capacidades publicadas | Consecuencia para CumpleFactura |
|---|---|---|
| [Aplicación gratuita AEAT](https://sede.agenciatributaria.gob.es/Sede/ayuda/consultas-informaticas/presentacion-declaraciones-ayuda-tecnica/aplicacion-gratuita-verifactu-aeat.html), muy bajo volumen | Emisión y remisión directa sin límite numérico de facturas; gestión en Sede. No admite facturas simplificadas, ciertos regímenes ni continuar sus registros en otro SIF | No competir por «solo emitir gratis». Ganar por trabajo sin conexión, datos locales, portabilidad y flujo comercial; cerrar aceptación real antes de vender cumplimiento |
| [FacturaDirecta](https://help.facturadirecta.com/es/articles/13538823-api-publica-de-facturadirecta), autónomo/pyme | Facturas, presupuestos, albaranes, API, [recurrencia](https://help.facturadirecta.com/es/articles/13544719-facturas-de-venta-recurrentes), [portal/pago online](https://help.facturadirecta.com/es/articles/16942201-configurar-los-pagos-online-de-tus-facturas) y remesas | Hace completo el ciclo facturar-compartir-cobrar; CumpleFactura cubre más trazabilidad local que gestión de cobro |
| [Billin](https://www.billin.net/wp-content/uploads/pdf-billin/declaracion-responsable-billin.pdf), microempresa | Declara adaptación VERI*FACTU; su [material de producto](https://www.billin.net/wp-content/uploads/2021/02/NdP-TeamSystem-Billin-v4.pdf) incluye facturas recurrentes y pago online | El mensaje de cumplimiento necesita evidencia externa propia; las funciones de cobro son referencia, aunque la fuente de producto citada es anterior y debe verificarse antes de una comparación comercial exhaustiva |
| [Factusol](https://www.sdelsol.com/programa-facturacion-factusol/), pyme de escritorio | Facturae, rectificativas, facturas periódicas, remesas, compras/stock y enlace contable | Tener datos locales o app de escritorio no diferencia por sí solo; CumpleFactura puede diferenciarse por enfoque ligero y prueba visible del rastro fiscal si la UX lo confirma |
| [Holded](https://www.holded.com/es/funcionalidades), empresa creciente | Facturación, bancos, CRM, proyectos, inventario y asesoría conectada | Suite de alcance muy superior; copiarla diluiría la propuesta local de CumpleFactura |
| [Quipu](https://getquipu.com/es/asesorias), autónomo con asesoría | OCR, documental, banco y colaboración en la misma plataforma | Si el comprador necesita asesoría online diaria, CumpleFactura requiere exportación/traspaso fiable o derivarlo a ContaPro; no prometer colaboración que no existe |

### Brechas priorizadas y mejoras concretas

| Prioridad | Situación detectada | Mejora propuesta | Criterio de aceptación |
|---|---|---|---|
| P0 | XML y huella se validan localmente; preflight/firma/invocación por sidecar existen. El gate usa sidecar falso y no demuestra acuse real AEAT, certificado autorizado ni cobertura de aceptación | Cerrar un piloto fiscal autorizado extremo a extremo: firma, transporte, respuesta, rechazo/reintento, acuse y evidencias redactadas. Mostrar estado distinto para preparado, enviado, aceptado y rechazado | Caso real autorizado con XML exacto, respuesta trazable y recuperación de errores; gate de regresión; no se anuncia cumplimiento completo antes de ello |
| P0 | `backup_restore` solo valida; `backup_export` genera copia SQLite. El brief propone cifrado, no acreditado | Restauración guiada y segura: verificación, copia de seguridad previa, importación a ubicación temporal, checksum/integridad, confirmación explícita y rollback. Cifrado de backup con gestión de clave recuperable | Ensayo con datos ficticios restaura factura, serie y cadena fiscal; archivo corrupto deja intacta la base; pérdida de clave y recuperación están documentadas |
| P1 | Existe onboarding, editor y PDF básico; no se acreditó una experiencia completa de entrega al cliente ni personalización de documento | Plantilla PDF con identidad del emisor y campos de pago; compartir mediante diálogo nativo/correo del usuario con registro local de entrega. Diseñar primero flujo sin servicio remoto obligatorio | De alta de empresa a factura entregable en una sesión; PDF pasa revisión visual; acción de compartir deja rastro local sin duplicar emisión |
| P1 | Facturas y filtros existen; no hay motor visible de facturación periódica ni recordatorios de cobro | Plantillas recurrentes que generan borrador revisable por periodo; vencimiento y registro manual de pagos. Envío automático solo tras validar necesidad y requisitos fiscales | La misma ocurrencia nunca emite dos números; pausa y cambio de cliente quedan auditados; cobro parcial actualiza pendiente sin alterar factura emitida |
| P1 | JSON/CSV/PDF/XML exportan; falta flujo de importación y migración desde otro facturador | Importación guiada de emisores, clientes y catálogo, con vista previa y conflictos. Para facturas históricas, conservar identificación de origen y evitar renumeración o reemisión accidental | Dos ejecuciones de importación no crean duplicados; originales se distinguen de facturas emitidas por el SIF actual |
| P1 | La cadena fiscal se verifica, pero el usuario ve varios paneles técnicos de preflight y sidecar | Reducir complejidad visible: tres estados principales «Listo para emitir», «Acción pendiente» y «Revisar incidencia», con detalle técnico desplegable y siguiente paso concreto | Pruebas de usuario: emisor sin formación técnica identifica qué puede hacer y qué bloquea el envío sin interpretar HMAC/WSDL |
| P2 | El sidecar de ContaPro conoce Facturae/FACe; CumpleFactura no acredita un recorrido B2G usable | Si clientes públicos aparecen en pilotos, diseñar Facturae desde factura existente, validación XSD/negocio, firma y seguimiento de estado FACe mediante sidecar | Piloto con caso público autorizado y acuse; sin certificados ni alta de proveedor, la UI informa el bloqueo |
| P2 | El producto no incluye colaboración con asesoría ni banca/portal cliente | Exportación estructurada con contrato versionado y manifiesto de integridad para asesor o ContaPro. Tratar conectividad/pago online como módulo opcional según demanda | Importación de prueba en destino conserva totales, identificadores y origen sin doble alta; se registran desacuerdos |
| P3 | CRM, inventario avanzado, RRHH, contabilidad completa y portal SaaS no son el alcance del producto | Mantener fuera del núcleo; documentar cuándo el cliente debe pasar a ContaPro | Pilotos y entrevistas, no comparación de catálogos, justifican cualquier ampliación |

El núcleo debe seguir funcionando sin red para borradores y emisión local. Una función opcional en línea nunca debe bloquear la consulta de facturas ni ocultar el estado de un envío fiscal pendiente.

### Diferenciación defendible y cómo potenciarla

1. **Control local real.** SQLite, exportaciones y copia local permiten al emisor poseer sus datos. Factusol también es una alternativa de escritorio; la ventaja deberá demostrarse con onboarding, restauración y exportación más sencillos. Medir tiempo hasta primera factura y recuperación de backup.
2. **Trazabilidad fiscal comprensible.** Cadena de huellas, XML XSD y auditoría ya existen. Convertirlos en un historial legible de cada factura: qué se creó, anuló, firmó, envió y aceptó, con documento y respuesta vinculados. No presentarlo como certificación mientras falte aceptación externa.
3. **Menor complejidad para emitir.** Centrar la interfaz en emisor, cliente, serie, borrador, emitir, entregar y consultar. Los controles técnicos siguen disponibles para soporte y auditoría. Medir errores por emisión y necesidad de ayuda en pruebas con autónomos.
4. **Puente voluntario al ERP.** ContaPro puede recibir datos cuando el negocio necesite contabilidad, banca, asesoría o multiempresa avanzada. Es una oportunidad de diseño, no una integración ya implementada. Definir propiedad de numeración, IDs, cadena fiscal, permisos y deduplicación antes de sincronizar productos.
5. **Transparencia comercial.** Publicar una tabla de cobertura por canal y estado probado. El comprador debería poder distinguir facturación local, XML técnico y aceptación AEAT/foral sin leer documentación de desarrolladores.

Posicionamiento propuesto a validar: «Facturación de escritorio sencilla, con datos bajo control del emisor y evidencia fiscal que se puede revisar; conectividad opcional cuando existe una necesidad real». No se ha probado disposición a pagar ni preferencia por instalación local frente a SaaS. Validar con 5–10 usuarios y un piloto con facturas reales autorizadas; comparar tiempo de primera factura, recuperación de backup y comprensión del estado fiscal.

### Fuentes y límites del análisis

- [AEAT: aplicación gratuita y límites](https://sede.agenciatributaria.gob.es/Sede/ayuda/consultas-informaticas/presentacion-declaraciones-ayuda-tecnica/aplicacion-gratuita-verifactu-aeat.html).
- [FacturaDirecta: API/funciones](https://help.facturadirecta.com/es/articles/13538823-api-publica-de-facturadirecta), [recurrencia](https://help.facturadirecta.com/es/articles/13544719-facturas-de-venta-recurrentes), [cobro online](https://help.facturadirecta.com/es/articles/16942201-configurar-los-pagos-online-de-tus-facturas), [remesas](https://help.facturadirecta.com/es/articles/15120945-trabajar-con-remesas-bancarias-sepa).
- [Billin: declaración responsable](https://www.billin.net/wp-content/uploads/pdf-billin/declaracion-responsable-billin.pdf) y [material de funciones](https://www.billin.net/wp-content/uploads/2021/02/NdP-TeamSystem-Billin-v4.pdf); este último no confirma estado actual por su antigüedad.
- [Factusol: funciones](https://www.sdelsol.com/programa-facturacion-factusol/), [Holded: funcionalidades](https://www.holded.com/es/funcionalidades), [Quipu: asesorías](https://getquipu.com/es/asesorias).
- Evidencia propia: `src/App.tsx`, `src-tauri/src/lib.rs`, `src-tauri/src/db`, `src-tauri/src/services`, `MODULE_CONTRACT.md`, `README_DEV.md` y `REGULATORY_EVIDENCE_MATRIX.md`. Las recomendaciones son hipótesis; no se ejecutaron pruebas funcionales ni envíos fiscales en esta revisión.
