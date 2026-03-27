#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  tools/perf/trace_optimization_bundle.sh --input <bench-key|path.hako> [options]
  tools/perf/trace_optimization_bundle.sh --mir-json <path> [options]

Options:
  --input <bench-key|path.hako>   Source input. Non-file values resolve to benchmarks/bench_<key>.hako
  --mir-json <path>               Existing MIR(JSON) input. Skips source emit
  --route <direct|hako-mainline|hako-helper>
                                  Source emit route when --input is used (default: direct)
  --function <name>               Target function for MIR summaries (default: main/Main.main/0 fallback)
  --callee-substr <text>          Restrict MIR window report to callees containing this text
  --lookahead <N>                 MIR window lookahead size (default: 8)
  --microasm-runs <N>             Optional perf-record runs on the same built exe (default: 0)
  --symbol <name>                 Optional symbol filter for symbol/perf annotate notes
  --skip-indexof-line-seed        Diagnostic only: bypass the dedicated indexOf("line") seed
  --timeout-secs <N>              Source emit timeout when --input is used (default: 60)
  --out-dir <dir>                 Bundle output directory

Examples:
  tools/perf/trace_optimization_bundle.sh \
    --input kilo_micro_array_getset \
    --route direct \
    --callee-substr RuntimeDataBox.get

  tools/perf/trace_optimization_bundle.sh \
    --mir-json target/kilo_micro_array_getset.mir.json \
    --callee-substr RuntimeDataBox.get
USAGE
}

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
EMIT_ROUTE="${ROOT_DIR}/tools/smokes/v2/lib/emit_mir_route.sh"
NY_MIR_BUILDER="${ROOT_DIR}/tools/ny_mir_builder.sh"
HAKORUNE_BIN="${ROOT_DIR}/target/release/hakorune"

INPUT_MODE=""
INPUT_RAW=""
ROUTE="direct"
FUNCTION=""
CALLEE_SUBSTR=""
LOOKAHEAD=8
MICROASM_RUNS=0
SYMBOL=""
SKIP_INDEXOF_LINE_SEED=0
OUT_DIR=""
TIMEOUT_SECS=60

while [[ $# -gt 0 ]]; do
  case "$1" in
    --input)
      INPUT_MODE="source"
      INPUT_RAW="${2:-}"
      shift 2
      ;;
    --mir-json)
      INPUT_MODE="mir"
      INPUT_RAW="${2:-}"
      shift 2
      ;;
    --route)
      ROUTE="${2:-}"
      shift 2
      ;;
    --function)
      FUNCTION="${2:-}"
      shift 2
      ;;
    --callee-substr)
      CALLEE_SUBSTR="${2:-}"
      shift 2
      ;;
    --lookahead)
      LOOKAHEAD="${2:-}"
      shift 2
      ;;
    --microasm-runs)
      MICROASM_RUNS="${2:-}"
      shift 2
      ;;
    --symbol)
      SYMBOL="${2:-}"
      shift 2
      ;;
    --skip-indexof-line-seed)
      SKIP_INDEXOF_LINE_SEED=1
      shift
      ;;
    --timeout-secs)
      TIMEOUT_SECS="${2:-}"
      shift 2
      ;;
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[error] unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "${INPUT_MODE}" || -z "${INPUT_RAW}" ]]; then
  usage >&2
  exit 2
fi

if ! [[ "${LOOKAHEAD}" =~ ^[0-9]+$ ]] || [[ "${LOOKAHEAD}" -lt 1 ]]; then
  echo "[error] --lookahead must be >= 1: ${LOOKAHEAD}" >&2
  exit 2
fi
if ! [[ "${MICROASM_RUNS}" =~ ^[0-9]+$ ]]; then
  echo "[error] --microasm-runs must be >= 0: ${MICROASM_RUNS}" >&2
  exit 2
fi
if ! [[ "${TIMEOUT_SECS}" =~ ^[0-9]+$ ]] || [[ "${TIMEOUT_SECS}" -lt 0 ]]; then
  echo "[error] --timeout-secs must be >= 0: ${TIMEOUT_SECS}" >&2
  exit 2
fi

INPUT_PATH=""
INPUT_LABEL=""
if [[ "${INPUT_MODE}" == "source" ]]; then
  if [[ -f "${INPUT_RAW}" ]]; then
    INPUT_PATH="${INPUT_RAW}"
    INPUT_LABEL="$(basename "${INPUT_RAW}")"
  else
    INPUT_PATH="${ROOT_DIR}/benchmarks/bench_${INPUT_RAW}.hako"
    INPUT_LABEL="${INPUT_RAW}"
  fi
  if [[ ! -f "${INPUT_PATH}" ]]; then
    echo "[error] source input not found: ${INPUT_PATH}" >&2
    exit 2
  fi
