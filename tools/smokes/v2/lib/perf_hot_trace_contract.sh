#!/bin/bash
# perf_hot_trace_contract.sh
# Shared helpers for LLVM hot-trace contract smokes.

perf_hot_trace_require_file() {
  local smoke_name="$1"
  local file_path="$2"
  if [ ! -f "$file_path" ]; then
    test_fail "$smoke_name: missing file: $file_path"
    return 1
  fi
}

perf_hot_trace_require_llvmlite_backend() {
  local smoke_name="$1"
  local env_name="$2"
  local backend="$3"
  if [ "$backend" != "llvmlite" ]; then
    test_fail "$smoke_name: ${env_name} must be llvmlite (got: $backend)"
    return 1
  fi
}

perf_hot_trace_require_boundary_backend() {
  local smoke_name="$1"
  local env_name="$2"
  local backend="$3"
  if [ -n "$backend" ] && [ "$backend" != "crate" ]; then
    test_fail "$smoke_name: ${env_name} must be crate or unset (got: $backend)"
    return 1
  fi
}

perf_hot_trace_require_uint_env() {
  local smoke_name="$1"
  local env_name="$2"
  local value="$3"
  case "$value" in
    ''|*[!0-9]*)
      test_fail "$smoke_name: ${env_name} must be uint (got: $value)"
      return 1
      ;;
  esac
}

perf_hot_trace_extract_field() {
  local line="$1"
  local key="$2"
  printf '%s\n' "$line" | sed -n -E "s/.*${key}=([0-9]+).*/\\1/p"
}

perf_hot_trace_require_numeric_field() {
  local smoke_name="$1"
  local line="$2"
  local key="$3"
  local value
  value="$(perf_hot_trace_extract_field "$line" "$key")"
  if [ -z "$value" ]; then
    printf '%s\n' "$line"
    test_fail "$smoke_name: missing numeric field $key in hot summary"
    return 1
  fi
}

perf_hot_trace_load_fields() {
  local trace_py="$1"
  python3 - "$trace_py" <<'PY'
import importlib.util
import pathlib
import sys

trace_path = pathlib.Path(sys.argv[1]).resolve()
sys.path.insert(0, str(trace_path.parent))
spec = importlib.util.spec_from_file_location("nyash_llvm_trace", trace_path)
if spec is None or spec.loader is None:
    raise SystemExit(1)
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)
fields = getattr(mod, "HOT_SUMMARY_FIELDS", ())
print(" ".join(str(x) for x in fields))
PY
}

perf_hot_trace_assert_aot_ok() {
  local smoke_name="$1"
  local key="$2"
  local out="$3"
  if ! printf '%s\n' "$out" | grep -q "\\[bench\\] name=${key} (aot).*status=ok"; then
    printf '%s\n' "$out"
    test_fail "$smoke_name: missing AOT status=ok line"
    return 1
  fi
}

perf_hot_trace_find_aot_line() {
  local out="$1"
  local key="$2"
  printf '%s\n' "$out" | grep "\\[bench\\] name=${key} (aot)" | tail -n 1 || true
}

perf_hot_trace_extract_aot_ms() {
  local aot_line="$1"
  printf '%s\n' "$aot_line" | sed -n -E 's/.*ny_aot_ms=([0-9]+).*/\1/p'
}

perf_hot_trace_assert_aot_ceiling() {
  local smoke_name="$1"
  local key="$2"
  local out="$3"
  local max_aot_ms="$4"

  perf_hot_trace_assert_aot_ok "$smoke_name" "$key" "$out" || return 1

  local aot_line
  aot_line="$(perf_hot_trace_find_aot_line "$out" "$key")"
  if [ -z "$aot_line" ]; then
    printf '%s\n' "$out"
    test_fail "$smoke_name: missing AOT bench line"
    return 1
  fi

  local aot_ms
  aot_ms="$(perf_hot_trace_extract_aot_ms "$aot_line")"
  if [ -z "$aot_ms" ]; then
    printf '%s\n' "$out"
    test_fail "$smoke_name: failed to parse ny_aot_ms"
    return 1
  fi

  if [ "$aot_ms" -gt "$max_aot_ms" ]; then
    printf '%s\n' "$out"
    test_fail "$smoke_name: ny_aot_ms=${aot_ms} exceeds max=${max_aot_ms}"
    return 1
  fi
}

perf_hot_trace_find_main_hot_line() {
  local trace_file="$1"
  grep -E '^\[llvm/hot\] fn=main ' "$trace_file" | tail -n 1 || true
}
