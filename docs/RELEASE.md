# Publicar una versión de CumpleFactura

Instalador NSIS para Windows, en español, instalación por usuario (sin permisos de administrador).
Fabricante, identificador y nombre: `docs/legal/FABRICANTE.md`.

## Dónde está cada dato

| Dato | Fuente única |
|---|---|
| Versión | `[workspace.package] version` en `Cargo.toml` (la heredan app y sidecar). `tauri.conf.json` no la fija: Tauri la toma de Cargo. El XML (`SistemaInformatico/Version`) usa `CARGO_PKG_VERSION`. |
| Versión de la UI (npm) | `package.json`: cambiarla a mano a la misma. |
| Identificador | `identifier` en `src-tauri/tauri.conf.json` (`es.saizaragon.cumplefactura`). |
| Instalador | `src-tauri/tauri.bundle.conf.json` (`targets: ["nsis"]`, idioma `Spanish`, `installMode: currentUser`). |

Al subir la versión, actualiza también `<sf:Version>` en `src-tauri/src/services/testdata/*.xml` y en `tests/regulatory/fixtures/verifactu-registro-alta-basic.xml` (los tests comparan el XML byte a byte).

## Pasos

1. **Rama limpia**: `main` actualizado, `git status` vacío.
2. **Versión**: sube `Cargo.toml` y `package.json` (y las fixtures, arriba); `cargo test --workspace`; commit.
3. **Gate regulatorio** en Windows, todo en verde:

   ```powershell
   npm run validate:regulatory
   ```

   Guarda la evidencia `tests/regulatory/evidence/latest-*` que genera.
4. **Comprobaciones**:

   ```powershell
   cargo fmt --all --check
   cargo test --workspace
   npm run build
   npm run validate:ui-contract
   ```

5. **Instalador sin firmar** (prueba):

   ```powershell
   npm run tauri:bundle
   ```

   Sale en `target\release\bundle\nsis\CumpleFactura_<versión>_x64-setup.exe` (o bajo `%CARGO_TARGET_DIR%` si está definido).
6. **Firma de código** (solo en la máquina que publica, nunca en el repo ni en el CI):
   - Importa el certificado de firma (`.pfx`) en el almacén **Personal** del usuario de Windows en ese momento (doble clic o `Import-PfxCertificate`); la contraseña se teclea, no se guarda en ningún fichero ni script.
   - Toma la huella (thumbprint) del certificado y pásala por variable de entorno:

     ```powershell
     $env:CF_SIGN_THUMBPRINT = "<huella>"
     npx tauri build --config src-tauri/tauri.bundle.conf.json --config "{\"bundle\":{\"windows\":{\"certificateThumbprint\":\"$env:CF_SIGN_THUMBPRINT\",\"digestAlgorithm\":\"sha256\",\"timestampUrl\":\"http://timestamp.digicert.com\"}}}"
     ```

     Tauri firma el ejecutable, el sidecar y el instalador con `signtool` (Windows SDK).
   - Comprueba: `Get-AuthenticodeSignature .\CumpleFactura_<versión>_x64-setup.exe` debe decir `Valid`.
   - Al acabar, borra el certificado del almacén y la variable (`Remove-Item Env:CF_SIGN_THUMBPRINT`).
   - Sin certificado, Windows SmartScreen avisa al instalar (H4 del backlog).
7. **Sumas SHA-256**:

   ```powershell
   Get-FileHash -Algorithm SHA256 .\CumpleFactura_<versión>_x64-setup.exe | Format-List
   ```

   Publica la suma junto al instalador (`SHA256SUMS.txt`).
8. **Notas de versión**: qué cambia para el usuario, cambios fiscales (XML, numeración, envío), migraciones de base de datos y problemas conocidos.
9. **Etiqueta git** sobre el commit publicado:

   ```powershell
   git tag -a v<versión> -m "CumpleFactura <versión>"
   git push origin v<versión>
   ```

10. **Distribución**: el instalador y las sumas van a la publicación (GitHub Release o equivalente). **Nunca** se suben binarios ni instaladores al repositorio.

## Datos del usuario

- La base de datos, los certificados (`certs\`), los logos y las exportaciones por defecto viven en `%APPDATA%\es.saizaragon.cumplefactura\`. Las copias de seguridad van donde elige el usuario.
- **Desinstalar no borra los datos**: el desinstalador NSIS de Tauri solo los borra si se marca expresamente la casilla «borrar datos de la aplicación», que sale desmarcada. En las actualizaciones no se ofrece.

### Migración desde versiones con el identificador antiguo

Hasta la 0.1.0 el identificador era `com.local.cumplefactura`. Al cambiarlo, la app nueva arranca con una carpeta de datos vacía. Para conservar los datos de una instalación anterior, **con la app cerrada**:

```powershell
Copy-Item -Recurse "$env:APPDATA\com.local.cumplefactura" "$env:APPDATA\es.saizaragon.cumplefactura"
```

(solo si la carpeta nueva aún no existe). Otra vía: hacer una copia de seguridad desde la versión antigua y restaurarla en la nueva. La carpeta antigua no se borra sola; se puede eliminar a mano tras comprobar los datos.

## Lista de comprobación en un Windows limpio

Máquina o VM sin herramientas de desarrollo. Anota versión, fecha y resultado de cada punto.

- [ ] **Instalar**: el asistente sale en español, no pide administrador, crea acceso en el menú Inicio. Si está firmado, SmartScreen no avisa y el editor es VENDINGSAIZARAGON, S.L.
- [ ] **Primera ejecución**: la app abre sin errores; Ajustes › Avanzado muestra la carpeta `%APPDATA%\es.saizaragon.cumplefactura`; el sidecar arranca (estado de envío disponible).
- [ ] **Emitir**: alta de empresa y cliente, emitir una factura, ver el PDF y el XML; la cadena de huellas verifica.
- [ ] **Enviar a la AEAT de pruebas**: con un certificado de pruebas, enviar la factura al entorno de pruebas y ver la respuesta (aceptada o el error concreto). Nunca producción en esta lista.
- [ ] **Copia y restauración**: exportar copia de seguridad, emitir otra factura, restaurar la copia y comprobar que vuelve al estado anterior.
- [ ] **Desinstalar conservando datos**: desinstalar desde Configuración › Aplicaciones sin marcar la casilla de borrar datos; comprobar que `%APPDATA%\es.saizaragon.cumplefactura` y las copias siguen ahí; reinstalar y ver las facturas.
