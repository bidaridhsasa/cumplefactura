/**
 * Declaración responsable del SIF (RRSIF art. 13; Orden HAC/1177/2024 art. 15.3).
 * Única fuente: `docs/legal/DECLARACION_RESPONSABLE_SIF.md`, incluido en el bundle al compilar
 * (Vite `?raw`). De ese documento se toma el texto entre las dos reglas `---` de la sección 2
 * y los datos de las letras a), c), h) e i). Si el formato cambia, se muestra el documento
 * entero y la vista lo marca como no definitivo.
 */
import raw from "../../docs/legal/DECLARACION_RESPONSABLE_SIF.md?raw";

const doc = raw.replace(/\r\n/g, "\n");

/** Texto entre la primera pareja de líneas `---` que siguen a «## 2.»; null si no se encuentra. */
function extractBody(source: string): string | null {
  const start = source.indexOf("\n## 2.");
  if (start < 0) return null;
  const parts = source.slice(start).split(/\n---\n/);
  return parts.length >= 3 ? parts[1].trim() : null;
}

const body = extractBody(doc);

/** Valor de la letra `x)` de la declaración, sin marcas de Markdown ni la aclaración entre paréntesis. */
function field(letter: string): string | null {
  const match = (body ?? "").match(new RegExp(`^${letter}\\) \\*\\*[^*]+?:?\\*\\*:? (.+)$`, "m"));
  return match ? plain(match[1]).split(" (")[0].replace(/\.$/, "") : null;
}

/** Quita negritas, cursivas, código y escapes de Markdown. */
export function plain(text: string): string {
  return text.replace(/\*\*|`/g, "").replace(/(^|[^\\])\*/g, "$1").replace(/\\\*/g, "*");
}

export const declaracion = {
  /** Texto a mostrar: la declaración, o el documento completo si no se pudo extraer. */
  text: body ?? doc.trim(),
  nombre: field("a"),
  version: field("c"),
  productor: field("h"),
  nif: field("i"),
  /** No definitivo: borrador, datos PENDIENTE o formato no reconocido. */
  provisional: body === null || /BORRADOR|PENDIENTE/.test(doc),
};
