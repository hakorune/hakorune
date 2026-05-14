#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-310-ASTCLEAN-017-RUNNER-PROVIDER-RUNTIME-DEAD-CODE-RATIONALE-PASS.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-017 runner/provider/runtime dead_code rationale pass' "$ssot"; then
  echo '[astclean-runner-provider-runtime] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-310 ASTCLEAN-017 runner/provider/runtime dead_code rationale pass' "$card"; then
  echo '[astclean-runner-provider-runtime] missing phase card' >&2
  exit 1
fi

targets=(
  src/runner/modes/common_util/selfhost/stage0_capture_route.rs
  src/runner/modes/common_util/provider_registry.rs
  src/runner/box_index.rs
  src/runner/modes/common_util/io.rs
  src/runner/json_v0_bridge/lowering.rs
  src/runner/repl/repl_runner.rs
  src/runner/child_env.rs
  src/runner/pipeline.rs
  src/runner/json_v1_bridge/parse/mod.rs
  src/runner/stage1_bridge/modules.rs
  src/runner/modes/common_util/resolve/context.rs
  src/runner/modes/common_util/resolve/using_resolution.rs
  src/runtime/plugin_loader_v2/enabled/loader/error_reporter.rs
  src/runtime/plugin_loader_v2/enabled/method_resolver.rs
  src/runtime/plugin_loader_v2/enabled/loader/specs.rs
)

if rg -n '#\[allow\(dead_code\)\]' "${targets[@]}" | grep -v 'ASTCLEAN-017' >/dev/null; then
  echo '[astclean-runner-provider-runtime] bare target dead_code allowance remains' >&2
  rg -n '#\[allow\(dead_code\)\]' "${targets[@]}" | grep -v 'ASTCLEAN-017' >&2
  exit 1
fi

if rg -n 'ProviderDescriptor|status_ok|is_special_method|get_special_method_id|BridgeEnv::load|post_run_exit_if_oob_strict_triggered|suggest_in_base' src/runner src/runtime/plugin_loader_v2/enabled >/dev/null; then
  echo '[astclean-runner-provider-runtime] stale helper surface returned' >&2
  rg -n 'ProviderDescriptor|status_ok|is_special_method|get_special_method_id|BridgeEnv::load|post_run_exit_if_oob_strict_triggered|suggest_in_base' src/runner src/runtime/plugin_loader_v2/enabled >&2
  exit 1
fi

for live in read_filebox_mode_from_env select_file_provider build_stage0_non_vm_capture_command try_parse_v1_to_module; do
  if ! rg -n "\b${live}\b" src/runner >/dev/null; then
    echo "[astclean-runner-provider-runtime] expected live surface missing: $live" >&2
    exit 1
  fi
done

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 132 ]; then
  echo "[astclean-runner-provider-runtime] expected source allowance count <= 132, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-runner-provider-runtime] OK source_count=$count"
