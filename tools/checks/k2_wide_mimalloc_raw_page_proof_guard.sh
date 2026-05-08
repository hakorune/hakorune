#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-raw-page-proof"
cd "$ROOT_DIR"

APP_DIR="apps/mimalloc-raw-page-proof"
APP="$APP_DIR/main.hako"
APP_TEST="$APP_DIR/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-062-M12-MIMALLOC-RAW-PAGE-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
SUBSTRATE_DOC="docs/reference/runtime/substrate-capabilities.md"

echo "[$TAG] running M12 raw-page proof guard"

bash "$APP_TEST"

rg -F -q 'using selfhost.runtime.substrate.raw_buf.raw_buf_core_box as RawBufCoreBox' "$APP"
rg -F -q 'using selfhost.runtime.substrate.raw_array.raw_array_core_box as RawArrayCoreBox' "$APP"
rg -F -q '@rune Contract(no_alloc)' "$APP"
rg -F -q '@rune Contract(no_safepoint)' "$APP"
rg -F -q 'RawBufCoreBox.alloc_bytes_i64' "$APP"
rg -F -q 'RawBufCoreBox.free_bytes_i64' "$APP"
rg -F -q 'RawArrayCoreBox.slot_load_i64' "$APP"
rg -F -q 'RawArrayCoreBox.slot_store_i64' "$APP"
rg -F -q 'RawArrayCoreBox.slot_append_any' "$APP"
rg -F -q 'M12 proof fixture' "$APP_DIR/README.md"
rg -F -q 'M12 mimalloc raw-page proof' "$CARD"
rg -F -q '| `M12 mimalloc raw-page proof` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'Decision: M12 raw-page proof is live-narrow.' "$SUBSTRATE_DOC"

if rg -F -q 'externcall "hako_mem_' "$APP"; then
  echo "[$TAG] ERROR: app must use RawBufCoreBox instead of direct hako.mem externcall" >&2
  exit 1
fi

if rg -F -q '@rune Profile' "$APP"; then
  echo "[$TAG] ERROR: Profile parser surface is not owned by M12" >&2
  exit 1
fi

if rg -F -q '@rune Capability' "$APP"; then
  echo "[$TAG] ERROR: Capability parser surface is not owned by M12" >&2
  exit 1
fi

if rg -F -q 'unsafe(' "$APP"; then
  echo "[$TAG] ERROR: restricted unsafe blocks are not owned by M12" >&2
  exit 1
fi

if rg -F -q 'mimalloc-raw-page-proof' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not know the M12 fixture name" >&2
  exit 1
fi

echo "[$TAG] ok"
