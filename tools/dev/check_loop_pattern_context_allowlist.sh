#!/usr/bin/env bash
set -euo pipefail

allowlist=(
  "src/mir/builder/control_flow/joinir/patterns/router.rs"
  "src/mir/builder/control_flow/plan/features/generic_loop_pipeline.rs"
  "src/mir/builder/control_flow/plan/features/loop_cond_return_in_body_pipeline.rs"
  "src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs"
  "src/mir/builder/control_flow/plan/normalizer/pattern1_coreloop_builder.rs"
)

mapfile -t files < <(rg -n "\\bLoopPatternContext\\b" src | cut -d: -f1 | sort -u)
if [[ ${#files[@]} -eq 0 ]]; then
  echo "[FAIL] LoopPatternContext not found; remove compatibility alias expectation first" >&2
  exit 1
fi

declare -A allow_map=()
for path in "${allowlist[@]}"; do
  allow_map["$path"]=1
done

unexpected=()
for path in "${files[@]}"; do
  if [[ -z "${allow_map[$path]+x}" ]]; then
    unexpected+=("$path")
  fi
done

if [[ ${#unexpected[@]} -gt 0 ]]; then
  echo "[FAIL] unexpected LoopPatternContext usage:" >&2
  for path in "${unexpected[@]}"; do
    echo "  - $path" >&2
  done
  exit 1
fi

echo "[PASS] LoopPatternContext allowlist (${#files[@]} files)"
for path in "${files[@]}"; do
  echo "  - $path"
done
