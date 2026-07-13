dev_gate_cmd_step "expected failure" "fixture failure" \
  bash -c 'echo failure-marker; exit 7'
dev_gate_cmd_step "must not execute" "fixture stop lock" \
  bash -c 'echo post-failure-marker; exit 99'