else
  INPUT_PATH="${INPUT_RAW}"
  INPUT_LABEL="$(basename "${INPUT_RAW}")"
  if [[ ! -f "${INPUT_PATH}" ]]; then
    echo "[error] mir-json input not found: ${INPUT_PATH}" >&2
    exit 2
  fi
fi

label_safe() {
  printf '%s' "$1" | tr '/ ' '__' | tr -cd 'A-Za-z0-9._-'
}

if [[ -z "${OUT_DIR}" ]]; then
  ts="$(date +%Y%m%d-%H%M%S)"
  OUT_DIR="${ROOT_DIR}/target/perf_state/optimization_bundle/${ts}-$(label_safe "${INPUT_LABEL}")"
fi
mkdir -p "${OUT_DIR}"

MIR_JSON="${OUT_DIR}/bundle.mir.json"
BUILD_LOG="${OUT_DIR}/build.log"
BUILD_RC_FILE="${OUT_DIR}/build.rc"
ROUTE_TRACE_LOG="${OUT_DIR}/route_trace.log"
ROUTE_TRACE_SUMMARY="${OUT_DIR}/route_trace_summary.txt"
RECIPE_ACCEPTANCE_OUT="${OUT_DIR}/recipe_acceptance.txt"
MIR_HOTOPS="${OUT_DIR}/mir_hotops.txt"
MIR_WINDOWS="${OUT_DIR}/mir_windows.txt"
LL_DUMP="${OUT_DIR}/lowered.ll"
HOT_BLOCK_RESIDUE_OUT="${OUT_DIR}/hot_block_residue.txt"
EXE_OUT="${OUT_DIR}/bundle.exe"
SYMBOLS_OUT="${OUT_DIR}/symbols.txt"
SYMBOL_MATCH_OUT="${OUT_DIR}/symbol_match.txt"
MANIFEST_OUT="${OUT_DIR}/bundle_manifest.txt"
PERF_TOP_OUT="${OUT_DIR}/perf_top.txt"
PERF_DATA="${OUT_DIR}/bundle.perf.data"
PERF_ANNOTATE_OUT="${OUT_DIR}/perf_annotate.txt"
OBJDUMP_OUT="${OUT_DIR}/objdump.txt"
RUNNER_C="${OUT_DIR}/microasm_runner.c"
RUNNER_BIN="${OUT_DIR}/microasm_runner.bin"

printf 'input_mode=%s\ninput_path=%s\ninput_label=%s\nroute=%s\nfunction=%s\ncallee_substr=%s\nlookahead=%s\nmicroasm_runs=%s\nsymbol=%s\n' \
  "${INPUT_MODE}" \
  "${INPUT_PATH}" \
  "${INPUT_LABEL}" \
  "${ROUTE}" \
  "${FUNCTION:-<auto>}" \
  "${CALLEE_SUBSTR:-<none>}" \
  "${LOOKAHEAD}" \
  "${MICROASM_RUNS}" \
  "${SYMBOL:-<none>}" > "${MANIFEST_OUT}"
printf 'skip_indexof_line_seed=%s\n' "${SKIP_INDEXOF_LINE_SEED}" >> "${MANIFEST_OUT}"

if [[ "${INPUT_MODE}" == "source" ]]; then
  cp "${INPUT_PATH}" "${OUT_DIR}/input.hako"
  if [[ ! -x "${EMIT_ROUTE}" ]]; then
    echo "[error] emit route helper missing: ${EMIT_ROUTE}" >&2
    exit 2
  fi
  set +e
  "${EMIT_ROUTE}" \
    --route "${ROUTE}" \
    --out "${MIR_JSON}" \
    --input "${INPUT_PATH}" \
    --timeout-secs "${TIMEOUT_SECS}" \
    >"${OUT_DIR}/emit.stdout.log" 2>"${OUT_DIR}/emit.stderr.log"
  emit_rc=$?
  set -e
  printf '%s\n' "${emit_rc}" > "${OUT_DIR}/emit.rc"
  if [[ "${emit_rc}" -ne 0 ]]; then
    echo "[error] source emit failed: rc=${emit_rc} out_dir=${OUT_DIR}" >&2
    exit 1
  fi
else
  cp "${INPUT_PATH}" "${MIR_JSON}"
fi

