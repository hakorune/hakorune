# RAW PUBLIC-INGRESS0 closeout repair execution task

Decision: `RAW-PUBLIC-CUTOVER-prime-r1`

Status: active execution.

This is a behavior-neutral evidence repair. It does not change Raw grammar,
configuration ownership, normal compilation, JSON, or any production caller.

## Scope

```text
CLOSEOUT-DOC0
  mark the landed PUBLIC-INGRESS0 task closed
  replace claims not yet backed by focused fixtures with exact evidence

CLOSEOUT-CENSUS0
  add one explicit cfg-test-aware route manifest
  Raw public API producer = 1
  Raw non-test caller = 0
  normal no-import/import caller families remain distinct
  direct build_module production callers = compiler + runtime AST-JSON only

CLOSEOUT-FIXTURE0
  repeated empty Raw success
  typed Raw failure followed by Raw success
  stable public stage prefix
  live Builder unchanged on failure

CLOSEOUT-G0
  make the landed ingress guard independent of the active-row pointer
  require landed state plus the exact producer/caller boundary
  keep every modified source/check file below 800 lines
```

## Structural placement

The route census belongs in an explicit manifest consumed by the guard. Do
not grow `raw_public_ingress.rs` with repository-scanning policy. Focused
fixtures stay in `raw_public_ingress_p0.rs` or a small sibling when needed.

## Acceptance

```text
PUBLIC-INGRESS0 task says closed
PUBLIC-INGRESS0 guard passes while this repair row is active
compile_raw_with_source definition = 1
compile_raw_with_source non-test callers = 0
old compiler Raw run_raw non-test callers = 0
host-provider AST-JSON is classified cfg(test)
runtime AST-JSON remains production legacy

empty Raw -> empty Raw reuse = success
typed public failure -> empty Raw reuse = success
public failure preserves live Builder snapshot
public error begins [raw-public/<stage>/
legacy fallback = 0
normal/JSON route delta = 0
```

## Required gates

```bash
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_public_ingress_p0 -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_public_ingress0_guard.py
cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-claims

```text
ambient-import correction
StaticHelper0 coverage
Legacy-vs-Raw parity
old Raw source deletion
normal/public production consumer
JSON/executor/selfhost/fastmem/CUT0
```

## Next row

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-INGRESS-CONFIG0-S0
```
