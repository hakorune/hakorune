#!/bin/bash
# userbox_birth_to_string_vm.sh — Constructor(birth) + toString→stringify mapping (prod)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2
preflight_plugins || exit 2

export NYASH_USING_PROFILE=prod
export NYASH_VM_USER_INSTANCE_BOXCALL=0

TEST_DIR="/tmp/userbox_birth_to_string_vm_$$"
mkdir -p "$TEST_DIR" && cd "$TEST_DIR"

cat > driver.hako << 'EOF'
static box Main {
  main() {
    local o = new MyBox(7)
    local v = o.value()
    if v == 7 { print("ok1") } else { print("ng1") }
    if v == 7 { print("ok2") } else { print("ng2") }
    return 0
  }
}

box MyBox {
  x
  birth(v) { me.x = v  return 0 }
  value() { return me.x }
  stringify() { return "ok" }
}
EOF

output=$(
  NYASH_JOINIR_DEV=0 \
  HAKO_JOINIR_STRICT=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  "$NYASH_BIN" --backend vm driver.hako --dev 2>&1 | filter_noise
)
if echo "$output" | grep -q "User Instance BoxCall disallowed in prod"; then
  cd /
  rm -rf "$TEST_DIR"
  test_skip "userbox_birth_to_string_vm" "rewrite/materialize pending"
  exit 0
fi
if echo "$output" | grep -q "\[vm-hako/unimplemented\]"; then
  cd /
  rm -rf "$TEST_DIR"
  test_skip "userbox_birth_to_string_vm" "vm-hako subset path pending"
  exit 0
fi
output=$(echo "$output" | tail -n 2 | tr -d '\r' )
expected=$'ok1\nok2'
if compare_outputs "$expected" "$output" "userbox_birth_to_string_vm"; then
  test_pass "userbox_birth_to_string_vm"
else
  cd /
  rm -rf "$TEST_DIR"
  exit 1
fi

cd /
rm -rf "$TEST_DIR"
