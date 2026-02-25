GC v0 — Mark & Sweep (Skeleton)
===============================

Responsibility
- Provide a minimal stop‑the‑world Mark & Sweep collector for Hakorune VM.
- Deterministic, observable, and Fail‑Fast by default.

Status
- Skeleton only. Not wired to VM yet. Safe to keep in repo without side‑effects.

Principles
- No generational/incremental logic in v0.
- Safepoints: call boundaries / loop back‑edges / before long I/O waits.
- Triggers: live_bytes growth (>80% since last sweep) or +4MB.
- Observability: `HAKO_GC_TRACE=1` for timings and survivor counts.

