#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN_PATH="${HAKORUNE_BIN:-$ROOT_DIR/target/release/hakorune}"

APP_PATH=""
MIR_JSON=""
METHOD=""
TOPN="8"
OUT_PATH=""
REQUIRE_CLEAN=0

usage() {
  cat <<'USAGE'
Usage:
  tools/hako_check/fastpath_explain.sh --mir-json app.mir.json [options]
  tools/hako_check/fastpath_explain.sh --app app.hako [options]
  tools/hako_check/fastpath_explain.sh app.hako [options]

Options:
  --mir-json PATH     Read an existing MIR JSON artifact.
  --app PATH          Emit MIR JSON for a .hako app, then explain it.
  --method NAME       Filter to an exact MIR function name.
  --topn N            Limit top rows in the report. Default: 8.
  --require-clean     Exit non-zero when FastPath obligations failed.
  --out PATH          Write the report to PATH.
  -h, --help          Show this help.

Notes:
  --app requires target/release/hakorune to already exist. This wrapper does not
  build the compiler, rewrite source, choose keepers, or run benchmarks.
USAGE
}

die() {
  echo "fastpath_explain.sh: $*" >&2
  exit 2
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --mir-json)
      [ "$#" -ge 2 ] || die "--mir-json requires a path"
      MIR_JSON="$2"
      shift 2
      ;;
    --app)
      [ "$#" -ge 2 ] || die "--app requires a path"
      APP_PATH="$2"
      shift 2
      ;;
    --method)
      [ "$#" -ge 2 ] || die "--method requires a name"
      METHOD="$2"
      shift 2
      ;;
    --topn)
      [ "$#" -ge 2 ] || die "--topn requires a number"
      TOPN="$2"
      shift 2
      ;;
    --require-clean)
      REQUIRE_CLEAN=1
      shift
      ;;
    --out)
      [ "$#" -ge 2 ] || die "--out requires a path"
      OUT_PATH="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    --*)
      die "unknown option: $1"
      ;;
    *)
      if [ -z "$APP_PATH" ] && [ -z "$MIR_JSON" ]; then
        APP_PATH="$1"
        shift
      else
        die "unexpected positional argument: $1"
      fi
      ;;
  esac
done

if [ -n "$APP_PATH" ] && [ -n "$MIR_JSON" ]; then
  die "choose either --app or --mir-json, not both"
fi
if [ -z "$APP_PATH" ] && [ -z "$MIR_JSON" ]; then
  usage >&2
  exit 2
fi

TMP_DIR=""
cleanup() {
  if [ -n "$TMP_DIR" ]; then
    rm -rf "$TMP_DIR"
  fi
}
trap cleanup EXIT

if [ -n "$APP_PATH" ]; then
  [ -f "$APP_PATH" ] || die "app not found: $APP_PATH"
  [ -x "$BIN_PATH" ] || die "hakorune binary not found or not executable: $BIN_PATH (run cargo build --release --bin hakorune)"
  TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hako_check_fastpath.XXXXXX")"
  MIR_JSON="$TMP_DIR/app.mir.json"
  NYASH_FEATURES="${NYASH_FEATURES:-rune}" \
  NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
    "$BIN_PATH" --backend mir --emit-mir-json "$MIR_JSON" "$APP_PATH" >/dev/null
else
  [ -f "$MIR_JSON" ] || die "MIR JSON not found: $MIR_JSON"
fi

ARGS=(
  python3
  "$ROOT_DIR/tools/hako_check/fastpath_explain.py"
  --mir-json "$MIR_JSON"
  --topn "$TOPN"
)

if [ -n "$METHOD" ]; then
  ARGS+=(--method "$METHOD")
fi
if [ "$REQUIRE_CLEAN" -eq 1 ]; then
  ARGS+=(--require-clean)
fi
if [ -n "$OUT_PATH" ]; then
  ARGS+=(--out "$OUT_PATH")
fi

"${ARGS[@]}"
