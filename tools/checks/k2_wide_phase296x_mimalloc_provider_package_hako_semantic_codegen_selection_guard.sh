#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-hako-semantic-codegen-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_32="docs/development/current/main/phases/phase-296x/296x-32-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
ARTIFACT_SSOT="docs/development/current/main/design/provider-package-artifact-ssot.md"
DOC="docs/reference/runtime/provider-package-v0.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_codegen_selection_guard.sh"

echo "[$TAG] checking phase-296x .hako semantic provider codegen selection"

guard_require_files "$TAG" "$CARD_32" "$TASKBOARD" "$ARTIFACT_SSOT" "$DOC" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_32" "selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION-296X-001' "$CARD_32" "selection card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'Select `ping-literal-v0`' "$CARD_32" "selection must choose ping-literal-v0"
guard_expect_fixed_in_file "$TAG" 'HakoProvider.ping/0' "$CARD_32" "selection must target HakoProvider.ping/0"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=ping-literal-v0' "$CARD_32" "selection must define semantic codegen output"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_codegen=1' "$CARD_32" "selection must define ping codegen evidence"
guard_expect_fixed_in_file "$TAG" 'provider_noop_call_result=<same i64>' "$CARD_32" "selection must require noop call value evidence"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_32" "selection must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_32" "selection must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT-296X-001' "$CARD_32" "selection must choose ping pilot"

guard_expect_fixed_in_file "$TAG" '## Phase C Semantic Codegen Step 1' "$ARTIFACT_SSOT" "artifact SSOT must document semantic step"
guard_expect_fixed_in_file "$TAG" 'ping-literal-v0' "$ARTIFACT_SSOT" "artifact SSOT must pin semantic mode"
guard_expect_fixed_in_file "$TAG" 'provider_noop_call_result=<same i64>' "$ARTIFACT_SSOT" "artifact SSOT must pin noop evidence"

guard_expect_fixed_in_file "$TAG" '--provider-package-hako-semantic-codegen ping-literal-v0' "$DOC" "reference docs must document semantic CLI mode"
guard_expect_fixed_in_file "$TAG" 'hako_provider_ping_value=7' "$DOC" "reference docs must document ping value evidence"

guard_expect_fixed_in_file "$TAG" '| 32 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 32 must be landed"
guard_expect_fixed_in_file "$TAG" '| 33 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 33 must be landed"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list semantic selection guard"

echo "[$TAG] ok"
