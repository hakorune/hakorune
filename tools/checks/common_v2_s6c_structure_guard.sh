#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="common-v2-s6c-structure"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc

files=(
  "$ROOT_DIR/src/mir/loop_recipe_contract/s6c_prephysical_ingress.rs"
  "$ROOT_DIR/src/mir/loop_recipe_contract/s6c_prephysical_ingress_validation.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_session.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_session_length.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_session_segments.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_substring_callout_materializer.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_text_content_root_admission.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_text_cursor_preheader.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_scalar_equality_leaf.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_cursor_cfg.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/physical_entry_draftseal.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/draft_seal/text_residence_exit.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/session/pinned_text_plan.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/session/residence_lifecycle.rs"
  "$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_cfg/session.rs"
  "$ROOT_DIR/src/mir/pinned_text_access_plan.rs"
  "$ROOT_DIR/src/mir/pinned_text_residence_lifecycle.rs"
  "$ROOT_DIR/src/mir/builder/pinned_text_invocation_binding.rs"
  "$ROOT_DIR/src/mir/builder/module_invocation_session.rs"
  "$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs"
  "$ROOT_DIR/src/mir/normal_callable_semantic_package/install.rs"
)
guard_require_files "$TAG" "${files[@]}"

for file in "${files[@]}"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source reached the hard 800-line boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

ingress="$ROOT_DIR/src/mir/loop_recipe_contract/s6c_prephysical_ingress.rs"
session="$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_session.rs"
guard_expect_fixed_in_file "$TAG" 's6c_prephysical_ingress_validation.rs' "$ingress" \
  "ingress must retain the private source-anchor validation child"
guard_expect_fixed_in_file "$TAG" 'common_v2_session_length.rs' "$session" \
  "session must retain the private Length child"
guard_expect_fixed_in_file "$TAG" 'common_v2_session_segments.rs' "$session" \
  "session must retain the private segment child"
guard_expect_fixed_in_file "$TAG" 's6c_substring_callout_materializer.rs' "$session" \
  "session must retain the private canonical V9 materializer child"
content_admission="$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_text_content_root_admission.rs"
guard_expect_fixed_in_file "$TAG" 'issue_common_v2_s6c_text_content_root_admission_v1' "$content_admission" \
  "base-root mapping must have one named compiler issuer"
guard_expect_fixed_in_file "$TAG" 'V9 remains a derived slice and never becomes a root' "$content_admission" \
  "V9 must remain outside the base-root namespace"
guard_expect_fixed_in_file "$TAG" 'Subject,' "$content_admission" \
  "base-root roles must include an explicit Subject label"
guard_expect_fixed_in_file "$TAG" 'Needle,' "$content_admission" \
  "base-root roles must include an explicit Needle label"
cursor_preheader="$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_text_cursor_preheader.rs"
guard_expect_fixed_in_file "$TAG" 'issue_common_v2_s6c_text_cursor_preheader_v1' "$cursor_preheader" \
  "cursor/preheader must have one named effect-free issuer"
guard_expect_fixed_in_file "$TAG" 'byte_offset: 0' "$cursor_preheader" \
  "cursor/preheader must initialize the byte offset exactly once"
if rg -n '^(use|pub|impl|struct|enum|fn).*\b(ValueId|MirInstruction|PinnedTextOp)\b' "$cursor_preheader"; then
  guard_fail "$TAG" "cursor/preheader I0 must not issue MIR/ValueId/PinnedTextOp"
fi
scalar_leaf="$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_scalar_equality_leaf.rs"
guard_expect_fixed_in_file "$TAG" 'issue_common_v2_s6c_text_scalar_equality_leaf_v1' "$scalar_leaf" \
  "scalar-equality I0 must have one named effect-free issuer"
if rg -n '^(use|pub|impl|struct|enum|fn).*\b(ValueId|MirInstruction|PinnedTextOp)\b' "$scalar_leaf"; then
  guard_fail "$TAG" "scalar-equality I0 must not issue MIR/ValueId/PinnedTextOp"
