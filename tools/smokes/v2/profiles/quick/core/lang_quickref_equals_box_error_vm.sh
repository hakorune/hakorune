#!/bin/bash
# lang_quickref_equals_box_error_vm.sh — guide equality comparisons on ordinary boxes

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/result_checker.sh"
require_env || exit 2
preflight_plugins || exit 2

# Enable 'box == box' guidance (as error) for this test
export NYASH_BOX_EQ_GUIDE_ERROR=1

TMP_DIR="/tmp/lang_quickref_equals_box_error_vm_$$"
mkdir -p "$TMP_DIR"
cat > "$TMP_DIR/code.hako" << 'EOF'
box A { }
static box Main { main(){ local x,y; x = new A(); y = new A(); if (x == y) { print("eq") } else { print("ne") } return 0 } }
EOF

if check_error_pattern "$TMP_DIR/code.hako" "Type error|Invalid|equals" "lang_quickref_equals_box_error_vm"; then
  rm -rf "$TMP_DIR"; exit 0
else
  rm -rf "$TMP_DIR"; exit 1
fi
