#!/bin/bash
# Shared VM lane env assignments for Phase 21.5 perf smokes.

perf_vm_lane_env_assignments() {
  PERF_VM_LANE_ENV=(
    NYASH_VM_HAKO_PREFER_STRICT_DEV=0
    NYASH_VM_USE_FALLBACK=0
    NYASH_JOINIR_DEV=0
    NYASH_JOINIR_STRICT=0
  )
}

perf_vm_lane_run() {
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_VM_USE_FALLBACK=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  "$@"
}
