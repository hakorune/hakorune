#!/bin/bash
# phase29cc_wsm_g4_min2_playground_canvas_primer_vm.sh
# Contract pin:
# - WSM-G4-min2: nyash_playground canvas primer lock
# - min vocab: WebCanvasBox + clear

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-172-wsm-g4-min2-nyash-playground-canvas-primer-lock-ssot.md"
html="$NYASH_ROOT/projects/nyash-wasm/nyash_playground.html"

if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_g4_min2_playground_canvas_primer_vm: lock doc missing"
  exit 1
fi

if [ ! -f "$html" ]; then
  test_fail "phase29cc_wsm_g4_min2_playground_canvas_primer_vm: playground html missing"
  exit 1
fi

for needle in \
  "WSM-G4-min2" \
  "canvas primer" \
  "wsm_g4_min2_canvas_primer_lock" \
  "wsm_g4_min2_canvas_vocab_clear"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_g4_min2_playground_canvas_primer_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

for needle in \
  "wsm_g4_min2_canvas_primer_lock" \
  "wsm_g4_min2_canvas_vocab_clear" \
  "new WebCanvasBox(\"game-canvas\", 400, 250)" \
  "me.canvas.clear()" \
  "fillRect(" \
  "strokeRect(" \
  "fillCircle(" \
  "drawLine(" \
  "fillText("; do
  if ! grep -Fq "$needle" "$html"; then
    test_fail "phase29cc_wsm_g4_min2_playground_canvas_primer_vm: missing keyword in nyash_playground.html: $needle"
    exit 1
  fi
done

bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh"

test_pass "phase29cc_wsm_g4_min2_playground_canvas_primer_vm: PASS (WSM-G4-min2 playground canvas primer lock)"
