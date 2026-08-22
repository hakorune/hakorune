#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

tag="script-direct-static-a-c-consumer-i0"

count_fixed() {
  local needle="$1"
  local file="$2"
  rg -F -c "$needle" "$file" 2>/dev/null || true
}

require_count() {
  local expected="$1"
  local needle="$2"
  local file="$3"
  local actual
  actual="$(count_fixed "$needle" "$file")"
  [[ "$actual" == "$expected" ]] || {
    echo "[$tag] expected $expected occurrence(s) of '$needle' in $file, got $actual" >&2
    exit 1
  }
}

issuer="src/mir/builder/normal_script_a/issuer.rs"
model="src/mir/builder/normal_script_a/model.rs"
required="src/mir/builder/normal_script_a/required_argument_source.rs"
consumer="src/mir/builder/normal_script_a/consumer.rs"
a_tests="src/mir/builder/normal_script_a/tests.rs"
pre_effect="src/mir/builder/normal_script_pre_effect_source_observation.rs"
lifecycle="src/mir/builder/normal_default_root_catalog_lifecycle.rs"
post_install="src/mir/builder/normal_default_root_catalog_post_install.rs"
lowering_input="src/mir/builder/normal_script_semantic_lowering_input.rs"
direct_input="src/mir/builder/normal_script_semantic_lowering_input/direct_static_claim_input.rs"
lowering_state="src/mir/builder/normal_script_semantic_lowering_state.rs"
profile="src/mir/resolved_semantics/shadow/traversal_profile.rs"
entry="src/mir/resolved_semantics/shadow/entry.rs"
card="docs/development/current/main/investigations/script-direct-static-a-source-window-coseal-i0-2026-08-21.md"

for file in \
  "$issuer" "$model" "$required" "$consumer" "$a_tests" "$pre_effect" \
  "$lifecycle" "$post_install" "$lowering_input" "$direct_input" "$lowering_state" \
  "$profile" "$entry" "$card"; do
  [[ -f "$file" ]] || {
    echo "[$tag] missing $file" >&2
    exit 1
  }
done

for file in "$issuer" "$model" "$required" "$consumer" "$pre_effect" "$lifecycle" "$post_install" "$lowering_input" "$direct_input" "$lowering_state" "$profile" "$entry"; do
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || {
    echo "[$tag] 760-line split trigger exceeded: $file ($lines)" >&2
    exit 1
  }
done

require_count 1 'struct CanonicalScriptASourceCapabilityV1 {' "$issuer"
require_count 1 'struct CanonicalScriptASourceCapabilityIssuerV1;' "$issuer"
require_count 1 'fn issue(' "$issuer"
require_count 1 'CanonicalScriptAObservationIssuerV1::consume(capability)' "$issuer"
require_count 1 'struct CanonicalScriptCDispositionIssuerV1;' "$issuer"
require_count 1 'CanonicalScriptCDispositionIssuerV1::consume(capability)' "$issuer"
require_count 1 'pub(in crate::mir::builder) fn issue_into_c_transport(' "$issuer"
require_count 1 'consume_into_lowering_source(' "$consumer"
require_count 1 'consume_into_lowering_source(admission)' "$post_install"
require_count 1 'issue_into_c_transport(observation)' "$lifecycle"
require_count 1 'ASTNode::MethodCall { .. } => matches!(self, Self::ScriptLexicalCoreV1)' "$profile"

rg -q 'CompleteNoDirectStaticClaims' "$direct_input"
rg -q 'DirectStaticClaims' "$direct_input"
rg -q 'CanonicalScriptANonDirectRowV1' "$direct_input"
rg -q 'CanonicalScriptCNoDirectClaimsV1' "$direct_input"
rg -q 'no_direct_arm_retains_each_explicit_non_direct_source_row' "$a_tests"
rg -q 'direct_arm_retains_candidate_rows_without_a_second_lookup' "$a_tests"
rg -q 'private capability production constructor/caller' "$card"
rg -q 'C transport production consumer' "$card"

for file in "$issuer" "$model" "$required"; do
  if rg -n 'ASTNode|ValueId|MirType|BasicBlockId|RecipeKey|JoinSig|prepare_script_recipe|fallback|retry|unwrap_or_default|as \*const' "$file"; then
    echo "[$tag] downstream, physical, or fallback authority leaked into $file" >&2
    exit 1
  fi
done

if rg -n 'ScriptDirectStaticClaimTakeV1::Absent|Option<VerifiedScriptDirectStatic|prepare_script_recipe|resolve_script\(|ScriptDirectStaticCallLookupIssuerV1::issue' "$post_install" "$consumer" "$lowering_input" "$direct_input" "$lowering_state"; then
  echo "[$tag] old optional/fallback/re-observation edge remains at the A/C consumer boundary" >&2
  exit 1
fi

if rg -n 'Option<.*(Bundle|Join|Recipe|RequiredArgument)' "$lowering_input" "$direct_input"; then
  echo "[$tag] parallel optional direct-static products remain in lowering transport" >&2
  exit 1
fi

echo "$tag guard: PASS"
