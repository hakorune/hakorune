#!/usr/bin/env bash
set -euo pipefail
[[ "${NYASH_CLI_VERBOSE:-0}" == "1" ]] && set -x

# Bootstrap Stage3 same-result sanity gate.
# This is separate from the parser/bridge Stage3 acceptance smoke in
# tools/selfhost/selfhost_stage3_accept_smoke.sh.

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)"
ARTIFACT_KIND="stage1-cli"
SKIP_BUILD=0
BOOTSTRAP_BIN=""
STAGE2_BIN=""
STAGE3_BIN=""
ENTRY=""

usage() {
  cat <<'USAGE'
Usage: tools/selfhost/stage3_same_result_check.sh [--artifact-kind <stage1-cli|launcher-exe>] [--skip-build] [--stage2-bin <path>] [--stage3-bin <path>] [-h|--help]

Options:
  --artifact-kind <kind>  Bootstrap artifact kind to compare (default: stage1-cli)
  --skip-build            Compare an explicit prebuilt Stage2/Stage3 pair only
  --seed-bin <path>       Stage1 bootstrap seed binary (default: derived from artifact kind)
  --stage2-bin <path>     Stage2 output base path (build mode) or binary path (skip-build)
  --stage3-bin <path>     Stage3 output base path (build mode) or binary path (skip-build)
  -h, --help              Show this help text

Behavior:
  - build mode: re-emit Program(JSON v0) and MIR(JSON) snapshots twice from a known-good bootstrap seed, then compare the snapshots plus .artifact_kind
  - build mode currently has a stable seed lane for stage1-cli only
  - compare mode: compare the provided Stage2/Stage3 pair only
  - `stage2-bin` / `stage3-bin` are compare labels in this helper, not standalone artifact-kind families
USAGE
}

fail() {
  echo "[Stage3:FAIL] $*" >&2
  exit 2
}

compare_exact_files() {
  local label="$1"
  local lhs="$2"
  local rhs="$3"

  if cmp -s "$lhs" "$rhs"; then
    echo "[Stage3] ${label}: MATCH" >&2
    return 0
  fi

  echo "[Stage3:FAIL] ${label} mismatch" >&2
  echo "          lhs: $lhs" >&2
  echo "          rhs: $rhs" >&2
  if command -v sha256sum >/dev/null 2>&1; then
    echo "          lhs sha256: $(sha256sum "$lhs" | awk '{print $1}')" >&2
    echo "          rhs sha256: $(sha256sum "$rhs" | awk '{print $1}')" >&2
  fi
  if [[ "$label" == "metadata" ]]; then
    diff -u "$lhs" "$rhs" >&2 || true
  fi
  return 1
}

emit_seed_payloads() {
  local seed_bin="$1"
  local entry="$2"
  local prog_out="$3"
  local mir_out="$4"

  bash "$ROOT_DIR/tools/selfhost/run_stage1_cli.sh" --bin "$seed_bin" emit program-json "$entry" >"$prog_out"
  bash "$ROOT_DIR/tools/selfhost/run_stage1_cli.sh" --bin "$seed_bin" emit mir-json "$entry" >"$mir_out"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --artifact-kind)
      ARTIFACT_KIND="$2"
      shift 2
      ;;
    --skip-build)
      SKIP_BUILD=1
      shift
      ;;
    --seed-bin)
      BOOTSTRAP_BIN="$2"
      shift 2
      ;;
    --stage2-bin)
      STAGE2_BIN="$2"
      shift 2
      ;;
    --stage3-bin)
      STAGE3_BIN="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      fail "unknown arg: $1"
      ;;
  esac
done

case "$ARTIFACT_KIND" in
  stage1-cli)
    DEFAULT_BOOTSTRAP_BIN="$ROOT_DIR/target/selfhost/hakorune.stage1_cli.stage2"
    DEFAULT_STAGE2_BIN="$ROOT_DIR/target/selfhost/hakorune.stage1_cli.stage2"
    DEFAULT_STAGE3_BIN="$ROOT_DIR/target/selfhost/hakorune.stage1_cli.stage3"
    DEFAULT_ENTRY="$ROOT_DIR/lang/src/runner/stage1_cli_env.hako"
    ;;
  launcher-exe)
    DEFAULT_BOOTSTRAP_BIN="$ROOT_DIR/target/selfhost/hakorune"
    DEFAULT_STAGE2_BIN="$ROOT_DIR/target/selfhost/hakorune.stage2"
    DEFAULT_STAGE3_BIN="$ROOT_DIR/target/selfhost/hakorune.stage3"
    DEFAULT_ENTRY="$ROOT_DIR/lang/src/runner/launcher.hako"
    ;;
  *)
    fail "--artifact-kind must be stage1-cli|launcher-exe: $ARTIFACT_KIND"
    ;;
