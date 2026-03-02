#!/usr/bin/env bash

# Resolve user-facing benchmark key aliases into dataset keys (bench file stems).
# Keep this mapping narrow and explicit for performance contract lanes.

perf_resolve_bench_dataset_key() {
  local key="$1"
  case "${key}" in
    chip8_kernel_small | chip8_kernel_small_hk | chip8_kernel_small_rk)
      printf '%s\n' "chip8_kernel_small"
      ;;
    kilo_kernel_small | kilo_kernel_small_hk | kilo_kernel_small_rk)
      printf '%s\n' "kilo_kernel_small"
      ;;
    *)
      printf '%s\n' "${key}"
      ;;
  esac
}

perf_is_supported_bench4_key() {
  local key="$1"
  case "${key}" in
    chip8_kernel_small | chip8_kernel_small_hk | chip8_kernel_small_rk | kilo_kernel_small | kilo_kernel_small_hk | kilo_kernel_small_rk)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}
