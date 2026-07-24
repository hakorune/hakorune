# RAW public cutover consultation

Status: closed by `RAW-PUBLIC-CUTOVER-prime-r1`.

Selected decision and fixed execution order:
`cut0-i0-raw-source0-lower-root-post0-public-cutover-decision-task-map-2026-07-24.md`.

Decision stop: `RAW-PUBLIC-CUTOVER-CONSULT0`

`PUBLICATION-ADAPTER0-S0` and `PUBLIC-INGRESS0-S0` are implemented and
focused-verified. `compile_raw_with_source` is now an explicit NarrowV1 entry;
`compile_with_source`, JSON bridges, the executor, and old Raw bridges remain
unchanged. The next decision must choose how, or whether, the normal entry is
cut over and when the old chain is retired.

## Evidence inventory

```text
new explicit Raw ingress producer                 = 1
normal compile_with_source legacy route           = unchanged
old MirBuilder::build_module production route    = present
old Raw finalizer/run_raw sources                 = present
JSON/Program(JSON v0) routes                      = unchanged
Raw focused chain (single-thread)                 = 240 passed
adapter/ingress guards                            = green
```

## Questions

### Q1 — normal-entry authority

Choose one:

```text
A. keep compile_with_source legacy and require explicit Raw ingress
B. switch compile_with_source to Raw NarrowV1 for admitted grammar only
C. add a separate capability-selected normal entry before any switch
```

The current recommendation is A until parity and caller census are complete.

### Q2 — admitted-grammar parity

Which evidence is sufficient before a switch?

```text
A. bounded success/failure parity for the locked Raw grammar and compiler reuse
B. full library-suite parity including unsupported legacy shapes
C. runtime-only parity without MIR/verification relation checks
```

Unsupported Raw shapes must remain typed rejection; they must not fallback.

### Q3 — failure and Builder reuse

Choose the required gate:

```text
A. every Raw failure leaves live Builder unchanged; repeated compiler reuse is tested
B. compare only public error strings
C. allow legacy retry for failures outside the first slice
```

### Q4 — old-chain retirement

Choose the retirement trigger:

```text
A. old Raw finalizer/run_raw/ledger-root evidence non-test callers = 0,
   then delete in a dedicated row
B. delete old sources immediately after explicit ingress lands
C. keep both chains permanently
```

### Q5 — JSON and production activation

Choose the boundary:

```text
A. JSON routes stay unchanged; executor/CUT0 activation is a later row
B. route JSON through compile_raw_with_source now
C. switch executor first and infer route from module symbols
```

### Q6 — sunset measurement

Choose the census scope:

```text
A. explicit production caller census by route plus RAW-PUBLICATION-SUNSET-001
B. repository-wide lexical zero for every old symbol
C. focused test caller census only
```

No implementation should begin until Q1-Q6 and the normal-entry stop line are
recorded. This consultation deliberately claims no normal-entry cutover,
legacy deletion, executor wiring, JSON change, or CUT0 activation.

## Census update (2026-07-24)

The current production-facing source callers are still:

```text
compile_with_source / compile_with_source_and_imports:
  src/mir/compiler/mod.rs
  src/runner/modes/common_util/source_hint.rs
  src/backend/mir_interpreter/strict_json_session.rs

direct build_module compatibility bridges:
  src/runtime/mirbuilder_emit.rs
  src/host_providers/mir_builder/lowering/ast_json.rs (cfg(test)-only)

old Raw run_raw:
  source definitions remain in module_postprocess/raw_physical_finalization;
  observed run_raw fixtures are cfg(test)-scoped

new Raw production entry:
  src/mir/compiler/raw_public_ingress.rs::compile_raw_with_source definition = 1
  non-test caller = 0
```

This census confirms that a normal-entry switch would affect runner and
interpreter-facing callers, while JSON bridges are a separate authority. The
selected decision keeps the normal entry Legacy and opens the bounded repair,
configuration, coverage, parity, and retirement rows instead.
