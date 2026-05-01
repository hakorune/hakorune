# PHI Trace Debug Probes

This directory owns manual PHI trace diagnostics.

Rules:

- These probes are explicit debug tools, not daily compiler proof routes.
- `phi_trace_run.sh` is the one-shot `.hako` trace runner.
- `phi_trace_bridge_try.sh` is the legacy bridge trace probe for explicit
  Program(JSON v0) investigations.
- `phi_trace_check.py` is the shared trace validator for both probes and local
  PHI trace smoke wrappers.
- Keep default compiler/backend behavior unchanged when editing this directory.

Entries:

- `tools/debug/phi/phi_trace_run.sh`
- `tools/debug/phi/phi_trace_bridge_try.sh`
- `tools/debug/phi/phi_trace_check.py`