esac

if [[ -z "$BOOTSTRAP_BIN" ]]; then
  BOOTSTRAP_BIN="$DEFAULT_BOOTSTRAP_BIN"
fi
if [[ -z "$STAGE2_BIN" ]]; then
  STAGE2_BIN="$DEFAULT_STAGE2_BIN"
fi
if [[ -z "$STAGE3_BIN" ]]; then
  STAGE3_BIN="$DEFAULT_STAGE3_BIN"
fi
if [[ -z "$ENTRY" ]]; then
  ENTRY="$DEFAULT_ENTRY"
fi

STAGE2_META="${STAGE2_BIN}.artifact_kind"
STAGE3_META="${STAGE3_BIN}.artifact_kind"

if [[ "$SKIP_BUILD" -eq 0 ]]; then
  mkdir -p "$(dirname "$STAGE2_BIN")" "$(dirname "$STAGE3_BIN")"

  if [[ ! -x "$BOOTSTRAP_BIN" ]]; then
    fail "Stage1 seed binary not found or not executable: $BOOTSTRAP_BIN"
  fi
  if [[ "$ARTIFACT_KIND" != "stage1-cli" ]]; then
    fail "build mode currently only supports the stage1-cli payload seed lane; use --skip-build for launcher-exe compare-only"
  fi

  STAGE2_PROG="${STAGE2_BIN}.program.json"
  STAGE2_MIR="${STAGE2_BIN}.mir.json"
  STAGE3_PROG="${STAGE3_BIN}.program.json"
  STAGE3_MIR="${STAGE3_BIN}.mir.json"

  echo "[Stage3] Using payload seed artifact (${ARTIFACT_KIND}): $BOOTSTRAP_BIN" >&2
  echo "[Stage3] Re-emitting Stage2 payload snapshots..." >&2
  emit_seed_payloads "$BOOTSTRAP_BIN" "$ENTRY" "$STAGE2_PROG" "$STAGE2_MIR"
  echo "[Stage3] Re-emitting Stage3 payload snapshots..." >&2
  emit_seed_payloads "$BOOTSTRAP_BIN" "$ENTRY" "$STAGE3_PROG" "$STAGE3_MIR"

  {
    echo "artifact_kind=${ARTIFACT_KIND}"
    echo "entry=${ENTRY}"
  } >"$STAGE2_META"
  cp "$STAGE2_META" "$STAGE3_META"

  compare_exact_files "program-json" "$STAGE2_PROG" "$STAGE3_PROG"
  compare_exact_files "mir-json" "$STAGE2_MIR" "$STAGE3_MIR"
  compare_exact_files "metadata" "$STAGE2_META" "$STAGE3_META"
else
  echo "[Stage3] Skip-build mode: using prebuilt Stage2/Stage3 binaries" >&2
fi

if [[ "$SKIP_BUILD" -eq 1 ]]; then
  if [[ ! -x "$STAGE2_BIN" ]]; then
    fail "Stage2 binary not found or not executable: $STAGE2_BIN"
  fi
  if [[ ! -x "$STAGE3_BIN" ]]; then
    fail "Stage3 binary not found or not executable: $STAGE3_BIN"
  fi
  if [[ ! -f "$STAGE2_META" ]]; then
    fail "Stage2 metadata not found: $STAGE2_META"
  fi
  if [[ ! -f "$STAGE3_META" ]]; then
    fail "Stage3 metadata not found: $STAGE3_META"
  fi

  compare_exact_files "bootstrap artifact" "$STAGE2_BIN" "$STAGE3_BIN"
  compare_exact_files "metadata" "$STAGE2_META" "$STAGE3_META"
fi

echo "[Stage3] same-result: PASS (${ARTIFACT_KIND})" >&2
