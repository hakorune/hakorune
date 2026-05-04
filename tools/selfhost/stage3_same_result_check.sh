#!/usr/bin/env bash
set -euo pipefail
[[ "${NYASH_CLI_VERBOSE:-0}" == "1" ]] && set -x

# Bootstrap Stage3 same-result sanity gate.
# This is separate from the parser/bridge Stage3 acceptance smoke in
# tools/selfhost/proof/selfhost_stage3_accept_smoke.sh.

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)"
source "$ROOT_DIR/tools/selfhost/lib/identity_routes.sh"
ARTIFACT_KIND="stage1-cli"
SKIP_BUILD=0
BUILD_SEED=0
BOOTSTRAP_BIN=""
STAGE2_BIN=""
STAGE3_BIN=""
ENTRY=""

usage() {
  cat <<'USAGE'
Usage: tools/selfhost/stage3_same_result_check.sh [--artifact-kind <stage1-cli|launcher-exe>] [--build-seed] [--skip-build] [--seed-bin <path>] [--stage2-bin <path>] [--stage3-bin <path>] [-h|--help]

Options:
  --artifact-kind <kind>  Bootstrap artifact kind to compare (default: stage1-cli)
  --build-seed            Build the full stage1_cli_env.hako payload seed before comparing
  --skip-build            Compare an explicit prebuilt Stage2/Stage3 pair only
  --seed-bin <path>       Stage1 bootstrap seed binary (default: derived from artifact kind)
  --stage2-bin <path>     Stage2 output base path (build mode) or binary path (skip-build)
  --stage3-bin <path>     Stage3 output base path (build mode) or binary path (skip-build)
  -h, --help              Show this help text

Behavior:
  - build mode: re-emit Program(JSON v0) and MIR(JSON) snapshots twice from a known-good bootstrap seed, then compare the snapshots plus .artifact_kind
  - build mode currently has a stable seed lane for stage1-cli only
  - --build-seed uses tools/selfhost/mainline/build_stage1.sh to materialize a full stage1_cli_env.hako seed
  - compare mode: compare the provided Stage2/Stage3 pair only
  - `stage2-bin` / `stage3-bin` are compare labels in this helper, not standalone artifact-kind families
USAGE
}

fail() {
  echo "[Stage3:FAIL] $*" >&2
  exit 2
}

fail_payload_seed() {
  local seed_bin="$1"
  local payload_kind="$2"
  local rc_label="$3"

  echo "[Stage3:FAIL] failed to materialize ${payload_kind} via stage1 env route: ${seed_bin}" >&2
  if stage1_contract_artifact_is_reduced_stage1_cli "$seed_bin"; then
    echo "              seed-boundary: reduced stage1-cli artifacts are runnable bootstrap outputs, not payload emit seeds" >&2
    echo "              hint: build a full stage1_cli_env.hako artifact and pass it with --seed-bin" >&2
  else
    echo "              artifact_kind=$(stage1_contract_artifact_kind "$seed_bin")" >&2
    echo "              artifact_entry=$(stage1_contract_artifact_entry "$seed_bin")" >&2
  fi
  exit "$rc_label"
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

  if ! run_stage1_env_route "$seed_bin" "program-json" "$entry" "$prog_out"; then
    fail_payload_seed "$seed_bin" "Program(JSON)" 2
  fi
  if ! run_stage1_env_route "$seed_bin" "mir-json" "$entry" "$mir_out"; then
    fail_payload_seed "$seed_bin" "MIR(JSON)" 2
  fi
}

build_stage1_cli_env_seed() {
  local seed_bin="$1"
  local entry="$2"

  if [[ "$ARTIFACT_KIND" != "stage1-cli" ]]; then
    fail "--build-seed currently supports --artifact-kind stage1-cli only"
  fi

  echo "[Stage3] Building payload seed artifact: $seed_bin" >&2
  env \
    NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
    HAKO_BACKEND_COMPILE_RECIPE="${HAKO_BACKEND_COMPILE_RECIPE:-pure-first}" \
    HAKO_BACKEND_COMPAT_REPLAY="${HAKO_BACKEND_COMPAT_REPLAY:-none}" \
    "$ROOT_DIR/tools/selfhost/mainline/build_stage1.sh" \
      --artifact-kind stage1-cli \
      --entry "$entry" \
      --out "$seed_bin" \
      --timeout-secs "${HAKORUNE_STAGE3_SEED_BUILD_TIMEOUT_SECS:-240}" \
      --force-rebuild
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --artifact-kind)
      ARTIFACT_KIND="$2"
      shift 2
      ;;
    --build-seed)
      BUILD_SEED=1
      shift
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
    DEFAULT_BOOTSTRAP_BIN="$ROOT_DIR/target/selfhost/hakorune.stage1_cli_env_seed"
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

  if [[ "$BUILD_SEED" -eq 1 ]]; then
    mkdir -p "$(dirname "$BOOTSTRAP_BIN")"
    build_stage1_cli_env_seed "$BOOTSTRAP_BIN" "$ENTRY"
  fi

  if [[ ! -x "$BOOTSTRAP_BIN" ]]; then
    echo "[Stage3:FAIL] Stage1 payload seed binary not found or not executable: $BOOTSTRAP_BIN" >&2
    echo "              hint: pass --build-seed or provide a full stage1_cli_env.hako artifact with --seed-bin" >&2
    exit 2
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
