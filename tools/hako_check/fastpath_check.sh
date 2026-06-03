#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN_PATH="${HAKORUNE_BIN:-${NYASH_BIN:-$ROOT_DIR/target/release/hakorune}}"

APP_PATH=""
MIR_JSON=""
METHOD=""
PROFILE="replacement-front"
GROUP=""

usage() {
  cat <<'USAGE'
Usage:
  tools/hako_check/fastpath_check.sh --mir-json app.mir.json [options]
  tools/hako_check/fastpath_check.sh --app app.hako [options]
  tools/hako_check/fastpath_check.sh app.hako [options]

Options:
  --mir-json PATH     Read an existing MIR JSON artifact.
  --app PATH          Emit MIR JSON for a .hako app, then check it.
  --method NAME       Filter to an exact MIR function name.
  --profile NAME      Check profile: direct-exact or replacement-front.
                      default/hot-report/direct-memory are accepted but only
                      report existing obligation failures in v0.
  --group NAME        Optional route target group.
  -h, --help          Show this help.
USAGE
}

die() {
  echo "fastpath_check.sh: $*" >&2
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
    --profile)
      [ "$#" -ge 2 ] || die "--profile requires a name"
      case "$2" in
        default|hot-report|direct-memory|direct-exact|replacement-front) PROFILE="$2" ;;
        *) die "--profile must be one of: default, hot-report, direct-memory, direct-exact, replacement-front" ;;
      esac
      shift 2
      ;;
    --group)
      [ "$#" -ge 2 ] || die "--group requires a name"
      GROUP="$2"
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
  TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hako_check_fastpath_check.XXXXXX")"
  MIR_JSON="$TMP_DIR/app.mir.json"
  NYASH_FEATURES="${NYASH_FEATURES:-rune}" \
  NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
    "$BIN_PATH" --backend mir --emit-mir-json "$MIR_JSON" "$APP_PATH" >/dev/null
else
  [ -f "$MIR_JSON" ] || die "MIR JSON not found: $MIR_JSON"
fi

ARGS=(
  python3
  "$ROOT_DIR/tools/hako_check/fastpath_check.py"
  --mir-json "$MIR_JSON"
  --profile "$PROFILE"
)

if [ -n "$GROUP" ]; then
  ARGS+=(--group "$GROUP")
fi
if [ -n "$METHOD" ]; then
  ARGS+=(--method "$METHOD")
fi

"${ARGS[@]}"
