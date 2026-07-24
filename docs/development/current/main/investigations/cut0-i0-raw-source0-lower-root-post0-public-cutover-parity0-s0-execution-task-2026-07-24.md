# RAW public cutover PARITY0 execution task

Decision: `RAW-PUBLIC-CUTOVER-prime-r1`

Status: queued after `PUBLIC-CUTOVER-COVERAGE0-S0`.

## Goal

Prove bounded Legacy-vs-Raw parity for the exact sealed NarrowV1 grammar.
This is a test/proof row; it does not switch a production caller.

## Success relation

```text
empty Script
seven literal variants
three admitted unary operators
ordinary binary operator table
Expr / Print / Local / Assignment / CompoundAssignment
App empty/scalar main
App exact-empty StaticHelper0
optimize on/off
source-file hint
Raw -> Raw / Raw failure -> Raw / Raw -> Legacy / Legacy -> Raw reuse
```

Compare normalized:

```text
function set
signature / arity / return / effects
CFG / value / op / constant relation
backend-required metadata
verification disposition
VM-observable behavior where applicable
```

## Failure relation

```text
REPL / invalid root route
Script declaration / non-Main App
If / Loop / LoopRange / Return / Break / Continue / ScopeBox
And / Or / unsupported unary
typed local / cardinality drift / invalid assignment target
App metadata or arity drift
undefined variable
helper outside StaticHelper0
dirty publication target
```

Every failure proves:

```text
stable raw-public stage/code
live Builder unchanged
no MirCompileResult
no legacy fallback
subsequent reuse success
```

POST0 natural fault injection is not added. Existing lower-level typed
optimizer/contract-refresh fixtures are cross-evidence.

## Structure and gate

Keep the normalized parity vocabulary in a test-only sibling module rather
than copying production policy.

```bash
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_public_cutover_parity_p0 -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_public_cutover_parity0_guard.py
cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
```

## Guard

```text
normal-entry Raw consumer = 0
bounded matrix rows complete
unsupported fallback = 0
JSON/executor/selfhost/fastmem delta = 0
all modified source/check files < 800 lines
```

## Next row

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-OLD-RAW-RETIRE0-R0a
```
