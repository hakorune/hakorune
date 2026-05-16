#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="selfhost-progress-diagnostics"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SELFHOST_BUILD="tools/selfhost/selfhost_build.sh"
PROGRESS_HELPER="tools/selfhost/lib/selfhost_progress.sh"
DIRECT_HELPER="tools/selfhost/lib/selfhost_build_direct.sh"
EXE_HELPER="tools/selfhost/lib/selfhost_build_exe.sh"
RUN_HELPER="tools/selfhost/lib/selfhost_build_run.sh"
PURE_FIRST_LIB="tools/checks/lib/pure_first_exe_guard.sh"
SSOT="docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-452-SELFHOST-PROGRESS-001-PHASE-PROGRESS-DIAGNOSTICS.md"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking selfhost phase progress diagnostics"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$SELFHOST_BUILD" \
  "$PROGRESS_HELPER" \
  "$DIRECT_HELPER" \
  "$EXE_HELPER" \
  "$RUN_HELPER" \
  "$PURE_FIRST_LIB" \
  "$SSOT" \
  "$CARD" \
  "$INDEX"

guard_expect_in_file "$TAG" 'selfhost_phase_start' "$PROGRESS_HELPER" "progress helper must own phase start"
guard_expect_in_file "$TAG" 'HAKO_SELFHOST_PROGRESS_FILE' "$PROGRESS_HELPER" "progress helper must own closeout file"
guard_expect_in_file "$TAG" 'phase=selfhost.emit_mir start' "$CARD" "452 card must pin emit_mir text phase"
guard_expect_in_file "$TAG" 'phase=selfhost.nyllvmc start' "$CARD" "452 card must pin nyllvmc text phase"
guard_expect_in_file "$TAG" 'selfhost.route_preflight' "$PURE_FIRST_LIB" "pure-first guard must report route preflight phase"
guard_expect_in_file "$TAG" 'last selfhost progress' "$PURE_FIRST_LIB" "pure-first guard must print timeout/failure closeout"
guard_expect_in_file "$TAG" 'tools/checks/selfhost_progress_diagnostics_guard.sh' "$INDEX" "check index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_selfhost_progress.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

app="$tmp_dir/min.hako"
mir_json="$tmp_dir/min.mir.json"
exe_out="$tmp_dir/min.exe"
emit_err="$tmp_dir/emit.err"
exe_err="$tmp_dir/exe.err"
run_err="$tmp_dir/run.err"
progress_file="$tmp_dir/progress.txt"
stub_nyllvm="$tmp_dir/ny-llvmc-stub.sh"

cat >"$app" <<'HAKO'
static box Main {
  main() {
    return 0
  }
}
HAKO

cargo build -q --bin hakorune

NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
NYASH_DISABLE_PLUGINS=1 \
  "$SELFHOST_BUILD" --in "$app" --mir-out "$mir_json" >/dev/null 2>"$emit_err"
guard_expect_in_file "$TAG" 'phase=selfhost.emit_mir start' "$emit_err" "MIR emit must report phase start"
guard_expect_in_file "$TAG" 'phase=selfhost.emit_mir done elapsed_ms=' "$emit_err" "MIR emit must report phase done"

cat >"$stub_nyllvm" <<'SH'
#!/usr/bin/env bash
set -euo pipefail

out_path=""
while [ $# -gt 0 ]; do
  case "$1" in
    --out|-o)
      out_path="${2:-}"
      shift 2
      ;;
    --in|--emit|--nyrt)
      shift 2
      ;;
    *)
      shift
      ;;
  esac
done

if [ -z "$out_path" ]; then
  echo "[ny-llvmc-stub] missing --out" >&2
  exit 2
fi
printf '#!/usr/bin/env bash\nexit 0\n' >"$out_path"
chmod +x "$out_path"
SH
chmod +x "$stub_nyllvm"

HAKO_SELFHOST_PROGRESS_FILE="$progress_file" \
NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
NYASH_DISABLE_PLUGINS=1 \
NYASH_NY_LLVM_COMPILER="$stub_nyllvm" \
NYASH_EMIT_EXE_NYRT="$tmp_dir" \
  "$SELFHOST_BUILD" --mir-in "$mir_json" --exe "$exe_out" >/dev/null 2>"$exe_err"
guard_expect_in_file "$TAG" 'phase=selfhost.nyllvmc start' "$exe_err" "EXE route must report nyllvmc phase start"
guard_expect_in_file "$TAG" 'phase=selfhost.nyllvmc done elapsed_ms=' "$exe_err" "EXE route must report nyllvmc phase done"
guard_expect_in_file "$TAG" 'phase=selfhost.nyllvmc state=done elapsed_ms=' "$progress_file" "progress file must record last completed phase"

printf '{"functions":[]}\n' >"$tmp_dir/bad.mir.json"
if HAKO_SELFHOST_PROGRESS_FILE="$progress_file" \
  NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
  NYASH_DISABLE_PLUGINS=1 \
  "$SELFHOST_BUILD" --mir-in "$tmp_dir/bad.mir.json" --run >/dev/null 2>"$run_err"; then
  echo "[$TAG] ERROR: invalid MIR run should fail" >&2
  exit 1
fi
guard_expect_in_file "$TAG" 'phase=selfhost.run start' "$run_err" "run route must report phase start"
guard_expect_in_file "$TAG" 'phase=selfhost.run fail rc=' "$run_err" "run route must report phase failure"
guard_expect_in_file "$TAG" 'phase=selfhost.run state=fail elapsed_ms=' "$progress_file" "progress file must record failed phase"

echo "[$TAG] ok"