python3 - "${MIR_JSON}" "${INPUT_LABEL}" "${FUNCTION}" "${CALLEE_SUBSTR}" "${LOOKAHEAD}" "${MIR_HOTOPS}" "${MIR_WINDOWS}" <<'PY'
import json
import sys
from collections import Counter

mir_path, input_label, wanted_fn, callee_substr, lookahead_s, hotops_out, windows_out = sys.argv[1:8]
lookahead = int(lookahead_s)

with open(mir_path, "r", encoding="utf-8") as f:
    mod = json.load(f)

funcs = mod.get("functions") or []
if not funcs:
    raise SystemExit("no functions in MIR JSON")

target = None
if wanted_fn:
    for fn in funcs:
        if fn.get("name") == wanted_fn:
            target = fn
            break
    if target is None:
        raise SystemExit(f"function not found: {wanted_fn}")
else:
    for cand in ("main", "Main.main/0", "Main.main"):
        for fn in funcs:
            if fn.get("name") == cand:
                target = fn
                break
        if target is not None:
            break
    if target is None:
        target = funcs[0]

blocks = target.get("blocks") or []
ops = Counter()
mir_calls = Counter()
inst_total = 0

def callee_key(inst):
    mc = inst.get("mir_call") or {}
    callee = mc.get("callee") or {}
    ctype = callee.get("type") or "<unknown>"
    if ctype == "Method":
        bname = callee.get("box_name") or callee.get("box_type") or "<box>"
        mname = callee.get("method") or callee.get("name") or "<method>"
        return f"Method:{bname}.{mname}"
    if ctype == "Constructor":
        bname = callee.get("box_name") or callee.get("box_type") or "<box>"
        return f"Constructor:{bname}"
    if ctype == "Global":
        gname = callee.get("name") or "<global>"
        return f"Global:{gname}"
    if ctype == "Extern":
        ename = callee.get("name") or "<extern>"
        return f"Extern:{ename}"
    return ctype

for bb in blocks:
    insts = bb.get("instructions") or []
    for inst in insts:
        op = inst.get("op", "<unknown>")
        ops[op] += 1
        inst_total += 1
        if op == "mir_call":
            mir_calls[callee_key(inst)] += 1

with open(hotops_out, "w", encoding="utf-8") as out:
    out.write(
        f"[mir-shape] input={input_label} function={target.get('name')} "
        f"blocks={len(blocks)} inst_total={inst_total} unique_ops={len(ops)} "
        f"mir_calls={sum(mir_calls.values())} unique_mir_callees={len(mir_calls)}\n"
    )
    for idx, (op, cnt) in enumerate(sorted(ops.items(), key=lambda x: (-x[1], x[0]))[:12], start=1):
        pct = (cnt * 100.0 / inst_total) if inst_total else 0.0
        out.write(f"[mir-shape/op] rank={idx} op={op} count={cnt} pct={pct:.1f}\n")
    for idx, (callee, cnt) in enumerate(sorted(mir_calls.items(), key=lambda x: (-x[1], x[0]))[:12], start=1):
        pct = (cnt * 100.0 / sum(mir_calls.values())) if mir_calls else 0.0
        out.write(f"[mir-shape/call] rank={idx} callee={callee} count={cnt} pct={pct:.1f}\n")

with open(windows_out, "w", encoding="utf-8") as out:
    for bb in blocks:
        bbid = bb.get("id", "?")
        insts = bb.get("instructions") or []
        for ii, inst in enumerate(insts):
            if inst.get("op") != "mir_call":
                continue
            callee = callee_key(inst)
            if callee_substr and callee_substr not in callee:
                continue
            next_ops = []
            for off in range(1, lookahead + 1):
                if ii + off < len(insts):
                    next_ops.append(insts[ii + off].get("op", "<unknown>"))
                else:
                    next_ops.append("-")
            mc = inst.get("mir_call") or {}
            cal = mc.get("callee") or {}
            recv = cal.get("receiver", 0)
            dst = inst.get("dst", 0)
            args = mc.get("args") or []
            out.write(
                f"[mir-window] fn={target.get('name')} bb={bbid} ii={ii} callee={callee} "
                f"dst={dst} recv={recv} args={args} next={next_ops}\n"
            )
PY

