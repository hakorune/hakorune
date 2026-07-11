# Claude Code Runbook (Phase 285): VM↔LLVM lifecycle conformance

This file is an instruction sheet for an implementation agent (Claude Code) to make the language SSOT pass end-to-end across backends.

Language SSOT:
- Lifecycle/weak/fini/GC policy: `docs/reference/language/lifecycle.md`
- Truthiness + `null`/`void`: `docs/reference/language/types.md`

Non-goal: changing language semantics. Any backend drift must be fixed as an implementation bug or explicitly tracked as “unsupported”.

## What to implement (in order)

### 0) Preflight (must-pass before any weak smokes)

Confirm the following are implemented; if any are missing, do **not** run weak fixtures yet:
- `weak <expr>` can be parsed and lowered into MIR (WeakRef/WeakNew).
- VM has a handler for MIR `WeakRef/WeakNew/WeakLoad` (no panic/unimplemented).
- `WeakRef.weak_to_strong()` exists at the language surface.

If any are missing, choose one:
- **Option A**: build the missing weak infrastructure first.
- **Option B**: temporarily scope to exit-time leak report only (skip weak smokes, document as “unsupported”).

### 1) WeakRef semantics (VM + LLVM)

Required behavior:
- `weak <expr>` creates a non-owning WeakRef.
- `w.weak_to_strong()` returns a strong BoxRef when the target is usable; otherwise returns `null` (runtime `Void`).
- WeakRef does not auto-upgrade on field access (field read returns WeakRef).
- WeakRef equality uses a stable token (do not make `dropped==dropped` true for unrelated targets).

Conformance checks:
- VM: weak works in real execution (not just `toString()`).
- LLVM (harness): must match VM behavior for the same program output/exit code.
- WASM: if unsupported, keep it explicitly documented as unsupported; do not pretend it is correct by copying strong refs.

### 2) Exit-time “roots still held” report (diagnostic, default-off)

Goal: when a program ends while strong references are still held in global roots, print a report so developers can see leaks/cycles.

Requirements:
- Must be default-off.
- Must not change program meaning (only prints when enabled).
- Should report “what roots still hold strong references”, not attempt to “fix” them.

Suggested interface (choose one and document it):
- Env: `NYASH_LEAK_LOG={1|2}`
  - `1`: summary counts
  - `2`: verbose (print up to N names/entries, with truncation)

Root candidates to include (best-effort):
- `env.modules` registry
- plugin singletons / plugin registry
- host handles / external handles registry

Output stability:
- Use stable tags like `[leak]` or `[lifecycle/leak]` so smokes can match logs.
- Truncate long lists deterministically (e.g., first N sorted entries).

### 3) Cross-backend smokes (VM + LLVM)

Add smokes under `tools/smokes/v2/` (preferred) to lock behavior.
Keep tests fast and deterministic.

Recommended fixtures (as `.hako` or inline sources in the smoke):

**A. Weak weak_to_strong success/fail**
```nyash
box SomeBox { x }
static box Main {
  main() {
    local x = new SomeBox()
    local w = weak x
    x = null
    local y = w.weak_to_strong()
    if y == null { print("ok: dropped") }
    return 0
  }
}
```
Expected (VM and LLVM): prints `ok: dropped`, exit 0.

**B. Strong cycle + leak report**
```nyash
box Node { other }
static box Main {
  main() {
    local a = new Node()
    local b = new Node()
    a.other = b
    b.other = a
    print("ok: cycle-created")
    return 0
  }
}
```
Expected:
- Program output stays `ok: cycle-created`.
- With leak report enabled, a report appears at exit (VM at minimum; LLVM if feasible).

**C. Weak breaks cycle (no strong-cycle leak)**
```nyash
box Node { other_weak }
static box Main {
  main() {
    local a = new Node()
    local b = new Node()
    a.other_weak = weak b
    b.other_weak = weak a
    print("ok: weak-cycle")
    return 0
  }
}
```
Expected:
- Program output stays `ok: weak-cycle`.
- Leak report should not claim an obvious strong-cycle root for these nodes (best-effort; depends on what is rooted globally).

### 4) Update docs after implementation

When the above is implemented:
- Add the chosen env var to `docs/reference/environment-variables.md` (avoid env var sprawl; keep it in the diagnostics table).
- If any backend remains unsupported, update `docs/reference/language/lifecycle.md` “Implementation status” with an explicit note and link.

## Commands (suggested)

Build:
- `cargo build --release --features llvm`

Run VM:
- `./target/release/hakorune --backend vm local_tests/phase285_weak_basic.hako`

Run LLVM:
- `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm local_tests/phase285_weak_basic.hako`

Leak report (example if env var chosen):
- `NYASH_LEAK_LOG=1 ./target/release/hakorune --backend vm local_tests/phase285_cycle.hako`

## Done criteria (acceptance)

- VM and LLVM outputs match for weak fixtures (success/fail).
- Strong-cycle fixture produces a visible exit-time report when the diagnostic is enabled (and produces no report when disabled).
- Weak-cycle fixture does not falsely report a strong-cycle “leak” for the nodes (within the documented root scope).
