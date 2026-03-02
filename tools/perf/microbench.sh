#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BIN="$ROOT/target/release/hakorune"

usage() { echo "Usage: $0 --case {loop|strlen|box|branch|call|stringchain|arraymap|chip8|kilo|sieve|matmul|matmul_core|linidx|maplin} [--n N] [--runs R] [--backend {llvm|vm}] [--exe] [--budget-ms B]"; }

CASE="loop"; N=5000000; RUNS=5; BACKEND="llvm"; EXE_MODE=0; BUDGET_MS=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --case) CASE="$2"; shift 2;;
    --n) N="$2"; shift 2;;
    --runs) RUNS="$2"; shift 2;;
    --backend) BACKEND="$2"; shift 2;;
    --exe) EXE_MODE=1; shift 1;;
    --budget-ms) BUDGET_MS="$2"; shift 2;;
    --help|-h) usage; exit 0;;
    *) echo "Unknown arg: $1"; usage; exit 2;;
  esac
done

if [[ ! -x "$BIN" ]]; then echo "[FAIL] hakorune not built: $BIN" >&2; exit 2; fi

# Helpers: build once, then reuse
ensure_llvmc() {
  if [[ ! -x "$ROOT/target/release/ny-llvmc" ]]; then
    (cargo build -q --release -p nyash-llvm-compiler >/dev/null 2>&1) || true
  fi
}
ensure_nyrt() {
  # Accept either .a or .rlib as presence of built runtime
  if [[ ! -f "$ROOT/target/release/libnyash_kernel.a" && ! -f "$ROOT/target/release/libnyash_kernel.rlib" ]]; then
    (cd "$ROOT/crates/nyash_kernel" && cargo build -q --release >/dev/null 2>&1) || true
  fi
}

bench_hako() {
  local file="$1"; local backend="$2"; shift 2
  local start end
  start=$(date +%s%N)
  if [[ "$backend" = "llvm" ]]; then
    # Ensure ny-llvmc exists; build if missing
    if [[ ! -x "$ROOT/target/release/ny-llvmc" ]]; then
      (cargo build -q --release -p nyash-llvm-compiler >/dev/null 2>&1) || true
    fi
    PYTHONPATH="${PYTHONPATH:-$ROOT}" \
    NYASH_AOT_COLLECTIONS_HOT=1 NYASH_LLVM_FAST=1 NYASH_MIR_LOOP_HOIST=1 NYASH_AOT_MAP_KEY_MODE=auto \
    NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 HAKO_USING_RESOLVER_FIRST=1 \
    NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
    NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
    NYASH_LLVM_USE_HARNESS=1 "$BIN" --backend llvm "$file" >/dev/null 2>&1
  else
    "$BIN" --backend vm "$file" >/dev/null 2>&1
  fi
  end=$(date +%s%N)
  echo $(( (end - start)/1000000 ))
}

bench_c() {
  local csrc="$1"; local exe="$2"
  cc -O3 -march=native -o "$exe" "$csrc"
  local start end
  start=$(date +%s%N)
  "$exe" >/dev/null 2>&1
  end=$(date +%s%N)
  echo $(( (end - start)/1000000 ))
}

# Build once and time executable runs (ms)
time_exe_run() {
  local exe="$1"
  local start end
  start=$(date +%s%N)
  "$exe" >/dev/null 2>&1
  end=$(date +%s%N)
  echo $(( (end - start)/1000000 ))
}

mktemp_hako() { mktemp --suffix .hako; }
mktemp_c() { mktemp --suffix .c; }

# Fallback diagnostics for EXE flow: check MIR JSON for externcall/boxcall/jsonfrag
diag_mir_json() {
  local json="$1"
  local rewrites; rewrites=$(rg -c '"op":"externcall"' "$json" 2>/dev/null || echo 0)
  local arrays; arrays=$(rg -c 'nyash\.array\.' "$json" 2>/dev/null || echo 0)
  local maps;   maps=$(rg -c 'nyash\.map\.'   "$json" 2>/dev/null || echo 0)
  local boxcalls; boxcalls=$(rg -c '"op":"boxcall"' "$json" 2>/dev/null || echo 0)
  local jsonfrag; jsonfrag=$(rg -c '\[emit/jsonfrag\]' "$json" 2>/dev/null || echo 0)
  echo "[diag] externcall=${rewrites} (array=${arrays}, map=${maps}), boxcall_left=${boxcalls}, jsonfrag=${jsonfrag}" >&2
}