set +e
env \
  NYASH_LLVM_ROUTE_TRACE=1 \
  NYASH_LLVM_BACKEND=crate \
  NYASH_LLVM_USE_CAPI="${NYASH_LLVM_USE_CAPI:-1}" \
  HAKO_V1_EXTERN_PROVIDER_C_ABI="${HAKO_V1_EXTERN_PROVIDER_C_ABI:-1}" \
  HAKO_BACKEND_COMPILE_RECIPE="${HAKO_BACKEND_COMPILE_RECIPE:-pure-first}" \
  HAKO_BACKEND_COMPAT_REPLAY="${HAKO_BACKEND_COMPAT_REPLAY:-none}" \
  NYASH_LLVM_SKIP_INDEXOF_LINE_SEED="${SKIP_INDEXOF_LINE_SEED}" \
  NYASH_LLVM_DUMP_IR="${LL_DUMP}" \
  bash "${NY_MIR_BUILDER}" \
    --in "${MIR_JSON}" \
    --emit exe \
    -o "${EXE_OUT}" \
    --quiet >"${BUILD_LOG}" 2>&1
build_rc=$?
set -e
printf '%s\n' "${build_rc}" > "${BUILD_RC_FILE}"

grep '^\[llvm-route/' "${BUILD_LOG}" > "${ROUTE_TRACE_LOG}" || true

python3 - "${ROUTE_TRACE_LOG}" "${ROUTE_TRACE_SUMMARY}" <<'PY'
import re
import sys
from collections import Counter

trace_path, out_path = sys.argv[1:3]
stage = Counter()
reason = Counter()
tag = Counter()

with open(trace_path, "r", encoding="utf-8", errors="replace") as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        m = re.match(r'^\[(llvm-route/[^\]]+)\]\s+(.*)$', line)
        if not m:
            continue
        tag_name = m.group(1)
        tag[tag_name] += 1
        if tag_name == "llvm-route/trace":
          stage_m = re.search(r'stage=([^ ]+)', line)
          result_m = re.search(r'result=([^ ]+)', line)
          reason_m = re.search(r'reason=([^ ]+)', line)
          if stage_m:
              stage[(stage_m.group(1), result_m.group(1) if result_m else "?")] += 1
          if reason_m:
              reason[(stage_m.group(1) if stage_m else "?", reason_m.group(1))] += 1

with open(out_path, "w", encoding="utf-8") as out:
    for name, count in sorted(tag.items()):
        out.write(f"[route-trace/tag] name={name} count={count}\n")
    for (stage_name, result_name), count in sorted(stage.items()):
        out.write(f"[route-trace/stage] stage={stage_name} result={result_name} count={count}\n")
    for (stage_name, reason_name), count in sorted(reason.items()):
        out.write(f"[route-trace/reason] stage={stage_name} reason={reason_name} count={count}\n")
PY

python3 - "${ROUTE_TRACE_LOG}" "${RECIPE_ACCEPTANCE_OUT}" <<'PY'
import re
import sys

trace_path, out_path = sys.argv[1:3]
rows = []

with open(trace_path, "r", encoding="utf-8", errors="replace") as f:
    for line in f:
        line = line.strip()
        if not line.startswith("[llvm-route/trace]"):
            continue
        stage = re.search(r"stage=([^ ]+)", line)
        result = re.search(r"result=([^ ]+)", line)
        reason = re.search(r"reason=([^ ]+)", line)
        extra = re.search(r"extra=(.*)$", line)
        rows.append(
            (
                stage.group(1) if stage else "?",
                result.group(1) if result else "?",
                reason.group(1) if reason else "?",
                extra.group(1) if extra else "-",
            )
        )

with open(out_path, "w", encoding="utf-8") as out:
    if not rows:
        out.write("[recipe-acceptance] empty=yes\n")
    for stage_name, result_name, reason_name, extra_text in rows:
        out.write(
            f"[recipe-acceptance/trace] stage={stage_name} result={result_name} "
            f"reason={reason_name} extra={extra_text}\n"
        )
PY

python3 - "${LL_DUMP}" "${HOT_BLOCK_RESIDUE_OUT}" <<'PY'
import sys

ll_path, out_path = sys.argv[1:3]
markers = [
    ("slot_load_hi", "nyash.array.slot_load_hi"),
    ("generic_box_call", "generic_box_call"),
    ("hostbridge", "hostbridge"),
    ("runtime_data", "runtime_data."),
]

text = ""
try:
    with open(ll_path, "r", encoding="utf-8", errors="replace") as f:
        text = f.read()
except FileNotFoundError:
    pass

with open(out_path, "w", encoding="utf-8") as out:
    if not text:
        out.write("[hot-block-residue] llvm_ir_present=no\n")
    else:
        out.write("[hot-block-residue] llvm_ir_present=yes scan_scope=whole_ir\n")
        for label, needle in markers:
            out.write(f"[hot-block-residue/item] label={label} count={text.count(needle)} needle={needle}\n")
PY

