#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
SUITE="proof/vm-adapter-legacy"

echo "[proof/vm-adapter-legacy] vm-adapter legacy cluster"

CHECK_FILE="/tmp/hako_inline_using_check_$$.hako"
trap 'rm -f "$CHECK_FILE" || true' EXIT
cat > "$CHECK_FILE" <<'HCODE'
using "selfhost.vm.helpers.mir_call_v1_handler" as MirCallV1HandlerBox
static box Main { method main(args) { return 0 } }
HCODE

set +e
NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 NYASH_FEATURES=stage3 \
  NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 "$ROOT/target/release/hakorune" --backend vm "$CHECK_FILE" >/dev/null 2>&1
USING_OK=$?
set -e

if [ "$USING_OK" -ne 0 ]; then
  echo "[proof/vm-adapter-legacy] SKIP adapter reps (inline using unsupported)" >&2
  exit 0
fi

bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --suite "$SUITE"

echo "[proof/vm-adapter-legacy] vm-adapter legacy cluster done."