fi
cursor_cfg="$ROOT_DIR/src/mir/builder/resolved_lowering/common_v2_s6c_cursor_cfg.rs"
guard_expect_fixed_in_file "$TAG" 'materialize_common_v2_s6c_cursor_cfg_v1' "$cursor_cfg" \
  "cursor CFG must have one named canonical materializer"
guard_expect_fixed_in_file "$TAG" 'emit_branch' "$cursor_cfg" \
  "cursor CFG must use the canonical CFG writer"
guard_expect_fixed_in_file "$TAG" 'define_provisional_phi' "$cursor_cfg" \
  "cursor CFG must use the canonical PHI lifecycle"
guard_expect_fixed_in_file "$TAG" 'MirInstruction::PinnedTextOp' "$cursor_cfg" \
  "cursor CFG must materialize the existing pinned-text leaf"
if rg -n 'TextContentFrame|Arc<|nyash\.string\.eq_hh|RawPointer' "$cursor_cfg"; then
  guard_fail "$TAG" "cursor CFG I0 must not open a new frame, Arc owner, or legacy C route"
fi
draftseal="$ROOT_DIR/src/mir/builder/resolved_lowering/physical_entry_draftseal.rs"
guard_expect_fixed_in_file "$TAG" 'with_common_v2_canonical_session_branded_finish' "$draftseal" \
  "DraftSeal probe must reuse the canonical finish wrapper"
guard_expect_fixed_in_file "$TAG" 'finish_for_draft_seal' "$session" \
  "common session must reuse the canonical finish owner"
guard_expect_fixed_in_file "$TAG" 'prepare_exact_two' "$draftseal" \
  "DraftSeal probe must use the existing exact exit-set preparation"
guard_expect_fixed_in_file "$TAG" 'discard_unpublished' "$draftseal" \
  "DraftSeal probe must retain one outer rollback owner"
if rg -n 'TextContentFrame|Arc<|nyash\.string\.eq_hh|RawPointer' "$draftseal"; then
  guard_fail "$TAG" "DraftSeal probe must not open runtime/legacy/production routes"
fi
lifecycle="$ROOT_DIR/src/mir/builder/resolved_lowering/draft_seal/text_residence_exit.rs"
guard_expect_fixed_in_file "$TAG" 'PreparedTextFormalExitFinishSetV1' "$lifecycle" \
  "Residence lifecycle I0 must use one private lifetime-bound admission"
guard_expect_fixed_in_file "$TAG" 'issue_pinned_text_residence_exit_finish_set_v1' "$lifecycle" \
  "Residence lifecycle I0 must have one named issuer"
guard_expect_fixed_in_file "$TAG" 'consume_for_materializer' "$lifecycle" \
  "Residence lifecycle admission must have one consuming boundary"
guard_expect_fixed_in_file "$TAG" "exits: &'exits PreparedFunctionExitSetV1" "$lifecycle" \
  "Residence lifecycle admission must retain the exact exit-set borrow"
guard_expect_fixed_in_file "$TAG" 'callback(self.exits)' "$lifecycle" \
  "Residence lifecycle consumer must use the retained exit-set borrow"
guard_expect_fixed_in_file "$TAG" 'Result<(), String>' "$lifecycle" \
  "lifecycle callback must not return an exit aggregate"
if rg -U -n 'consume_for_materializer\([^)]*exits\s*:' "$lifecycle"; then
  guard_fail "$TAG" "Residence lifecycle consumer must not accept an external exit-set argument"
fi
if rg -U -n '#\[derive\(Debug, Clone, Copy, PartialEq, Eq\)\]\s*struct FunctionExitSetStampV1' "$lifecycle"; then
  guard_fail "$TAG" "Residence lifecycle provenance identity must remain non-copyable"
fi
if rg -n 'PinnedTextResidenceExitObligation|PinnedTextResidenceExitRow|rows:|TextContentFrame|Arc<|MirInstruction' "$lifecycle"; then
  guard_fail "$TAG" "Residence lifecycle I0 must not retain copied rows or open runtime/MIR owners"