if [[ -f "${EXE_OUT}" ]]; then
  if command -v nm >/dev/null 2>&1; then
    nm -C "${EXE_OUT}" > "${SYMBOLS_OUT}" || true
  elif command -v objdump >/dev/null 2>&1; then
    objdump -t --demangle "${EXE_OUT}" > "${SYMBOLS_OUT}" || true
  else
    printf '[symbol-proof] skipped=no_symbol_tool\n' > "${SYMBOLS_OUT}"
  fi
  if [[ -n "${SYMBOL}" && -f "${SYMBOLS_OUT}" ]]; then
    grep -F "${SYMBOL}" "${SYMBOLS_OUT}" > "${SYMBOL_MATCH_OUT}" || true
  fi
fi

if [[ "${MICROASM_RUNS}" -gt 0 && -f "${EXE_OUT}" && -x "${EXE_OUT}" ]]; then
  if command -v perf >/dev/null 2>&1 && command -v "${CC:-cc}" >/dev/null 2>&1; then
    cat >"${RUNNER_C}" <<'EOF'
#include <errno.h>
#include <spawn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

extern char **environ;

int main(int argc, char **argv) {
  if (argc != 3) {
    fprintf(stderr, "usage: %s <runs> <exe>\n", argv[0]);
    return 2;
  }
  char *end = NULL;
  long runs = strtol(argv[1], &end, 10);
  if (!end || *end != '\0' || runs < 1) {
    fprintf(stderr, "invalid runs: %s\n", argv[1]);
    return 2;
  }
  char *const child_argv[] = { argv[2], NULL };
  for (long i = 0; i < runs; ++i) {
    pid_t pid = 0;
    int rc = posix_spawn(&pid, argv[2], NULL, NULL, child_argv, environ);
    if (rc != 0) {
      fprintf(stderr, "posix_spawn failed: %s\n", strerror(rc));
      return 1;
    }
    int status = 0;
    if (waitpid(pid, &status, 0) < 0) {
      fprintf(stderr, "waitpid failed: %s\n", strerror(errno));
      return 1;
    }
    if (!WIFEXITED(status)) return 1;
  }
  return 0;
}
EOF
    "${CC:-cc}" -O2 -std=c11 -Wall -Wextra -o "${RUNNER_BIN}" "${RUNNER_C}"
    env \
      NYASH_GC_MODE="${NYASH_GC_MODE:-off}" \
      NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}" \
      NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
      NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}" \
      perf record -o "${PERF_DATA}" -F 999 -- "${RUNNER_BIN}" "${MICROASM_RUNS}" "${EXE_OUT}" >/dev/null 2>&1 || true
    if [[ -f "${PERF_DATA}" ]]; then
      perf report --stdio --no-children -i "${PERF_DATA}" > "${PERF_TOP_OUT}" || true
      if [[ -n "${SYMBOL}" ]]; then
        perf annotate --stdio -i "${PERF_DATA}" --symbol "${SYMBOL}" > "${PERF_ANNOTATE_OUT}" || true
      fi
      if command -v objdump >/dev/null 2>&1; then
        objdump -d --demangle "${EXE_OUT}" > "${OBJDUMP_OUT}" || true
      fi
    fi
  else
    printf '[microasm] skipped=perf_or_cc_missing\n' > "${PERF_TOP_OUT}"
  fi
fi

echo "[bundle] out_dir=${OUT_DIR}"
echo "[bundle] mir_json=${MIR_JSON}"
echo "[bundle] route_trace=${ROUTE_TRACE_LOG}"
echo "[bundle] route_summary=${ROUTE_TRACE_SUMMARY}"
echo "[bundle] recipe_acceptance=${RECIPE_ACCEPTANCE_OUT}"
echo "[bundle] mir_hotops=${MIR_HOTOPS}"
echo "[bundle] mir_windows=${MIR_WINDOWS}"
echo "[bundle] build_log=${BUILD_LOG} build_rc=${build_rc}"
if [[ -f "${LL_DUMP}" ]]; then
  echo "[bundle] llvm_ir=${LL_DUMP}"
  echo "[bundle] hot_block_residue=${HOT_BLOCK_RESIDUE_OUT}"
fi
if [[ -f "${SYMBOLS_OUT}" ]]; then
  echo "[bundle] symbols=${SYMBOLS_OUT}"
fi
if [[ "${MICROASM_RUNS}" -gt 0 && -f "${PERF_TOP_OUT}" ]]; then
  echo "[bundle] perf_top=${PERF_TOP_OUT}"
fi

if [[ "${build_rc}" -ne 0 ]]; then
  echo "[bundle] build failed; see ${BUILD_LOG}" >&2
  exit 1
fi
