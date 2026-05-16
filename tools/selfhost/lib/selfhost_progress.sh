#!/usr/bin/env bash
# selfhost_progress.sh - stable phase progress helpers for selfhost wrappers.

selfhost_progress_now_ms() {
  date +%s%3N
}

selfhost_progress_record() {
  local phase="$1"
  local state="$2"
  local elapsed_ms="${3:-}"
  local file="${HAKO_SELFHOST_PROGRESS_FILE:-}"

  if [ -z "$file" ]; then
    return 0
  fi

  mkdir -p "$(dirname "$file")" 2>/dev/null || true
  if [ -n "$elapsed_ms" ]; then
    printf 'phase=%s state=%s elapsed_ms=%s\n' "$phase" "$state" "$elapsed_ms" >"$file"
  else
    printf 'phase=%s state=%s\n' "$phase" "$state" >"$file"
  fi
}

selfhost_phase_start() {
  local phase="$1"
  __SELFHOST_PHASE_NAME="$phase"
  __SELFHOST_PHASE_START_MS="$(selfhost_progress_now_ms)"
  echo "[selfhost] phase=$phase start" >&2
  selfhost_progress_record "$phase" "start"
}

selfhost_phase_done() {
  local phase="$1"
  local now start elapsed
  now="$(selfhost_progress_now_ms)"
  start="${__SELFHOST_PHASE_START_MS:-$now}"
  elapsed=$((now - start))
  echo "[selfhost] phase=$phase done elapsed_ms=$elapsed" >&2
  selfhost_progress_record "$phase" "done" "$elapsed"
}

selfhost_phase_fail() {
  local phase="$1"
  local rc="$2"
  local now start elapsed
  now="$(selfhost_progress_now_ms)"
  start="${__SELFHOST_PHASE_START_MS:-$now}"
  elapsed=$((now - start))
  echo "[selfhost] phase=$phase fail rc=$rc elapsed_ms=$elapsed" >&2
  selfhost_progress_record "$phase" "fail" "$elapsed"
}
