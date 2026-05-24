#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-hakmem-external-corpus-catalog"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-87-MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-CORPUS-CATALOG.md"
CATALOG="docs/development/current/main/phases/phase-295x/295x-hakmem-external-results-catalog-v0.toml"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-86-MIMALLOC-COMPARISON-HAKOZUNA-COMPARE-LOG-ADAPTER.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_hakmem_external_corpus_catalog_guard.sh"
BENCHRES_ADAPTER="tools/allocator/hakmem_benchres_adapter.py"
LOG_ADAPTER="tools/allocator/hakmem_hakozuna_compare_log_adapter.py"

echo "[$TAG] checking phase-295x hakmem external corpus catalog"

guard_require_files "$TAG" "$CARD" "$CATALOG" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$BENCHRES_ADAPTER" "$LOG_ADAPTER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$BENCHRES_ADAPTER" "$LOG_ADAPTER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-CORPUS-CATALOG-295X-001' "$CARD" "card must identify blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001' "$CARD" "card must select malloc-large contract"
guard_expect_in_file "$TAG" 'winner_claim = 0' "$CATALOG" "catalog must keep winner claims closed"
guard_expect_in_file "$TAG" 'provider_activation = 0' "$CATALOG" "catalog must keep provider activation closed"
guard_expect_in_file "$TAG" 'selected_next_workload_family = "mimalloc-bench-malloc-large"' "$CATALOG" "catalog must select malloc-large"
guard_expect_in_file "$TAG" 'bench_results/mimalloc_bench_full_20260117_064211/benchres.csv' "$CATALOG" "catalog must include clean benchres candidate"
guard_expect_in_file "$TAG" 'bench_results/hakozuna_compare_20260118_034633' "$CATALOG" "catalog must include hakozuna compare candidate"
guard_expect_in_file "$TAG" 'bench_results/s51_malloc_large_b40795031_20260105_105541' "$CATALOG" "catalog must include malloc-large secondary evidence"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-CORPUS-CATALOG-295X-001' "$PREV_CARD" "previous card must select catalog"
guard_expect_in_file "$TAG" '295x-87' "$TASKBOARD" "taskboard must expose catalog row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

python3 - <<'PY'
from pathlib import Path

catalog = Path("docs/development/current/main/phases/phase-295x/295x-hakmem-external-results-catalog-v0.toml")
text = catalog.read_text(encoding="utf-8")
root_line = next(line for line in text.splitlines() if line.startswith("dataset_root = "))
root = Path(root_line.split("=", 1)[1].strip().strip('"'))
for rel in [
    "bench_results/mimalloc_bench_full_20260117_064211/benchres.csv",
    "bench_results/mimalloc_bench_full_20260117_064626/benchres.csv",
    "bench_results/hakozuna_compare_20260118_034633",
    "bench_results/hakozuna_compare_20260118_035003",
    "bench_results/s51_malloc_large_b40795031_20260105_105541",
]:
    path = root / rel
    if not path.exists():
        raise SystemExit(f"missing catalog artifact: {path}")
print("hakmem_external_corpus_catalog_readable=1")
PY

echo "[$TAG] ok"
