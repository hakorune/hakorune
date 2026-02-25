#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

if [[ ! -x "$BIN" ]]; then
  echo "[smoke] building hakorune (release, cranelift-jit)..." >&2
  (cd "$ROOT_DIR" && cargo build --release --features cranelift-jit >/dev/null)
fi

# Build required plugins (release)
build_plugin() {
  local dir=$1
  if [[ -d "$ROOT_DIR/$dir" ]]; then
    echo "[smoke] building $dir ..." >&2
    (cd "$ROOT_DIR/$dir" && cargo build --release >/dev/null)
  fi
}

build_plugin plugins/nyash-filebox-plugin
build_plugin plugins/nyash-console-plugin
build_plugin plugins/nyash-math-plugin

export NYASH_CLI_VERBOSE=1
export NYASH_PLUGIN_STRICT=1
export NYASH_MIR_PLUGIN_INVOKE=1
unset NYASH_USE_PLUGIN_BUILTINS
unset NYASH_PLUGIN_OVERRIDE_TYPES

mkdir -p "$ROOT_DIR/tmp"
echo -n "OK" > "$ROOT_DIR/tmp/plugin_smoke_filebox.txt"

FILEBOX_APP="$ROOT_DIR/tmp/plugin_smoke_filebox_min.hako"
MATH_APP="$ROOT_DIR/tmp/plugin_smoke_math_min.hako"
TIME_APP="$ROOT_DIR/tmp/plugin_smoke_time_min.hako"

cat > "$FILEBOX_APP" <<'EOF'
static box Main {
  main() {
    local path
    local f
    local s
    path = "tmp/plugin_smoke_filebox.txt"
    f = new FileBox()
    f.open(path, "r")
    s = f.read()
    f.close()
    return 0
  }
}
EOF

cat > "$MATH_APP" <<'EOF'
static box Main {
  main() {
    local m
    local x
    m = new MathBox()
    x = m.sqrt(9)
    return 0
  }
}
EOF

cat > "$TIME_APP" <<'EOF'
static box Main {
  main() {
    local t
    local n
    t = new TimeBox()
    n = t.now()
    return 0
  }
}
EOF

run_case() {
  local name=$1
  local file=$2
  local target=$file
  local out="/tmp/smoke_plugins_${name}.out"
  if [[ "$target" != /* ]]; then
    target="$ROOT_DIR/$target"
  fi
  echo "[smoke] case=$name file=$target" >&2
  set +e
  env -u NYASH_OPT_DIAG_FORBID_LEGACY "$BIN" --backend vm "$target" >"$out" 2>&1
  local rc=$?
  set -e
  if [[ $rc -ne 0 ]]; then
    echo "[smoke] FAIL: $name (rc=$rc)" >&2
    tail -n 60 "$out" || true
    return $rc
  fi
  echo "[smoke] ok: $name" >&2
}

# Core plugin demos (parser-stable fixtures)
run_case console_demo examples/console_demo.hako
run_case filebox_min "$FILEBOX_APP"
run_case math_min "$MATH_APP"
run_case time_min "$TIME_APP"

echo "[smoke] all green" >&2
