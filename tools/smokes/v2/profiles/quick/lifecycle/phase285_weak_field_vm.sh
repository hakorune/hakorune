#!/bin/bash
# phase285_weak_field_vm.sh - Phase 285A1: Weak Field Contract smoke test (VM)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

# Test 1: OK cases (should compile and run)
for fixture in explicit transfer void; do
    FIXTURE="$NYASH_ROOT/apps/tests/phase285_weak_field_ok_${fixture}.hako"

    if ! output=$(NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$FIXTURE" 2>&1); then
        log_error "phase285_weak_field_vm: OK case '${fixture}' failed to compile"
        echo "$output"
        exit 1
    fi
done

# Test 2: NG cases (should fail to compile)
for fixture in boxref primitive; do
    FIXTURE="$NYASH_ROOT/apps/tests/phase285_weak_field_ng_${fixture}.hako"

    if output=$(NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$FIXTURE" 2>&1); then
        log_error "phase285_weak_field_vm: NG case '${fixture}' should have failed"
        exit 1
    fi

    if ! echo "$output" | grep -q "weak"; then
        log_error "phase285_weak_field_vm: NG case '${fixture}' missing 'weak' in error"
        echo "$output"
        exit 1
    fi
done

# Test 3: Phase 285A1.3 - Visibility block and mixed members
for fixture in visibility_block mixed_members; do
    FIXTURE="$NYASH_ROOT/apps/tests/phase285_weak_${fixture}.hako"

    if ! output=$(NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$FIXTURE" 2>&1); then
        log_error "phase285_weak_field_vm: A1.3 case '${fixture}' failed to compile"
        echo "$output"
        exit 1
    fi
done

# Test 4: Phase 285A1.4 - Sugar syntax OK case
FIXTURE="$NYASH_ROOT/apps/tests/phase285_visibility_weak_sugar_ok.hako"
if ! output=$(NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$FIXTURE" 2>&1); then
    log_error "phase285_weak_field_vm: A1.4 sugar syntax OK case failed to compile"
    echo "$output"
    exit 1
fi

# Test 5: Phase 285A1.4 - Sugar syntax NG case (should fail to compile)
FIXTURE="$NYASH_ROOT/apps/tests/phase285_visibility_weak_sugar_ng.hako"
if output=$(NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$FIXTURE" 2>&1); then
    log_error "phase285_weak_field_vm: A1.4 sugar syntax NG case should have failed"
    exit 1
fi

if ! echo "$output" | grep -q "weak"; then
    log_error "phase285_weak_field_vm: A1.4 sugar syntax NG case missing 'weak' in error"
    echo "$output"
    exit 1
fi

log_success "phase285_weak_field_vm: All weak field contract tests passed (8 tests: 3 OK, 2 NG, 2 A1.3, 1 OK + 1 NG sugar)"
exit 0
