#!/usr/bin/env bash
# stageb_program_json_capture.sh — Stage-B stdout Program(JSON v0) extraction SSOT
#
# Purpose:
# - Extract the first balanced JSON object containing `"kind":"Program"` from
#   noisy Stage-B stdout.
# - Keep bracket/string escaping behavior identical across Stage-B delegate
#   scripts while Program(JSON v0) remains an explicit compat/debug keeper.

stageb_program_json_extract_from_stdin() {
  python3 -c '
import sys

stdin = sys.stdin.read()
start = stdin.find("\"kind\":\"Program\"")
if start < 0:
    sys.exit(1)

pos = start
depth = 0
while pos >= 0:
    if stdin[pos] == "{":
        depth += 1
        if depth == 1:
            break
    elif stdin[pos] == "}":
        depth -= 1
    pos -= 1

if pos < 0:
    sys.exit(1)

obj_start = pos
depth = 0
in_string = False
escape = False
i = obj_start

while i < len(stdin):
    ch = stdin[i]
    if escape:
        escape = False
    elif in_string:
        if ch == "\\":
            escape = True
        elif ch == "\"":
            in_string = False
    else:
        if ch == "\"":
            in_string = True
        elif ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                print(stdin[obj_start:i + 1])
                sys.exit(0)
    i += 1

sys.exit(1)
'
}
