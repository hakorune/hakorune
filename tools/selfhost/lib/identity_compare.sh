validate_emit_payload() {
  local subcmd="$1"
  local outfile="$2"
  local stage_label="$3"

  if [[ ! -s "$outfile" ]]; then
    echo "[G1:FAIL] ${stage_label} ${subcmd} produced empty output" >&2
    return 1
  fi

  if [[ "$subcmd" == "program-json" ]]; then
    if grep -Eq '^[[:space:]]*Result:[[:space:]]*[0-9]+' "$outfile"; then
      echo "[G1:FAIL] ${stage_label} program-json payload contains execution trailer (Result: ...)" >&2
      return 1
    fi
    if ! grep -Eq '^[[:space:]]*\{' "$outfile"; then
      echo "[G1:FAIL] ${stage_label} program-json does not start with JSON object" >&2
      return 1
    fi
    if ! grep -Eq '"kind"[[:space:]]*:[[:space:]]*"Program"' "$outfile"; then
      echo "[G1:FAIL] ${stage_label} program-json missing Program kind marker" >&2
      return 1
    fi
    if ! grep -Eq '"version"[[:space:]]*:[[:space:]]*0' "$outfile"; then
      echo "[G1:FAIL] ${stage_label} program-json missing version=0 marker" >&2
      return 1
    fi
  else
    if grep -Eq '^[[:space:]]*Result:[[:space:]]*[0-9]+' "$outfile"; then
      echo "[G1:FAIL] ${stage_label} mir-json payload contains execution trailer (Result: ...)" >&2
      return 1
    fi
    if ! grep -Eq '^[[:space:]]*\{' "$outfile"; then
      echo "[G1:FAIL] ${stage_label} mir-json does not start with JSON object" >&2
      return 1
    fi
    if ! grep -Eq '"functions"[[:space:]]*:' "$outfile"; then
      echo "[G1:FAIL] ${stage_label} mir-json missing functions marker" >&2
      return 1
    fi
  fi

  return 0
}

compare_mir_emit_outputs() {
  local label="$1"
  local lhs="$2"
  local rhs="$3"
  local diff_path="$4"
  local helper="$ROOT/tools/selfhost/lib/mir_canonical_compare.py"
  local lhs_canon rhs_canon raw_diff_path
  lhs_canon="$(mktemp)"
  rhs_canon="$(mktemp)"
  raw_diff_path="${diff_path}.raw"

  if ! python3 "$helper" canonicalize "$lhs" >"$lhs_canon"; then
    echo "[G1:FAIL] ${label} canonicalization failed for lhs" >&2
    rm -f "$lhs_canon" "$rhs_canon" 2>/dev/null || true
    return 1
  fi
  if ! python3 "$helper" canonicalize "$rhs" >"$rhs_canon"; then
    echo "[G1:FAIL] ${label} canonicalization failed for rhs" >&2
    rm -f "$lhs_canon" "$rhs_canon" 2>/dev/null || true
    return 1
  fi

  if ! diff -q "$lhs_canon" "$rhs_canon" >/dev/null 2>&1; then
    echo "[G1:FAIL] ${label} mismatch" >&2
    echo "         Canonical diff saved: ${diff_path}" >&2
    diff "$lhs_canon" "$rhs_canon" >"$diff_path" 2>&1 || true
    diff "$lhs" "$rhs" >"$raw_diff_path" 2>&1 || true
    echo "         Raw diff saved: ${raw_diff_path}" >&2
    rm -f "$lhs_canon" "$rhs_canon" 2>/dev/null || true
    return 1
  fi

  if diff -q "$lhs" "$rhs" >/dev/null 2>&1; then
    echo "[G1] ${label}: ✅ MATCH" >&2
    rm -f "$lhs_canon" "$rhs_canon" 2>/dev/null || true
    return 0
  fi

  diff "$lhs" "$rhs" >"$raw_diff_path" 2>&1 || true
  echo "[G1] ${label}: ✅ CANONICAL MATCH" >&2
  echo "     raw exact diff saved: ${raw_diff_path}" >&2
  rm -f "$lhs_canon" "$rhs_canon" 2>/dev/null || true
  return 0
}

compare_emit_outputs() {
  local label="$1"
  local lhs="$2"
  local rhs="$3"
  local diff_path="$4"

  if [[ "$label" == "MIR JSON v0" ]]; then
    compare_mir_emit_outputs "$label" "$lhs" "$rhs" "$diff_path"
    return $?
  fi

  if ! diff -q "$lhs" "$rhs" >/dev/null 2>&1; then
    echo "[G1:FAIL] ${label} mismatch" >&2
    echo "         Diff saved: ${diff_path}" >&2
    diff "$lhs" "$rhs" >"$diff_path" 2>&1 || true
    return 1
  fi

  echo "[G1] ${label}: ✅ MATCH" >&2
  return 0
}
