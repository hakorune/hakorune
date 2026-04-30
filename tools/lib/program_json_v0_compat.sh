#!/usr/bin/env bash
# program_json_v0_compat.sh — neutral Program(JSON v0) compat emit owner
#
# Keep the raw public compat CLI spelling in one shell helper while the
# remaining selfhost and smoke evidence lanes still need Program(JSON v0)
# payloads.

program_json_v0_compat_emit_to_file() {
  local bin="$1"
  local out_path="$2"
  local input_path="$3"
  "$bin" --emit-program-json-v0 "$out_path" "$input_path"
}

# Compatibility alias for older selfhost helper callsites.
selfhost_emit_program_json_v0_to_file() {
  program_json_v0_compat_emit_to_file "$@"
}
