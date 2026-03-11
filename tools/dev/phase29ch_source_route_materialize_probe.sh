#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
BIN="${BIN:-target/release/hakorune}"

if [[ ! -x "$BIN" ]]; then
  echo "[FAIL] missing bin: $BIN" >&2
  exit 2
fi

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

helper_src="$tmp_dir/phase29ch_source_route_materialize_probe.hako"
helper_bin="$tmp_dir/phase29ch_source_route_materialize_probe"

python3 - "$helper_src" <<'PY'
import json
import pathlib
import sys

helper_path = pathlib.Path(sys.argv[1])
source_text = (
    "static box Main {\n"
    "  main() {\n"
    "    print(42)\n"
    "    return 0\n"
    "  }\n"
    "}\n"
)
source_literal = json.dumps(source_text, ensure_ascii=False)
helper_path.write_text(
    "using lang.mir.builder.MirBuilderBox as MirBuilderBox\n"
    "using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox\n\n"
    "static box Main {\n"
    "  main() {\n"
    f"    local source_text = {source_literal}\n"
    "    local mir = MirBuilderBox.emit_from_source_v0(source_text, null)\n"
    "    local mir_kind = BoxTypeInspectorBox.kind(mir)\n"
    "    local mir_desc = BoxTypeInspectorBox.describe(mir)\n"
    "    local mir_text = \"\" + mir\n"
    "    if mir == null { print(\"MIR_NULL\") } else { print(\"MIR_NONNULL\") }\n"
    "    if mir == \"\" { print(\"MIR_EMPTY\") } else { print(\"MIR_NONEMPTY\") }\n"
    "    if mir_text == \"\" { print(\"TEXT_EMPTY\") } else { print(\"TEXT_NONEMPTY\") }\n"
    "    if mir_text.length() > 0 { print(\"LEN_POS\") } else { print(\"LEN_NONPOS\") }\n"
    "    if mir_text.substring(0, 1) == \"{\" { print(\"HEAD_OK\") } else { print(\"HEAD_BAD\") }\n"
    "    if mir_text.indexOf(\"functions\") >= 0 { print(\"IDX_OK\") } else { print(\"IDX_BAD\") }\n"
    "    print(\"K=\" + mir_kind)\n"
    "    print(\"D=\" + mir_desc)\n"
    "    print(\"T=\" + mir_text)\n"
    "    return 0\n"
    "  }\n"
    "}\n",
    encoding="utf-8",
)
PY

NYASH_BIN="$BIN" bash "$ROOT/tools/selfhost_exe_stageb.sh" "$helper_src" -o "$helper_bin" >/dev/null

out="$tmp_dir/materialize.out"
NYASH_NYRT_SILENT_RESULT="${NYASH_NYRT_SILENT_RESULT:-1}" \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
NYASH_FILEBOX_MODE="${NYASH_FILEBOX_MODE:-core-ro}" \
"$helper_bin" >"$out" 2>&1

echo "[source-route-materialize] bin=$BIN"
echo "[source-route-materialize] helper-build=compiled-artifact post-call materialization probe"
echo "[source-route-materialize] note=diagnostics-only (host-compiled helper, not selfhost authority evidence)"
sed -n '1,20p' "$out"
