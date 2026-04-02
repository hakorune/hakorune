# Engineering Tools

Purpose:
- hold engineering-only helper scripts outside the top-level `tools/` narrative
- keep product-facing and bootstrap-critical entrypoints separate from dev-only tooling

Rules:
- actual engineering helpers live here
- old top-level paths may stay as thin compatibility shims for a while
- delete shims only after current docs and live callers stop pointing at them

Current residents:
- `run_vm_stats.sh`
- `parity.sh`
