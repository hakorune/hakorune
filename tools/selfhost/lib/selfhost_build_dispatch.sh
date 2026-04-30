#!/usr/bin/env bash
# selfhost_build_dispatch.sh — Final route dispatcher helpers
#
# Purpose:
# - Own the final output routing between Program(JSON path) and MIR emit.
# - Keep this helper separate from the producer / direct / exe artifact owners.

print_program_json_path_result() {
  local json_path="$1"
  echo "$json_path"
  return 0
}

dispatch_stageb_primary_output() {
  local json_path="$1"

  print_program_json_path_result "$json_path"
}

dispatch_stageb_downstream_outputs() {
  local json_path="$1"
  emit_requested_mir_output_if_needed
  dispatch_stageb_primary_output "$json_path"
}
