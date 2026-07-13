dev_gate_cmd_step "quiet success" "fixture success" \
  bash -c 'echo hidden-stdout; echo hidden-stderr >&2'
