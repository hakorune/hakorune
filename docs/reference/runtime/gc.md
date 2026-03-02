Nyash GC Modes — Design and Usage

Overview
- Nyash adopts a pragmatic GC strategy that balances safety, performance, and simplicity.
- Current operational contract is fixed by Phase 29y RC/GC alignment gates.
- Semantics invariance is pinned between `rc+cycle` and `off` (GC ON/OFF).

Operationally pinned modes (current SSOT)
- rc+cycle (default, safe)
  - Reference counting with periodic cycle detection/collection.
  - Beginner/safe profile for day-to-day operation.
- off (expert, self‑responsibility)
  - GC hooks are disabled; strong cycles may leak.
  - Expert profile used by ON/OFF invariance gates.

Selection & Precedence
- CLI: `--gc {auto,rc+cycle,off}` (auto = rc+cycle)
- ENV: `NYASH_GC_MODE` (overridden by CLI)
- nyash.toml [env] applies last

Instrumentation & Diagnostics
- `NYASH_GC_METRICS=1`: print brief metrics (allocs/bytes/cycles/pauses)
- Optional lane diag tag (metrics ON only): `[gc/optional:mode] mode=<...> collect_sp=<...> collect_alloc=<...>`

Operational Guidance
- Default: rc+cycle for stable operations.
- Validate semantics equivalence with `rc+cycle` and `off` via G-RC-5/G-RC-2 gates.

Implementation status and lane direction
1) Boundary lock (done)
   - `GcMode` is operationally fixed to `rc+cycle/off` (`auto` is alias to `rc+cycle`).
   - `GcController` remains the single GC entry for safepoints/metrics/coordination.
2) Optional lane min2 (done)
   - Pin optional-GC observability points in dev/diagnostic scope only (default OFF, stable one-line tags).
3) Optional lane min3 (next)
   - Introduce guarded pilot behavior only under optional mode and keep rollback-ready granularity.

Current status note
- The runtime currently guarantees operational contracts through `rc+cycle/off` invariance gates.
- Unsupported modes fail-fast (`NYASH_GC_MODE` must be `auto|rc+cycle|off`).

Notes
- Safepoint and barrier MIR ops already exist and are reused as GC coordination hooks.
- Handle indirection keeps future moving GCs compatible with plugin/FFI boundaries.

LLVM Safepoints
- Automatic safepoint insertion can be toggled for the LLVM harness/backend:
  - NYASH_LLVM_AUTO_SAFEPOINT=1 enables insertion (default 1)
  - Injection points: loop headers, function calls, externcalls, and selected boxcalls.
  - Safepoints call ny_check_safepoint/ny_safepoint in NyRT, which forwards to runtime hooks (GC.safepoint + scheduler poll).

Controller & Metrics
- The unified GcController implements GcHooks and aggregates metrics (safepoints/read/write/alloc).
- CountingGc is a thin wrapper around GcController for compatibility.
