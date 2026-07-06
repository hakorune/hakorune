#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
POLICY="$ROOT_DIR/src/mir/route_value_type_publication.rs"
BOX_HELPERS="$ROOT_DIR/lang/src/shared/common/box_helpers.hako"
INSPECTOR="$ROOT_DIR/lang/src/shared/common/box_type_inspector_box.hako"
MIR_SCHEMA="$ROOT_DIR/lang/src/shared/mir/mir_schema_box.hako"
LEGACY_BRIDGE="$ROOT_DIR/lang/src/compat/codegen/legacy_emit_object_bridge_box.hako"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3122-HAKO-AOT-PROGRAMJSON-PHASE-STATE-POSITIVE-PATH-HELPER-ROUTE-READINESS-001.md"

need_fixed() {
  local needle="$1"
  local file="$2"
  local message="$3"
  if ! grep -F -q "$needle" "$file"; then
    echo "[box-type-inspector-predicate-publication-inventory] ERROR: $message" >&2
    echo "  missing: $needle" >&2
    echo "  file: $file" >&2
    exit 1
  fi
}

forbidden_fixed() {
  local needle="$1"
  local file="$2"
  local message="$3"
  if grep -F -q "$needle" "$file"; then
    echo "[box-type-inspector-predicate-publication-inventory] ERROR: $message" >&2
    echo "  forbidden: $needle" >&2
    echo "  file: $file" >&2
    exit 1
  fi
}

need_fixed 'pub(crate) const BOX_HELPERS_IS_ARRAY: &str = "BoxHelpers.is_array/1";' "$POLICY" \
  "BoxHelpers.is_array must remain the published polymorphic predicate helper"
need_fixed 'pub(crate) const BOX_HELPERS_IS_MAP: &str = "BoxHelpers.is_map/1";' "$POLICY" \
  "BoxHelpers.is_map must remain the published polymorphic predicate helper"
forbidden_fixed 'BoxTypeInspectorBox.is_map/1' "$POLICY" \
  "BoxTypeInspectorBox.is_map must stay parked until route/caller inventory is promoted"
forbidden_fixed 'BoxTypeInspectorBox.is_array/1' "$POLICY" \
  "BoxTypeInspectorBox.is_array must stay parked until route/caller inventory is promoted"

need_fixed 'is_map(val)' "$BOX_HELPERS" \
  "BoxHelpers must remain the wrapper surface for is_map"
need_fixed 'val.is("MapBox")' "$BOX_HELPERS" \
  "BoxHelpers.is_map must keep the direct predicate wrapper body"
need_fixed 'is_array(val)' "$BOX_HELPERS" \
  "BoxHelpers must remain the wrapper surface for is_array"
need_fixed 'val.is("ArrayBox")' "$BOX_HELPERS" \
  "BoxHelpers.is_array must keep the direct predicate wrapper body"
need_fixed 'method is_map(value)' "$INSPECTOR" \
  "BoxTypeInspectorBox must still expose is_map"
need_fixed 'method is_array(value)' "$INSPECTOR" \
  "BoxTypeInspectorBox must still expose is_array"
need_fixed 'BoxTypeInspectorBox.is_map(val)' "$MIR_SCHEMA" \
  "MirSchemaBox direct predicate caller must stay inventoried"
need_fixed 'BoxTypeInspectorBox.is_array(val)' "$MIR_SCHEMA" \
  "MirSchemaBox direct predicate caller must stay inventoried"
need_fixed 'BoxTypeInspectorBox.is_array(value)' "$LEGACY_BRIDGE" \
  "legacy emit object bridge direct predicate caller must stay inventoried"
need_fixed 'BoxTypeInspectorBox predicate publication remains parked' "$CARD" \
  "phase card must document parked BoxTypeInspectorBox publication"

echo "output_contract=hako-aot-box-type-inspector-predicate-publication-inventory-v0"
echo "box_helpers_predicate_publication=green"
echo "box_type_inspector_predicate_publication=parked"
echo "direct_callers_inventoried=MirSchemaBox,LegacyEmitObjectBridgeBox"
echo "route_family_unification_claim=0"
echo "backend_lowering_claim=0"
echo "summary=ok"
