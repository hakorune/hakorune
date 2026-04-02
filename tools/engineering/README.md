# Engineering Tools

Purpose:
- hold engineering-only helper scripts outside the top-level `tools/` narrative
- keep product-facing and bootstrap-critical entrypoints separate from dev-only tooling

Rules:
- actual engineering helpers live here
- top-level compatibility shims were drained and removed in `phase-31x / 31xE1`
- reintroduce a top-level alias only when a concrete external contract requires it

Current residents:
- `run_vm_stats.sh`
- `parity.sh`
