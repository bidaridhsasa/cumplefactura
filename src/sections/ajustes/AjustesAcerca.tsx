/**
 * Ajustes › Acerca de: identificación del sistema informático (nombre, versión, fabricante y NIF)
 * y texto completo de la declaración responsable (Orden HAC/1177/2024 art. 15.3). Todo sale de
 * `src/legal/declaracion.ts`, que lee el documento legal al compilar. Si es borrador o tiene datos
 * PENDIENTE, un aviso lo dice antes del texto. Solo lectura, sin estado.
 */
import { Fragment } from "react";
import { t } from "../../i18n/es";
import { declaracion, plain } from "../../legal/declaracion";
import { Callout } from "../../ui/Callout";
import { Card } from "../../ui/Card";

/** Negritas `**…**` como <strong>; el resto, texto plano. */
function Inline({ text }: { text: string }) {
  return (
    <>
      {text.split("**").map((part, i) => (i % 2 ? <strong key={i}>{plain(part)}</strong> : <Fragment key={i}>{plain(part)}</Fragment>))}
    </>
  );
}

/** Bloques separados por línea en blanco; dentro, las líneas «- » seguidas forman una lista y el resto párrafos. */
function Declaracion({ text }: { text: string }) {
  const blocks: { list: boolean; lines: string[] }[] = [];
  for (const block of text.split(/\n{2,}/)) {
    let current: { list: boolean; lines: string[] } | null = null;
    for (const line of block.split("\n")) {
      const list = line.startsWith("- ");
      if (!current || current.list !== list) blocks.push((current = { list, lines: [] }));
      current.lines.push(list ? line.slice(2) : line);
    }
  }
  return (
    <div className="cf-prose">
      {blocks.map((block, i) =>
        block.list ? (
          <ul key={i}>
            {block.lines.map((line, j) => (
              <li key={j}><Inline text={line} /></li>
            ))}
          </ul>
        ) : (
          <p key={i}><Inline text={block.lines.join(" ")} /></p>
        ),
      )}
    </div>
  );
}

export function AjustesAcerca() {
  const { nombre, version, productor, nif, provisional, text } = declaracion;
  return (
    <>
      {provisional ? <Callout tone="warn" title={t("acerca.borrador")}>{t("acerca.borradorAyuda")}</Callout> : null}
      <Card title={t("acerca.sistema")}>
        <dl className="cf-dl">
          <dt>{t("acerca.nombre")}</dt>
          <dd>{nombre ?? t("acerca.sinDato")}</dd>
          <dt>{t("acerca.version")}</dt>
          <dd className="cf-mono">{version ?? t("acerca.sinDato")}</dd>
          <dt>{t("acerca.fabricante")}</dt>
          <dd>{productor ?? t("acerca.sinDato")}</dd>
          <dt>{t("acerca.nif")}</dt>
          <dd className="cf-mono">{nif ?? t("acerca.sinDato")}</dd>
        </dl>
      </Card>
      <Card title={t("acerca.declaracion")}>
        <Declaracion text={text} />
      </Card>
    </>
  );
}
