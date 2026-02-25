#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"

BIN="${NYASH_BIN:-$NYASH_ROOT/target/release/hakorune}"
if [ ! -x "$BIN" ]; then
  BIN="$NYASH_ROOT/target/release/nyash"
fi
WRAPPER="$NYASH_ROOT/tools/selfhost/run.sh"

if [ ! -x "$BIN" ]; then
  log_error "nyash binary not found: $BIN"
  exit 2
fi
if [ ! -x "$WRAPPER" ]; then
  log_error "selfhost wrapper not found/executable: $WRAPPER"
  exit 2
fi

FEATURES="${PARSER_ANNOTATION_FEATURES:-stage3,opt-annotations,no-try-compat}"
TIMEOUT_SECS="${PARSER_ANNOTATION_SELFHOST_TIMEOUT_SECS:-30}"
if ! [[ "$TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  log_error "timeout must be integer: $TIMEOUT_SECS"
  exit 2
fi

TMPDIR="$(mktemp -d /tmp/parser_opt_annotations_dual_route.XXXXXX)"
cleanup() {
  rm -rf "$TMPDIR"
}
trap cleanup EXIT

BASE_SRC="$TMPDIR/base.hako"
ANNO_SRC="$TMPDIR/annotated.hako"
cat >"$BASE_SRC" <<'HK'
static box Main {
  main() {
    local s = "abc"
    return s.length()
  }
}
HK

cat >"$ANNO_SRC" <<'HK'
static box Main {
  @hint(hot)
  @intrinsic_candidate("StringBox.length/0")
  main() {
    @contract(no_alloc)
    local s = "abc"
    return s.length()
  }
}
HK

RUST_BASE_JSON="$TMPDIR/rust_base.json"
RUST_ANNO_JSON="$TMPDIR/rust_anno.json"
HAKO_BASE_LOG="$TMPDIR/hako_base.log"
HAKO_ANNO_LOG="$TMPDIR/hako_anno.log"
HAKO_BASE_JSON="$TMPDIR/hako_base.json"
HAKO_ANNO_JSON="$TMPDIR/hako_anno.json"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-program-json-v0 "$RUST_BASE_JSON" "$BASE_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --emit-program-json-v0 "$RUST_ANNO_JSON" "$ANNO_SRC" >/dev/null

NYASH_FEATURES="$FEATURES" \
  "$WRAPPER" --direct --source-file "$BASE_SRC" --timeout-secs "$TIMEOUT_SECS" \
  >"$HAKO_BASE_LOG" 2>&1
NYASH_FEATURES="$FEATURES" \
  "$WRAPPER" --direct --source-file "$ANNO_SRC" --timeout-secs "$TIMEOUT_SECS" \
  >"$HAKO_ANNO_LOG" 2>&1

if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' \
  "$HAKO_BASE_LOG" >"$HAKO_BASE_JSON"; then
  log_error "hako parser route did not emit Program(JSON v0) for baseline fixture"
  tail -n 80 "$HAKO_BASE_LOG" >&2 || true
  exit 1
fi

if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' \
  "$HAKO_ANNO_LOG" >"$HAKO_ANNO_JSON"; then
  log_error "hako parser route did not emit Program(JSON v0) for annotated fixture"
  tail -n 80 "$HAKO_ANNO_LOG" >&2 || true
  exit 1
fi

python3 - "$RUST_BASE_JSON" "$RUST_ANNO_JSON" "$HAKO_BASE_JSON" "$HAKO_ANNO_JSON" <<'PY'
import json
import sys

RUST_BASE, RUST_ANNO, HAKO_BASE, HAKO_ANNO = sys.argv[1:5]
DROP_KEYS = {"span", "line", "column", "loc", "source", "file", "start", "end"}


def normalize(obj):
    if isinstance(obj, dict):
        return {
            key: normalize(value)
            for key, value in sorted(obj.items())
            if key not in DROP_KEYS
        }
    if isinstance(obj, list):
        return [normalize(v) for v in obj]
    return obj


def load_norm(path):
    with open(path, "r", encoding="utf-8") as f:
        return normalize(json.load(f))


rust_base = load_norm(RUST_BASE)
rust_anno = load_norm(RUST_ANNO)
hako_base = load_norm(HAKO_BASE)
hako_anno = load_norm(HAKO_ANNO)

if rust_base != rust_anno:
    print("rust parser route changed Program(JSON v0) with annotations", file=sys.stderr)
    sys.exit(1)
if hako_base != hako_anno:
    print("hako parser route changed Program(JSON v0) with annotations", file=sys.stderr)
    sys.exit(1)
PY

log_success "parser_opt_annotations_dual_route_noop"