fi
plan_bridge="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/session/pinned_text_plan.rs"
guard_expect_fixed_in_file "$TAG" 'bind_stamp_once' "$plan_bridge" \
  "canonical plan bridge must bind the source stamp before issuing rows"
plan_table="$ROOT_DIR/src/mir/pinned_text_access_plan.rs"
guard_expect_fixed_in_file "$TAG" 'bind_stamp_once' "$plan_table" \
  "plan table must reject the unpublished zero stamp"

residence_carrier="$ROOT_DIR/src/mir/pinned_text_residence_lifecycle.rs"
guard_expect_fixed_in_file "$TAG" 'PreparedPinnedTextResidenceLifecycleV1' "$residence_carrier" \
  "Residence lifecycle must have one private affine physical carrier"
guard_expect_fixed_in_file "$TAG" 'issue_from_frame' "$residence_carrier" \
  "Residence carrier must be issued from the existing frame/plan cohort"
if rg -n 'Arc<|RawPointer|ValueId|\*const|\*mut|MirInstruction|StringBox' "$residence_carrier"; then
  guard_fail "$TAG" "Residence carrier must not expose runtime/raw-value authorities"
fi

cfg_session="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_cfg/session.rs"
guard_expect_fixed_in_file "$TAG" 'emit_pinned_text_residence_enter' "$cfg_session" \
  "canonical CFG must own the Residence Enter writer"
guard_expect_fixed_in_file "$TAG" 'emit_pinned_text_residence_finish' "$cfg_session" \
  "canonical CFG must own the Residence Finish writer"

ssa_lifecycle="$ROOT_DIR/src/mir/builder/resolved_lowering/canonical_ssa/session/residence_lifecycle.rs"
guard_expect_fixed_in_file "$TAG" 'finish_emitted' "$ssa_lifecycle" \
  "canonical session must reject duplicate Finish consumption"
if rg -n 'Arc<|RawPointer|ValueId|\*const|\*mut|MirInstruction|StringBox' "$ssa_lifecycle"; then
  guard_fail "$TAG" "canonical lifecycle session must not own runtime/raw-value authorities"
fi

invocation_binding="$ROOT_DIR/src/mir/builder/pinned_text_invocation_binding.rs"
invocation_session="$ROOT_DIR/src/mir/builder/module_invocation_session.rs"
normal_root_lifecycle="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs"
signature_install="$ROOT_DIR/src/mir/normal_callable_semantic_package/install.rs"
guard_expect_fixed_in_file "$TAG" 'PinnedTextCompileInvocationBindingRefV1' "$invocation_binding" \
  "pinned-Text ingress must use one private session-scoped binding"
guard_expect_fixed_in_file "$TAG" 'PreparedPinnedTextPhysicalEntryIngressV1' "$invocation_binding" \
  "pinned-Text ingress must co-seal one affine physical-entry product"
guard_expect_fixed_in_file "$TAG" 'from_s6c_row' "$invocation_binding" \
  "physical signature must be adapted from the retained S6C loan"
guard_expect_fixed_in_file "$TAG" 'install_pinned_text_target_capability' "$invocation_session" \
  "module session must own the pinned-Text target capability"
guard_expect_fixed_in_file "$TAG" 'with_builder_and_pinned_text_invocation_binding' "$normal_root_lifecycle" \
  "normal-root lifecycle must lend the binding through the session"
guard_expect_fixed_in_file "$TAG" 'from_s6c_row' "$signature_install" \
  "signature adapter must preserve the package-owned row"
if rg -n 'PreparedDraftSealReturn|TextContentFrame|Arc<|nyash\.string\.eq_hh|MirInstruction|\bValueId\b|raw (slot|generation|token|pointer)' "$invocation_binding"; then
  guard_fail "$TAG" "pre-DraftSeal ingress must not open envelope/runtime/MIR/raw-value authorities"
fi
if rg -n 'brand\(\).*invocation_ordinal|invocation_ordinal\(\).*brand' \
  "$invocation_binding" "$invocation_session"; then
  guard_fail "$TAG" "module brand and target ordinals must never be compared"
fi

echo "[$TAG] ok (one ingress owner, one session owner, files below 800 lines)"
