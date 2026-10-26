# CumpleFactura — fase 1 en curso

Rama: `ux/cumplefactura-fase-1`, desde main a2d429c. No fusionar. El trabajo continúa repo por repo con la UI compartida vendorizada temporalmente; la publicación privada ya no bloquea la implementación local.

## Implementado

- App dividido en shared/types, helpers, controlador de estado y features invoices/customers/products/verifactu/settings/onboarding.
- Shell BDR y nueve áreas por hash; documentos comerciales identificados como todavía no disponibles hasta fase 4.
- Tokens, fuentes locales, icono BDR y colores semánticos desde @bdr/ui.
- Dependencia temporal `file:vendor/emprendurias-bdr-ui-0.1.0.tgz`, build verificado del paquete privado pendiente de publicación. Sustituirla después por `npm:@emprendurias-bdr/ui@0.1.0`. No se publicó nada en npmjs.
- Preferencias de apariencia controladas por el usuario, con carga inicial protegida ante cambios del usuario y escrituras serializadas. Dos comandos nativos guardan JSON bajo ui_preferences en la tabla existente, sin migración ni alteración de los ajustes fiscales.
- La web sin backend conserva una preferencia temporal e informa de la limitación; no se inventan sesión de cuenta ni capacidad de emisión web.
- Sin estados de conexión, envío o aceptación inventados; aviso explícito de cola aún no disponible.
- Contrato de UI inspecciona ahora todos los módulos, preservando sus checks originales. 39 invocaciones nativas y formas de argumentos coinciden con la línea base; la revisión confirmó 121 declaraciones originales sin cambios semánticos, incluidas las monetarias.

## Verificación

Build, contrato de UI y smoke original con mock Tauri pasan. Comprobación de rutas: el salto al contenido no cambia la pantalla. Se capturan y analizan nueve áreas en claro/oscuro a 1024×700 y 1440×900; los resultados exactos están en browser-results.json. No confundir la inspección del mock con pruebas nativas o envío AEAT.

La revisión encontró y se corrigieron pérdida de mensajes en rutas secundarias, salto al contenido interpretado como ruta, contraste de enlaces Menta sobre blanco, falta de nombres de campos, carrera de preferencias y densidad sin efecto en listas. Los registros de validación conservan fallos iniciales; no se filtran ni se presentan como verdes.

## Bloqueos para cerrar la fase

- `npm run validate:regulatory` no arranca: `powershell: not found`.
- No están instalados cargo/rustc ni el runtime nativo Windows: nuevo test de preferencias SQLite y tauri:dev no ejecutados. Cambios nativos no se declaran validados.
- Publicación privada @emprendurias-bdr/ui pendiente; el consumidor usa la instantánea local para ser reproducible.
- Persistencia de cuenta web y backend web quedan pendientes; no simularla con almacenamiento de sesión.
- Este PR debe mantenerse en borrador hasta verificar los gates exigidos y corregir cualquier resultado pendiente.

Resultado final automatizado: 36 casos de nueve áreas a 1024×700/1440×900 en claro y oscuro, cero violaciones axe detectadas; build, contrato y smoke con mock pasan. Los tres selectores de rectificativa y campos técnicos mantienen ahora nombres visibles. El contrato normaliza únicamente espacios en sus guards para tolerar formato JSX multilínea, sin quitar comprobaciones.