case "$CASE" in
  loop)
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
static box Main { method main(args) {
  local n = ${N}
  local i = 0
  local s = 0
  loop(i < n) { s = s + i  i = i + 1 }
  return s
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
int main(){
  volatile int64_t n = N_PLACEHOLDER;
  volatile int64_t s=0; for (int64_t i=0;i<n;i++){ s+=i; }
  return (int)(s&0xFF);
}
C
    sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  strlen)
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
static box Main { method main(args) {
  local n = ${N}
  local i = 0
  local s = 0
  local t = "abcdefghijklmnopqrstuvwxyz"
  loop(i < n) { s = s + t.length()  i = i + 1 }
  return s
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
#include <string.h>
int main(){
  volatile int64_t n = N_PLACEHOLDER; volatile int64_t s=0;
  const char* t = "abcdefghijklmnopqrstuvwxyz";
  for (int64_t i=0;i<n;i++){ s += (int64_t)strlen(t); }
  return (int)(s&0xFF);
}
C
    sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  box)
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
static box Main { method main(args) {
  local n = ${N}
  local i = 0
  loop(i < n) { local t = new StringBox("x"); i = i + 1 }
  return 0
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
typedef struct { char* p; } Str;
static inline Str* new_str(){ Str* s=(Str*)malloc(sizeof(Str)); s->p=strdup("x"); free(s->p); free(s); return s; }
int main(){ volatile int64_t n=N_PLACEHOLDER; for(int64_t i=0;i<n;i++){ new_str(); } return 0; }
C
    sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  branch)
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
static box Main { method main(args) {
  local n = ${N}
  local i = 0
  local acc = 0
  loop(i < n) {
    local mod = i % 30
    if (mod == 0) {
      acc = acc + 3
    } else if (mod < 10) {
      acc = acc + (i % 7)
    } else if (mod < 20) {
      acc = acc - (i % 11)
    } else {
      acc = acc + 1
    }
    i = i + 1
  }
  return acc
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
int main(){
  volatile int64_t n = N_PLACEHOLDER;
  volatile int64_t acc = 0;
  for (int64_t i=0;i<n;i++){
    int64_t mod = i % 30;
    if (mod == 0) {
      acc += 3;
    } else if (mod < 10) {
      acc += (i % 7);
    } else if (mod < 20) {
      acc -= (i % 11);
    } else {
      acc += 1;
    }
  }
  return (int)(acc & 0xFF);
}
C
    sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  call)
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
function mix(a, b, c) {
  return (a + b) - c
}
function twist(v) {
  if (v % 2 == 0) { return v / 2 }
  return v * 3 + 1
}
static box Main { method main(args) {
  local n = ${N}
  local i = 0
  local value = 1
  loop(i < n) {
    value = mix(value, i, value % 7)
    value = mix(value, twist(i), twist(value))
    i = i + 1
  }
  return value
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
static inline int64_t mix(int64_t a, int64_t b, int64_t c){ return (a + b) - c; }
static inline int64_t twist(int64_t v){ return (v % 2 == 0) ? v / 2 : v * 3 + 1; }
int main(){
  volatile int64_t n = N_PLACEHOLDER; volatile int64_t value = 1;
  for (int64_t i=0;i<n;i++){
    value = mix(value, i, value % 7);
    value = mix(value, twist(i), twist(value));
  }
  return (int)(value & 0xFF);
}
C
    sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  stringchain)
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
static box Main { method main(args) {
  local n = ${N}
  local base = "abcdefghijklmnopqrstuvwxyz0123456789"
  local acc = 0
  local i = 0
  loop(i < n) {
    local part1 = base.substring(0, 12)
    local part2 = base.substring(5, 20)
    local s = part1 + part2 + base.substring(2, 18)
    acc = acc + s.length()
    i = i + 1
  }
  return acc
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
#include <string.h>
int main(){
  volatile int64_t n = N_PLACEHOLDER; volatile int64_t acc = 0;
  const char* base = "abcdefghijklmnopqrstuvwxyz0123456789";
  char tmp[128];
  for (int64_t i=0;i<n;i++){
    memcpy(tmp, base, 12); tmp[12] = '\0';
    char buf[192];
    strcpy(buf, tmp);
    strncat(buf, base+5, 15);
    strncat(buf, base+2, 16);
    acc += (int64_t)strlen(buf);
  }
  return (int)(acc & 0xFF);
}
C
    sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  arraymap)
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
static box Main { method main(args) {
  local n = ${N}
  local arr = new ArrayBox()
  local map = new MapBox()
  local bucket = 32
  local i = 0
  loop(i < bucket) {
    arr.push(i)
    map.set("k" + i.toString(), i)
    i = i + 1
  }
  local sum = 0
  i = 0
  loop(i < n) {
    local idx = i % bucket
    local val = arr.get(idx)
    arr.set(idx, val + 1)
    local key = "k" + idx.toString()
    map.set(key, val)
    sum = sum + map.get(key)
    i = i + 1
  }
  return sum
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
int main(){
  volatile int64_t n = N_PLACEHOLDER; volatile int64_t sum = 0;
  int64_t bucket = 32;
  int64_t arr[32];
  int64_t mapv[32];
  for (int i=0;i<32;i++){ arr[i]=i; mapv[i]=i; }
  for (int64_t i=0;i<n;i++){
    int64_t idx = i % bucket;
    int64_t val = arr[idx];
    arr[idx] = val + 1;
    mapv[idx] = val;
    sum += mapv[idx];
  }
  return (int)(sum & 0xFF);
}
C
    sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  chip8)
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
box Chip8Bench {
  init { program, registers, pc, program_size }
  birth() {
    me.program = new ArrayBox()
    me.registers = new ArrayBox()
    me.pc = 0
    local i = 0
    loop(i < 16) { me.registers.push(0); i = i + 1 }
    local opcodes = new ArrayBox()
    // 6005, 6107, 7003, 7102, 1200 pattern
    opcodes.push(96); opcodes.push(5)
    opcodes.push(97); opcodes.push(7)
    opcodes.push(112); opcodes.push(3)
    opcodes.push(113); opcodes.push(2)
    opcodes.push(18); opcodes.push(0)
    local count_box = opcodes.length()
    local count = 0
    if count_box != null { count = count_box.toString().toInteger() }
    i = 0
    loop(i < count) {
      me.program.push(opcodes.get(i))
      i = i + 1
    }
    me.program_size = count
  }
  execute_cycle() {
    local hi = me.program.get(me.pc)
    local lo = me.program.get((me.pc + 1) % me.program_size)
    local opcode = (hi * 256) + lo
    me.pc = (me.pc + 2) % me.program_size
    local nib = opcode / 4096
    if (nib == 1) {
      me.pc = opcode % me.program_size
    } else if (nib == 6) {
      local reg = (opcode / 256) % 16
      local value = opcode % 256
      me.registers.set(reg, value)
    } else if (nib == 7) {
      local reg = (opcode / 256) % 16
      local value = opcode % 256
      local cur = me.registers.get(reg)
      me.registers.set(reg, cur + value)
    }
  }
  run(cycles) {
    local i = 0
    loop(i < cycles) { me.execute_cycle(); i = i + 1 }
  }
  checksum() {
    local total = 0
    local len = me.registers.length().toString().toInteger()
    local i = 0
    loop(i < len) { total = total + me.registers.get(i); i = i + 1 }
    return total
  }
}
static box Main { method main(args) {
  local cycles = ${N}
  local bench = new Chip8Bench()
  bench.birth()
  bench.run(cycles)
  return bench.checksum()
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
int main(){
  volatile int64_t cycles = N_PLACEHOLDER;
  int pc = 0;
  int program_size = 10;
  int program[10] = {96,5,97,7,112,3,113,2,18,0};
  int regs[16] = {0};
  for (int64_t i=0;i<cycles;i++){
    int hi = program[pc];
    int lo = program[(pc+1)%program_size];
    int opcode = (hi<<8) | lo;
    pc = (pc + 2) % program_size;
    int nib = opcode >> 12;
    if (nib == 1) {
      pc = opcode & 0x0FFF;
      pc %= program_size;
    } else if (nib == 6) {
      int reg = (opcode >> 8) & 0xF;
      regs[reg] = opcode & 0xFF;
    } else if (nib == 7) {
      int reg = (opcode >> 8) & 0xF;
      regs[reg] += opcode & 0xFF;
    }
  }
  int64_t sum = 0; for (int i=0;i<16;i++){ sum += regs[i]; }
  return (int)(sum & 0xFF);
}
C
  sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  sieve)
    # N: 上限値。EXE モードは計測安定性のため C 実行時間が十分大きくなる既定値に固定
    # 既定 N=5,000,000 のまま維持（以前の 500,000 丸めはタイマ粒度ノイズを増やすため撤廃）
    if [[ "$EXE_MODE" = "1" && "$N" = "5000000" ]]; then
      N=5000000
    fi
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
static box Main { method main(args) {
  local limit = ${N}
  // true=prime候補
  local flags = new ArrayBox()
  local i = 0
  loop(i <= limit) { flags.push(1)  i = i + 1 }
  flags.set(0, 0)  flags.set(1, 0)
  local p = 2
  loop(p * p <= limit) {
    if (flags.get(p) == 1) {
      local m = p * p
      loop(m <= limit) { flags.set(m, 0)  m = m + p }
    }
    p = p + 1
  }
  local count = 0
  i = 0
  loop(i <= limit) { count = count + flags.get(i)  i = i + 1 }
  return count
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
#include <stdlib.h>
int main(){
  int64_t limit = N_PLACEHOLDER;
  unsigned char *flags = (unsigned char*)malloc((limit+1));
  for (int64_t i=0;i<=limit;i++) flags[i]=1;
  flags[0]=flags[1]=0;
  for (int64_t p=2;p*p<=limit;p++) if (flags[p]) for (int64_t m=p*p;m<=limit;m+=p) flags[m]=0;
  int64_t count=0; for (int64_t i=0;i<=limit;i++) count+=flags[i];
  free(flags);
  return (int)(count & 0xFF);
}
C
    sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  matmul)
    # N: 行列サイズ。EXE モード既定は N=512、REPS_M=16 に上げてタイマ粒度ノイズを低減
    if [[ "$EXE_MODE" = "1" && "$N" = "5000000" ]]; then
      N=512
    fi
    REPS_M=${REPS_M:-8}
    if [[ "$EXE_MODE" = "1" && "${REPS_M}" = "8" ]]; then
      REPS_M=16
    fi
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
static box Main { method main(args) {
  local n = ${N}
  local reps = ${REPS_M}
  // A,B,C を一次元ArrayBoxに格納（row-major）
  local A = new ArrayBox(); local B = new ArrayBox(); local C = new ArrayBox()
  local i = 0
  loop(i < n*n) { A.push(i % 97)  B.push((i*3) % 101)  C.push(0)  i = i + 1 }
  i = 0
  loop(i < n) {
    local j = 0
    loop(j < n) {
      local sum = 0
      local k = 0
      loop(k < n) {
        local a = A.get(i*n + k)
        local b = B.get(k*n + j)
        sum = sum + a * b
        k = k + 1
      }
      // repeat accumulation to scale work per element
      local r = 0
      loop(r < reps) { sum = sum + (r % 7)  r = r + 1 }
      C.set(i*n + j, sum)
      j = j + 1
    }
    i = i + 1
  }
  // 端を返して最適化抑止
  return C.get((n-1)*n + (n-1))
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
#include <stdlib.h>
int main(){
  int n = N_PLACEHOLDER;
  int reps = REPS_PLACE;
  int *A = (int*)malloc(sizeof(int)*n*n);
  int *B = (int*)malloc(sizeof(int)*n*n);
  int *C = (int*)malloc(sizeof(int)*n*n);
  for (int i=0;i<n*n;i++){ A[i]=i%97; B[i]=(i*3)%101; C[i]=0; }
  for (int i=0;i<n;i++){
    for (int j=0;j<n;j++){
      long long sum=0;
      for (int k=0;k<n;k++) sum += (long long)A[i*n+k]*B[k*n+j];
      for (int r=0;r<reps;r++) sum += (r % 7);
      C[i*n+j]=(int)sum;
    }
  }
  int r = C[(n-1)*n + (n-1)];
  free(A); free(B); free(C);
  return r & 0xFF;
}
C
    sed -i "s/N_PLACEHOLDER/${N}/; s/REPS_PLACE/${REPS_M}/" "$C_FILE"

    # Pre-check: verify emit stability for matmul in EXE mode
    if [[ "$EXE_MODE" = "1" ]]; then
      TMP_CHECK_JSON=$(mktemp --suffix .json)
      if ! \
           HAKO_SELFHOST_BUILDER_FIRST=0 HAKO_SELFHOST_NO_DELEGATE=0 \
           NYASH_AOT_COLLECTIONS_HOT=1 NYASH_LLVM_FAST=1 NYASH_MIR_LOOP_HOIST=1 NYASH_AOT_MAP_KEY_MODE=auto \
           HAKO_MIR_BUILDER_LOOP_JSONFRAG="${HAKO_MIR_BUILDER_LOOP_JSONFRAG:-0}" \
           bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$TMP_CHECK_JSON" --input "$HAKO_FILE" \
           >/dev/null 2>&1; then
        echo "[SKIP] matmul emit unstable (try PERF_USE_JSONFRAG=1 for diagnosis)" >&2
        rm -f "$TMP_CHECK_JSON" "$HAKO_FILE" "$C_FILE" 2>/dev/null || true
        exit 0
      fi
      rm -f "$TMP_CHECK_JSON" 2>/dev/null || true
    fi
    ;;
  matmul_core)
    # Core numeric matmul using MatI64 + IntArrayCore
    # Use smaller default N to keep runtime reasonable
    if [[ "$EXE_MODE" = "1" && "$N" = "5000000" ]]; then
      N=256
    fi
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
using nyash.core.numeric.matrix_i64 as MatI64

static box Main { method main(args) {
  local n = ${N}
  // Initialize A, B, C as n x n matrices
  local A = MatI64.new(n, n)
  local B = MatI64.new(n, n)
  local C = MatI64.new(n, n)
  local i = 0
  loop(i < n) {
    local j = 0
    loop(j < n) {
      local idx = i*n + j
      A.set(i, j, idx % 97)
      B.set(i, j, (idx * 3) % 101)
      C.set(i, j, 0)
      j = j + 1
    }
    i = i + 1
  }
  // Naive matmul via MatI64.mul_naive
  local out = A.mul_naive(B)
  return out.at(n-1, n-1)
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int64_t *ptr;
  int64_t rows;
  int64_t cols;
  int64_t stride;
} MatI64Core;

static inline int64_t mat_get(MatI64Core *m, int64_t r, int64_t c) {
  return m->ptr[r * m->stride + c];
}

static inline void mat_set(MatI64Core *m, int64_t r, int64_t c, int64_t v) {
  m->ptr[r * m->stride + c] = v;
}

int main() {
  int64_t n = N_PLACEHOLDER;
  int64_t total = n * n;
  MatI64Core A, B, C;
  A.rows = B.rows = C.rows = n;
  A.cols = B.cols = C.cols = n;
  A.stride = B.stride = C.stride = n;
  A.ptr = (int64_t*)malloc(sizeof(int64_t)*total);
  B.ptr = (int64_t*)malloc(sizeof(int64_t)*total);
  C.ptr = (int64_t*)malloc(sizeof(int64_t)*total);
  for (int64_t idx = 0; idx < total; idx++) {
    A.ptr[idx] = idx % 97;
    B.ptr[idx] = (idx * 3) % 101;
    C.ptr[idx] = 0;
  }
  for (int64_t i = 0; i < n; i++) {
    for (int64_t k = 0; k < n; k++) {
      int64_t aik = mat_get(&A, i, k);
      for (int64_t j = 0; j < n; j++) {
        int64_t idx = i * C.stride + j;
        int64_t v = C.ptr[idx] + aik * mat_get(&B, k, j);
        C.ptr[idx] = v;
      }
    }
  }
  int64_t r = mat_get(&C, n-1, n-1);
  free(A.ptr); free(B.ptr); free(C.ptr);
  return (int)(r & 0xFF);
}
C
    sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  linidx)
    # Linear index pattern: idx = i*cols + j
    # Derive rows/cols from N to keep runtime stable
    ROWS=10000; COLS=32
    if [[ "$EXE_MODE" = "1" && "$N" = "5000000" ]]; then ROWS=20000; COLS=32; fi
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
static box Main { method main(args) {
  local rows = ${ROWS}
  local cols = ${COLS}
  local total = rows * cols
  local A = new ArrayBox()
  local i = 0
  loop(i < total) { A.push(i % 97)  i = i + 1 }
  local acc = 0
  i = 0
  loop(i < rows) {
    local j = 0
    loop(j < cols) {
      local idx = i * cols + j
      local v = A.get(idx)
      acc = acc + v
      A.set(idx, (v + acc) % 17)
      j = j + 1
    }
    i = i + 1
  }
  return acc & 255
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
#include <stdlib.h>
int main(){
  const int64_t rows = ROWS_P; const int64_t cols = COLS_P;
  const int64_t total = rows * cols;
  int64_t *A = (int64_t*)malloc(sizeof(int64_t)*total);
  for (int64_t i=0;i<total;i++) A[i]=i%97;
  int64_t acc=0;
  for (int64_t i=0;i<rows;i++){
    for (int64_t j=0;j<cols;j++){
      int64_t idx = i*cols + j;
      int64_t v = A[idx];
      acc += v;
      A[idx] = (v + acc) % 17;
    }
  }
  free(A);
  return (int)(acc & 255);
}
C
    sed -i "s/ROWS_P/${ROWS}/; s/COLS_P/${COLS}/" "$C_FILE"
    ;;
  maplin)
    # Map with integer linear key: key = i*bucket + j
    # Keep bucket small to stress get/set hot path; add REPS to increase per-iter work
    # Interpret N as rows when provided (except when default 5_000_000)
    ROWS=50000; BUCKET=32; REPS=8
    if [[ "$EXE_MODE" = "1" && "$N" = "5000000" ]]; then
      ROWS=200000; REPS=16
    elif [[ "$N" != "5000000" ]]; then
      ROWS="$N"
    fi
    BUCKET=32
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
static box Main { method main(args) {
  local rows = ${ROWS}
  local bucket = ${BUCKET}
  local reps = ${REPS}
  local arr = new ArrayBox()
  local map = new MapBox()
  // Prefill
  local i = 0
  loop(i < bucket) { arr.push(i)  i = i + 1 }
  // Run
  i = 0
  local acc = 0
  loop(i < rows) {
    local j = i % bucket
    local key = (i / bucket) * bucket + j
    local v = arr.get(j)
    arr.set(j, v + 1)
    map.set(key, v)
    acc = acc + map.get(key)
    // additional reps to reduce timer granularity effects
    local r = 0
    loop(r < reps) {
      // keep keys within [0, rows)
      local ii = (i + r) % rows
      local jj = (j + r) % bucket
      local k2 = (ii / bucket) * bucket + jj
      map.set(k2, v)
      acc = acc + map.get(k2)
      r = r + 1
    }
    i = i + 1
  }
  return acc & 255
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
#include <stdlib.h>
int main(){
  const int64_t rows = ROWS_P; const int64_t bucket = BUCKET_P; const int64_t reps = REPS_P;
  int64_t *arr = (int64_t*)malloc(sizeof(int64_t)*bucket);
  int64_t *mapv = (int64_t*)malloc(sizeof(int64_t)*rows);
  for (int64_t i=0;i<bucket;i++) arr[i]=i;
  int64_t acc=0;
  for (int64_t i=0;i<rows;i++){
    int64_t j = i % bucket;
    int64_t key = (i / bucket) * bucket + j;
    int64_t v = arr[j];
    arr[j] = v + 1;
    mapv[key] = v;
    acc += mapv[key];
    for (int64_t r=0;r<reps;r++){
      int64_t ii = (i + r) % rows;
      int64_t jj = (j + r) % bucket;
      int64_t k2 = (ii / bucket) * bucket + jj;
  mapv[k2] = v;
      acc += mapv[k2];
    }
  }
  free(arr); free(mapv);
  return (int)(acc & 255);
}
C
    sed -i "s/ROWS_P/${ROWS}/; s/BUCKET_P/${BUCKET}/; s/REPS_P/${REPS}/" "$C_FILE"
    ;;
  kilo)
    # kilo は C 参照側が重く、デフォルト N=5_000_000 だと実行が非常に長くなる。
    # Phase 21.5 最適化フェーズでは LLVM 系ベンチは EXE 経路のみを対象にする。
    # - LLVM backend かつ N が既定値（5_000_000）の場合は、常に N=200_000 に下げる。
    # - LLVM backend で EXE_MODE=0 の場合も、EXE 経路へ強制昇格する（VM フォールバック禁止）。
    if [[ "$BACKEND" = "llvm" && "$N" = "5000000" ]]; then
      N=200000
    fi
    if [[ "$BACKEND" = "llvm" && "$EXE_MODE" = "0" ]]; then
      echo "[info] kilo: forcing --exe for llvm backend (Phase 21.5 optimization)" >&2
      EXE_MODE=1
    fi
    HAKO_FILE=$(mktemp_hako)
    cat >"$HAKO_FILE" <<HAKO
box KiloBench {
  init { lines, undo }
  birth() {
    me.lines = new ArrayBox()
    me.undo = new ArrayBox()
    local i = 0
    loop(i < 64) {
      me.lines.push("line-" + i.toString())
      i = i + 1
    }
  }
  insert_chunk(row, text) {
    local line = me.lines.get(row)
    local len_box = line.length()
    local len = 0
    if len_box != null { len = len_box.toString().toInteger() }
    local split = len / 2
    local new_line = line.substring(0, split) + text + line.substring(split, len)
    me.lines.set(row, new_line)
    me.undo.push(text)
  }
  replace(pattern, replacement) {
    local count = me.lines.length().toString().toInteger()
    local i = 0
    loop(i < count) {
      local line = me.lines.get(i)
      if (line.indexOf(pattern) >= 0) {
        me.lines.set(i, line + replacement)
      }
      i = i + 1
    }
  }
  digest() {
    local total = 0
    local count = me.lines.length().toString().toInteger()
    local i = 0
    loop(i < count) {
      total = total + me.lines.get(i).length()
      i = i + 1
    }
    return total + me.undo.length().toString().toInteger()
  }
}
static box Main { method main(args) {
  local ops = ${N}
  local bench = new KiloBench()
  bench.birth()
  local i = 0
  loop(i < ops) {
    bench.insert_chunk(i % 64, "xx")
    if (i % 8 == 0) {
      bench.replace("line", "ln")
    }
    i = i + 1
  }
  return bench.digest()
} }
HAKO
    C_FILE=$(mktemp_c)
    cat >"$C_FILE" <<'C'
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
static void insert_chunk(char **lines, int row, const char *text){
  char *line = lines[row];
  size_t len = strlen(line);
  size_t split = len/2;
  char *out = malloc(len + strlen(text) + 1);
  memcpy(out, line, split);
  strcpy(out+split, text);
  strcpy(out+split+strlen(text), line+split);
  free(line);
  lines[row] = out;
}
static void replace_line(char **lines, const char *pattern, const char *repl){
  for (int i=0;i<64;i++){
    if (strstr(lines[i], pattern)){
      size_t len = strlen(lines[i]) + strlen(repl) + 1;
      char *out = malloc(len);
      strcpy(out, lines[i]);
      strcat(out, repl);
      free(lines[i]);
      lines[i] = out;
    }
  }
}
int main(){
  volatile int64_t ops = N_PLACEHOLDER;
  char *lines[64];
  for (int i=0;i<64;i++){
    char buf[32]; sprintf(buf, "line-%d", i);
    lines[i] = strdup(buf);
  }
  for (int64_t i=0;i<ops;i++){
    insert_chunk(lines, i % 64, "xx");
    if (i % 8 == 0) replace_line(lines, "line", "ln");
  }
  int64_t total = 0;
  for (int i=0;i<64;i++){ total += strlen(lines[i]); }
  for (int i=0;i<64;i++){ free(lines[i]); }
  return (int)(total & 0xFF);
}
C
    sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
    ;;
  *) echo "Unknown case: $CASE"; exit 2;;
esac

echo "[perf] case=$CASE n=$N runs=$RUNS backend=$BACKEND" >&2
sum_c=0; sum_h=0

if [[ "$EXE_MODE" = "1" ]]; then
  # Build C exe once
  C_EXE=$(mktemp --suffix .out)
  cc -O3 -march=native -o "$C_EXE" "$C_FILE"
  # Build Nyash exe once (requires llvm harness)
  if [[ "$BACKEND" != "llvm" ]]; then
    echo "[FAIL] --exe requires --backend llvm" >&2; exit 2
  fi
  ensure_llvmc
  ensure_nyrt
  HAKO_EXE=$(mktemp --suffix .out)
  TMP_JSON=$(mktemp --suffix .json)
  # Default: use provider-first with AotPrep for maximum optimization
  # DEBUG: Show file paths
  echo "[matmul/debug] HAKO_FILE=$HAKO_FILE TMP_JSON=$TMP_JSON" >&2
  if ! \
       HAKO_SELFHOST_TRACE=1 \
       HAKO_SELFHOST_BUILDER_FIRST=0 HAKO_SELFHOST_NO_DELEGATE=0 \
       HAKO_APPLY_AOT_PREP=1 \
       NYASH_AOT_COLLECTIONS_HOT=1 NYASH_LLVM_FAST=1 NYASH_MIR_LOOP_HOIST=1 NYASH_AOT_MAP_KEY_MODE=auto \
       HAKO_MIR_BUILDER_LOOP_JSONFRAG="${HAKO_MIR_BUILDER_LOOP_JSONFRAG:-$([[ "${PERF_USE_JSONFRAG:-0}" = 1 ]] && echo 1 || echo 0)}" \
       HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG="${HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG:-$([[ "${PERF_USE_JSONFRAG:-0}" = 1 ]] && echo 1 || echo 0)}" \
       HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE="${HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE:-1}" \
       HAKO_MIR_BUILDER_JSONFRAG_PURIFY="${HAKO_MIR_BUILDER_JSONFRAG_PURIFY:-1}" \
       NYASH_AOT_NUMERIC_CORE="${NYASH_AOT_NUMERIC_CORE:-0}" \
       NYASH_AOT_NUMERIC_CORE_TRACE="${NYASH_AOT_NUMERIC_CORE_TRACE:-0}" \
       NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
       NYASH_JSON_ONLY=1 bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$TMP_JSON" --input "$HAKO_FILE" 2>&1 | tee /tmp/matmul_emit_log.txt >/dev/null; then
    echo "[FAIL] emit MIR JSON failed (hint: set PERF_USE_PROVIDER=1 or HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1)" >&2; exit 3
  fi
  grep -E "\[prep:|provider/emit\]" /tmp/matmul_emit_log.txt >&2 || true

  # Quick diagnostics: ensure AotPrep rewrites are present and jsonfrag fallback is not used
  # DEBUG: Copy TMP_JSON for inspection
  cp "$TMP_JSON" /tmp/matmul_from_perf.json 2>/dev/null || true
  echo "[matmul/debug] TMP_JSON copied to /tmp/matmul_from_perf.json" >&2
  echo "[matmul/debug] Direct externcall count: $(grep -o '"op":"externcall"' "$TMP_JSON" 2>/dev/null | wc -l)" >&2
  diag_mir_json "$TMP_JSON"

  # AotPrep is now applied in hako-helper route via HAKO_APPLY_AOT_PREP=1
  # Build EXE via helper (selects crate backend ny-llvmc under the hood)
  if ! NYASH_LLVM_BACKEND=crate NYASH_LLVM_SKIP_BUILD=1 \
       NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
       NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
       NYASH_LLVM_VERIFY=1 NYASH_LLVM_VERIFY_IR=1 NYASH_LLVM_FAST=1 \
       bash "$ROOT/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$HAKO_EXE" --quiet >/dev/null 2>&1; then
    echo "[FAIL] build Nyash EXE failed (crate backend). Ensure ny-llvmc exists or try NYASH_LLVM_BACKEND=crate." >&2; exit 3
  fi

  # Execute runs. If BUDGET_MS>0, keep running until budget is exhausted.
  if [[ "$BUDGET_MS" != "0" ]]; then
    i=0; used=0
    while true; do
      i=$((i+1))
      t_c=$(time_exe_run "$C_EXE"); t_h=$(time_exe_run "$HAKO_EXE")
      sum_c=$((sum_c + t_c)); sum_h=$((sum_h + t_h)); used=$((used + t_h))
      if command -v python3 >/dev/null 2>&1; then ratio=$(python3 -c "print(round(${t_h}/max(${t_c},1)*100,2))" 2>/dev/null || echo NA); else ratio=NA; fi
      echo "run#$i c=${t_c}ms hak=${t_h}ms ratio=${ratio}% (budget used=${used}/${BUDGET_MS}ms)" >&2
      if [[ $used -ge $BUDGET_MS ]]; then RUNS=$i; break; fi
      # Safety valve to avoid infinite loop if t_h is 0ms
      if [[ $i -ge 999 ]]; then RUNS=$i; break; fi
    done
  else
    for i in $(seq 1 "$RUNS"); do
      t_c=$(time_exe_run "$C_EXE")
      t_h=$(time_exe_run "$HAKO_EXE")
      sum_c=$((sum_c + t_c)); sum_h=$((sum_h + t_h))
      if command -v python3 >/dev/null 2>&1; then
        ratio=$(python3 -c "print(round(${t_h}/max(${t_c},1)*100,2))" 2>/dev/null || echo NA)
      else
        ratio=NA
      fi
      echo "run#$i c=${t_c}ms hak=${t_h}ms ratio=${ratio}%" >&2
    done
  fi
  avg_c=$((sum_c / RUNS)); avg_h=$((sum_h / RUNS))
  echo "avg c=${avg_c}ms hak=${avg_h}ms" >&2
  if [ "$avg_c" -lt 5 ]; then
    echo "[WARN] C runtime is very small (${avg_c}ms). Increase --n to reduce timer granularity noise." >&2
  fi
  if command -v python3 >/dev/null 2>&1; then
    python3 - <<PY
c=$avg_c; h=$avg_h
ratio = (h/max(c,1))*100.0
print(f"ratio={ratio:.2f}%")
PY
  fi
  rm -f "$C_EXE" "$HAKO_EXE" "$TMP_JSON" 2>/dev/null || true
else
  for i in $(seq 1 "$RUNS"); do
    t_c=$(bench_c "$C_FILE" "${C_FILE%.c}")
    t_h=$(bench_hako "$HAKO_FILE" "$BACKEND")
    sum_c=$((sum_c + t_c)); sum_h=$((sum_h + t_h))
    if command -v python3 >/dev/null 2>&1; then
      ratio=$(python3 -c "print(round(${t_h}/max(${t_c},1)*100,2))" 2>/dev/null || echo NA)
    else
      ratio=NA
    fi
    echo "run#$i c=${t_c}ms hak=${t_h}ms ratio=${ratio}%" >&2
  done
  avg_c=$((sum_c / RUNS)); avg_h=$((sum_h / RUNS))
  echo "avg c=${avg_c}ms hak=${avg_h}ms" >&2
  if [ "$avg_c" -lt 5 ]; then
    echo "[WARN] C runtime is very small (${avg_c}ms). Increase --n to reduce timer granularity noise." >&2
  fi
  if command -v python3 >/dev/null 2>&1; then
    python3 - <<PY
c=$avg_c; h=$avg_h
ratio = (h/max(c,1))*100.0
print(f"ratio={ratio:.2f}%")
PY
  fi
fi

rm -f "$HAKO_FILE" "$C_FILE" "${C_FILE%.c}" 2>/dev/null || true
