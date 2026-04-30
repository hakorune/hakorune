#!/usr/bin/env bash
# program_json_v0_compat.sh — selfhost Program(JSON v0) compat emit owner
#
# Keep the raw public compat CLI surface in one selfhost helper while Stage-B
# and explicit Stage1 contract probes still need Program(JSON v0) payloads.

selfhost_emit_program_json_v0_to_file() {
  local bin="$1"
  local out_path="$2"
  local input_path="$3"
  "$bin" --emit-program-json-v0 "$out_path" "$input_path"
}
