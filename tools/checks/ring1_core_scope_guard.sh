#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/design/ring1-core-provider-scope-ssot.md"
PROMOTION_TEMPLATE_DOC="$ROOT_DIR/docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md"
DRYRUN_TASK_PACK_DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md"
PHASE29Y_README_DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/README.md"
PHASE29Y_NEXT_DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md"
RING1_MOD="$ROOT_DIR/src/providers/ring1/mod.rs"
RING1_README="$ROOT_DIR/src/providers/ring1/README.md"
DOMAIN_SSV="$ROOT_DIR/tools/checks/ring1_domains.tsv"
source "$(dirname "$0")/lib/guard_common.sh"

TAG="ring1-core-scope-guard"
ACCEPTED_DOMAINS=()
EXPECTED_ACCEPTED_CSV="array,console,file,map,path"

sorted_csv() {
  if [ "$#" -eq 0 ]; then
    echo ""
    return 0
  fi
  printf '%s\n' "$@" | sort | paste -sd',' -
}

cd "$ROOT_DIR"

echo "[$TAG] checking ring1 core provider scope"

guard_require_command "$TAG" rg
guard_require_command "$TAG" sort
guard_require_command "$TAG" paste
guard_require_files "$TAG" \
  "$DOC" \
  "$PROMOTION_TEMPLATE_DOC" \
  "$DRYRUN_TASK_PACK_DOC" \
  "$PHASE29Y_README_DOC" \
  "$PHASE29Y_NEXT_DOC" \
  "$RING1_MOD" \
  "$RING1_README" \
  "$DOMAIN_SSV"

while IFS=$'\t' read -r domain status; do
  if [[ -z "${domain:-}" || "${domain:0:1}" == "#" ]]; then
    continue
  fi
  case "$status" in
    accepted) ACCEPTED_DOMAINS+=("$domain") ;;
    *)
      guard_fail "$TAG" "ring1_domains.tsv must contain accepted only after RING1-CORE-09: status=${status} domain=${domain} file=tools/checks/ring1_domains.tsv"
      ;;
  esac
done < "$DOMAIN_SSV"

if [ "${#ACCEPTED_DOMAINS[@]}" -eq 0 ]; then
  guard_fail "$TAG" "ring1_domains.tsv must contain at least one accepted domain"
fi

actual_accepted_csv="$(sorted_csv "${ACCEPTED_DOMAINS[@]}")"
if [ "$actual_accepted_csv" != "$EXPECTED_ACCEPTED_CSV" ]; then
  guard_fail "$TAG" "accepted domains mismatch: expected=${EXPECTED_ACCEPTED_CSV} actual=${actual_accepted_csv} (sync tools/checks/ring1_domains.tsv + src/providers/ring1/mod.rs + scope SSOT)"
fi

# SSOT Decision matrix must keep accepted domain set in sync.
for domain in "${ACCEPTED_DOMAINS[@]}"; do
  guard_expect_in_file "$TAG" "\\| \`${domain}\` \\| \`accepted\` \\|" "$DOC" "SSOT missing ${domain}=accepted decision"
done
guard_expect_in_file "$TAG" 'ring1-core-provider-promotion-template-ssot.md' "$DOC" "scope SSOT must reference promotion template SSOT"
guard_expect_in_file "$TAG" 'Domain Dry-Run Checklist' "$PROMOTION_TEMPLATE_DOC" "promotion template must contain domain dry-run checklist section"
guard_expect_in_file "$TAG" 'Commit Boundary Lock \(min1/min2/min3\)' "$PROMOTION_TEMPLATE_DOC" "promotion template must contain commit boundary section"
guard_expect_in_file "$TAG" '85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md' "$PROMOTION_TEMPLATE_DOC" "promotion template must reference ring1 dry-run task packs"
guard_expect_in_file "$TAG" 'ring1-core-provider-promotion-template-ssot.md' "$PHASE29Y_README_DOC" "phase-29y README must reference ring1 promotion template SSOT"
guard_expect_in_file "$TAG" '85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md' "$PHASE29Y_README_DOC" "phase-29y README must reference ring1 dry-run task packs"
guard_expect_in_file "$TAG" 'Ring1 Promotion Commit Boundary Lock' "$PHASE29Y_NEXT_DOC" "phase-29y next plan must include ring1 promotion commit boundary section"
guard_expect_in_file "$TAG" '85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md' "$PHASE29Y_NEXT_DOC" "phase-29y next plan must reference ring1 dry-run task packs"

# ring1 module export must match accepted set exactly.
for domain in "${ACCEPTED_DOMAINS[@]}"; do
  guard_expect_in_file "$TAG" "^pub mod ${domain};" "$RING1_MOD" "src/providers/ring1/mod.rs must export accepted domain: ${domain}"
done

actual_exported_csv="$(sed -n 's/^[[:space:]]*pub mod \([a-z0-9_]\+\);/\1/p' "$RING1_MOD" | sort | paste -sd',' -)"
if [ "$actual_exported_csv" != "$EXPECTED_ACCEPTED_CSV" ]; then
  guard_fail "$TAG" "ring1 mod exports mismatch: expected=${EXPECTED_ACCEPTED_CSV} actual=${actual_exported_csv} file=src/providers/ring1/mod.rs"
fi

check_accepted_domain_active() {
  local domain="$1"
  local service_symbol="$2"
  local runtime_pattern="$3"
  local domain_dir="$ROOT_DIR/src/providers/ring1/$domain"
  local domain_readme="$domain_dir/README.md"

  guard_require_files "$TAG" "$domain_readme" "$domain_dir/mod.rs"
  guard_expect_in_file "$TAG" 'Decision: `accepted`' "$domain_readme" "$domain README must declare accepted status"
  guard_expect_in_file "$TAG" "$service_symbol" "$domain_dir/mod.rs" "$domain provider implementation must define $service_symbol"
  if ! rg -n "$runtime_pattern" src/runtime >/dev/null 2>&1; then
    guard_fail "$TAG" "$domain domain wiring is missing: pattern=${runtime_pattern} search_root=src/runtime (run: rg -n \"${runtime_pattern}\" src/runtime)"
  fi
}

# Accepted domains must be active (provider file + runtime wiring + active README).
check_accepted_domain_active "array" "Ring1ArrayService" "providers::ring1::array"
check_accepted_domain_active "map" "Ring1MapService" "providers::ring1::map"
check_accepted_domain_active "path" "Ring1PathService" "providers::ring1::path"
check_accepted_domain_active "console" "Ring1ConsoleService" "providers::ring1::console"

# Ensure file domain remains actually wired in runtime path.
if ! rg -n "providers::ring1::file" src/runtime src/boxes >/dev/null 2>&1; then
  guard_fail "$TAG" "file domain wiring is missing: pattern=providers::ring1::file search_root=src/runtime src/boxes"
fi

echo "[$TAG] ok"
