#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

tag="mirbuilder-script-direct-static-semantic-shelf-r0"
shelf="src/mir/builder/normal_script/direct_static/semantic"
builder="src/mir/builder.rs"
lowering_state="src/mir/builder/normal_script_semantic_lowering_state.rs"

new_recipe="$shelf/normal_script_direct_static_recipe.rs"
new_recipe_tests="$shelf/normal_script_direct_static_recipe_tests.rs"
new_ledger="$shelf/normal_script_direct_static_claim_ledger.rs"
new_ledger_tests="$shelf/normal_script_direct_static_claim_ledger_tests.rs"

old_recipe="src/mir/builder/normal_script_direct_static_recipe.rs"
old_recipe_tests="src/mir/builder/normal_script_direct_static_recipe_tests.rs"
old_ledger="src/mir/builder/normal_script_direct_static_claim_ledger.rs"
old_ledger_tests="src/mir/builder/normal_script_direct_static_claim_ledger_tests.rs"

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

require_file() {
  [[ -f "$1" ]] || {
    echo "[$tag] missing required file: $1" >&2
    exit 1
  }
}

require_absent() {
  [[ ! -e "$1" ]] || {
    echo "[$tag] old physical path remains: $1" >&2
    exit 1
  }
}

require_lines() {
  local expected="$1"
  local file="$2"
  local actual
  actual="$(wc -l < "$file")"
  [[ "$actual" == "$expected" ]] || {
    echo "[$tag] expected preserved line count $expected for $file, got $actual" >&2
    exit 1
  }
  (( actual < 760 )) || {
    echo "[$tag] 760-line split trigger exceeded: $file ($actual)" >&2
    exit 1
  }
  (( actual < 800 )) || {
    echo "[$tag] 800-line hard stop exceeded: $file ($actual)" >&2
    exit 1
  }
}

require_sha256() {
  local expected="$1"
  local file="$2"
  local actual
  actual="$(sha256sum "$file" | awk '{print $1}')"
  [[ "$actual" == "$expected" ]] || {
    echo "[$tag] content hash drift in $file: expected $expected, got $actual" >&2
    exit 1
  }
}

production_count() {
  local needle="$1"
  rg -F -n "$needle" src/mir/builder --glob '*.rs' 2>/dev/null \
    | awk -F: '$1 !~ /_tests\.rs$/ { count++ } END { print count + 0 }'
}

require_production_count() {
  local expected="$1"
  local needle="$2"
  local actual
  actual="$(production_count "$needle")"
  [[ "$actual" == "$expected" ]] || {
    echo "[$tag] expected $expected production occurrence(s) of '$needle', got $actual" >&2
    exit 1
  }
}

for file in "$new_recipe" "$new_recipe_tests" "$new_ledger" "$new_ledger_tests"; do
  require_file "$file"
done

for file in "$old_recipe" "$old_recipe_tests" "$old_ledger" "$old_ledger_tests"; do
  require_absent "$file"
done

expected_files=$'normal_script_direct_static_claim_ledger.rs\nnormal_script_direct_static_claim_ledger_tests.rs\nnormal_script_direct_static_recipe.rs\nnormal_script_direct_static_recipe_tests.rs'
actual_files="$(find "$shelf" -maxdepth 1 -type f -printf '%f\n' | sort)"
[[ "$actual_files" == "$expected_files" ]] || {
  echo "[$tag] semantic shelf contains an unexpected file set" >&2
  printf 'expected:\n%s\nactual:\n%s\n' "$expected_files" "$actual_files" >&2
  exit 1
}

[[ ! -e "$shelf/mod.rs" ]] || {
  echo "[$tag] shelf facade mod.rs is forbidden" >&2
  exit 1
}
if rg -n '^[[:space:]]*pub[[:space:]]+' "$shelf"; then
  echo "[$tag] re-export or new shelf-owned child module is forbidden" >&2
  exit 1
fi
if rg -n '^[[:space:]]*mod[[:space:]]+[^;]+;' "$shelf" | grep -v 'mod tests;'; then
  echo "[$tag] unexpected shelf-owned child module is forbidden" >&2
  exit 1
fi
require_count 1 'mod tests;' "$new_recipe"
require_count 1 'mod tests;' "$new_ledger"

require_count 1 '#[path = "builder/normal_script/direct_static/semantic/normal_script_direct_static_recipe.rs"]' "$builder"
require_count 1 'mod normal_script_direct_static_recipe;' "$builder"
require_count 1 '#[path = "normal_script/direct_static/semantic/normal_script_direct_static_claim_ledger.rs"]' "$lowering_state"
require_count 1 'mod direct_static_claim_ledger;' "$lowering_state"

require_lines 739 "$builder"
require_lines 270 "$lowering_state"
require_lines 333 "$new_recipe"
require_lines 199 "$new_recipe_tests"
require_lines 424 "$new_ledger"
require_lines 235 "$new_ledger_tests"

require_sha256 2e0f4cf84425bfeb7366c90ee0a7fe2d1eb3930d24f1325fcd9fa054fd08e23e "$new_recipe"
require_sha256 13f8fa2b70cf5388eef664d709266109a928a4114f5f30b3a49ba90bd7006928 "$new_recipe_tests"
require_sha256 65b1f698bf12dc7a68440885d08f2d488ebac55242257cf374f922b67ffac614 "$new_ledger"
require_sha256 b69d227ad7699c598945aae23f831603960fa8b214aff15bc43ff5a3b020efe5 "$new_ledger_tests"

require_count 1 'pub(super) fn issue(' "$new_recipe"
require_count 1 'pub(super) fn issue_direct(' "$new_ledger"
require_count 1 'pub(super) fn complete_no_direct(' "$new_ledger"
require_production_count 1 'VerifiedScriptDirectStaticRecipeV1::issue'
require_production_count 1 'ScriptDirectStaticClaimLedgerV1::issue_direct'
require_production_count 1 'ScriptDirectStaticClaimLedgerV1::complete_no_direct'
require_production_count 1 'lower_claimed_script_direct_static_v1(self,'

physical_input_callers="$(rg -F -n 'lower_direct_static_physical_input_v1(' src/mir/builder --glob '*.rs' 2>/dev/null \
  | awk -F: '$1 !~ /direct_static_entry_kernel\.rs$/ && $1 !~ /_tests\.rs$/ { count++ } END { print count + 0 }')"
[[ "$physical_input_callers" == "0" ]] || {
  echo "[$tag] detached physical-input kernel has a production caller" >&2
  exit 1
}

if rg -n 'pub[[:space:]]+use|compatibility[[:space:]]+(shim|module)|old[-_ ]path|re-export' "$shelf"; then
  echo "[$tag] alias/re-export/compatibility shelf surface detected" >&2
  exit 1
fi

echo "$tag guard: PASS"
