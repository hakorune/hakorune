#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="pure-first-mir-artifact-exactness"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SELFHOST_BUILD="tools/selfhost/selfhost_build.sh"
ROUTE_HELPER="tools/selfhost/lib/selfhost_build_route.sh"
EXE_HELPER="tools/selfhost/lib/selfhost_build_exe.sh"
DIRECT_HELPER="tools/selfhost/lib/selfhost_build_direct.sh"
PURE_FIRST_LIB="tools/checks/lib/pure_first_exe_guard.sh"
EMIT_ROUTE="tools/smokes/v2/lib/emit_mir_route.sh"
SSOT="docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-450-MIR-EMIT-SSOT-001-PURE-FIRST-MIR-ARTIFACT-EXACTNESS.md"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking selfhost --mir-in artifact exactness"

guard_require_command "$TAG" python3
guard_require_command "$TAG" sha256sum
guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$SELFHOST_BUILD" \
  "$ROUTE_HELPER" \
  "$EXE_HELPER" \
  "$DIRECT_HELPER" \
  "$PURE_FIRST_LIB" \
  "$EMIT_ROUTE" \
  "$SSOT" \
  "$CARD" \
  "$INDEX"

guard_expect_in_file "$TAG" '--mir-in FILE' "$SSOT" "SSOT must define MIR input semantics"
guard_expect_in_file "$TAG" '--mir-out FILE' "$SSOT" "SSOT must define MIR output semantics"
guard_expect_in_file "$TAG" 'preflight MIR SHA == EXE input MIR SHA' "$CARD" "card must pin exactness return condition"
guard_expect_in_file "$TAG" 'tools/checks/pure_first_mir_artifact_exactness_guard.sh' "$INDEX" "check index must list this guard"
guard_expect_in_file "$TAG" '--mir-in' "$PURE_FIRST_LIB" "pure-first guard must build EXE from preflight MIR input"
guard_expect_in_file "$TAG" 'sha256sum .*mir_json' "$PURE_FIRST_LIB" "pure-first guard must hash the MIR artifact around EXE build"

tmp_dir="$(mktemp -d /tmp/hakorune_mir_exactness.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

app="$tmp_dir/min.hako"
mir_json="$tmp_dir/min.mir.json"
mir_out="$tmp_dir/min.out.mir.json"
mir_alias="$tmp_dir/min.alias.mir.json"
exe_out="$tmp_dir/min.exe"
build_log="$tmp_dir/build.log"
stub_log="$tmp_dir/stub.log"
stub_nyllvm="$tmp_dir/ny-llvmc-stub.sh"

cat >"$app" <<'HAKO'
static box Main {
  main() {
    return 0
  }
}
HAKO

cargo build -q --bin hakorune

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
  "$EMIT_ROUTE" --route direct --out "$mir_json" --input "$app" >/dev/null

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = data.get("functions")
if not isinstance(functions, list) or not functions:
    raise SystemExit("missing functions array")

main = next((fn for fn in functions if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function")

metadata = main.get("metadata")
if not isinstance(metadata, dict):
    raise SystemExit("main metadata is missing")

lowering = metadata.get("lowering_plan", None)
if not isinstance(lowering, list):
    raise SystemExit("main metadata.lowering_plan is not an array")
PY

cat >"$stub_nyllvm" <<'SH'
#!/usr/bin/env bash
set -euo pipefail

in_path=""
out_path=""
while [ $# -gt 0 ]; do
  case "$1" in
    --in)
      in_path="${2:-}"
      shift 2
      ;;
    --out|-o)
      out_path="${2:-}"
      shift 2
      ;;
    --emit|--nyrt)
      shift 2
      ;;
    *)
      shift
      ;;
  esac
done

if [ -z "$in_path" ] || [ -z "$out_path" ]; then
  echo "[ny-llvmc-stub] missing --in/--out" >&2
  exit 2
fi
if [ "$in_path" != "${EXPECTED_MIR_IN:-}" ]; then
  echo "[ny-llvmc-stub] unexpected input: $in_path expected=${EXPECTED_MIR_IN:-}" >&2
  exit 2
fi
printf '%s\n' "$in_path" >>"${STUB_LOG:?}"
printf '#!/usr/bin/env bash\nexit 0\n' >"$out_path"
chmod +x "$out_path"
SH
chmod +x "$stub_nyllvm"

before_sha="$(sha256sum "$mir_json" | awk '{print $1}')"
EXPECTED_MIR_IN="$mir_json" \
STUB_LOG="$stub_log" \
NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
NYASH_DISABLE_PLUGINS=1 \
NYASH_NY_LLVM_COMPILER="$stub_nyllvm" \
NYASH_EMIT_EXE_NYRT="$tmp_dir" \
  "$SELFHOST_BUILD" --mir-in "$mir_json" --exe "$exe_out" >"$build_log" 2>&1
after_sha="$(sha256sum "$mir_json" | awk '{print $1}')"

if [ "$before_sha" != "$after_sha" ]; then
  echo "[$TAG] ERROR: --mir-in EXE route rewrote MIR artifact" >&2
  sed -n '1,160p' "$build_log" >&2
  exit 1
fi
guard_expect_in_file "$TAG" 'using MIR JSON input' "$build_log" "--mir-in route must report existing MIR input"
if rg -F -q 'emitting MIR JSON' "$build_log"; then
  echo "[$TAG] ERROR: --mir-in route re-emitted source MIR" >&2
  sed -n '1,160p' "$build_log" >&2
  exit 1
fi
guard_expect_in_file "$TAG" "$mir_json" "$stub_log" "ny-llvmc stub must receive the preflight MIR artifact"

NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
NYASH_DISABLE_PLUGINS=1 \
  "$SELFHOST_BUILD" --in "$app" --mir-out "$mir_out" >/dev/null
NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
NYASH_DISABLE_PLUGINS=1 \
  "$SELFHOST_BUILD" --in "$app" --mir "$mir_alias" >/dev/null

python3 - "$mir_out" "$mir_alias" <<'PY'
import json
import sys

for path in sys.argv[1:]:
    with open(path, encoding="utf-8") as fh:
        data = json.load(fh)
    main = next((fn for fn in data.get("functions", []) if fn.get("name") == "main"), None)
    if main is None:
        raise SystemExit(f"{path}: missing main")
    lowering = main.get("metadata", {}).get("lowering_plan")
    if not isinstance(lowering, list):
        raise SystemExit(f"{path}: missing metadata.lowering_plan")
PY

if NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_NY_LLVM_COMPILER="$stub_nyllvm" \
  "$SELFHOST_BUILD" --mir-in "$mir_json" --mir-out "$tmp_dir/ambiguous.mir.json" --exe "$tmp_dir/ambiguous.exe" \
  >"$tmp_dir/ambiguous.out" 2>"$tmp_dir/ambiguous.err"; then
  echo "[$TAG] ERROR: --mir-in and --mir-out combination must fail" >&2
  exit 1
fi
guard_expect_in_file "$TAG" 'cannot combine --mir-in with --mir/--mir-out' "$tmp_dir/ambiguous.err" "ambiguous MIR direction must fail fast"

echo "[$TAG] ok"
