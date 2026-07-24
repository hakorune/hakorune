# RAW public cutover PARITY0 execution task

Decision: `RAW-PUBLIC-CUTOVER-PARITY0-prime-r1`

Status: held at the first exact effect mismatch. Resume only after the paired
effect-repair task
`cut0-i0-raw-source0-lower-root-post0-public-cutover-parity0-effect-repair-s0-execution-task-2026-07-25.md`
closes. The original implementation remains authorized only after the paired
design lock
`cut0-i0-raw-source0-lower-root-post0-public-cutover-parity0-design-question-2026-07-24.md`.

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

Compare normalized using the sole test-only snapshot authority
`raw_public_cutover_parity_snapshot.rs`:

```text
function set
signature / arity / return / effects
CFG / value / op / constant relation
source_file, Main declaration facts, symbol, arity, return, effects,
specialized-lane absence
verification disposition
VM-observable behavior where applicable
```

The snapshot is deterministic: function symbol order, fixed entry/edge order,
definition-order ValueId normalization, and an allowlisted MIR operation
surface. Unknown shapes fail fast. `MirPrinter`, module JSON, parser output,
and backend serializers are not parity authorities.

The exact success counts are seven literals, three unary operators, and sixteen
ordinary binary operators. `And`/`Or` are excluded; CompoundAssignment uses the
same ordinary operator table and is not silently narrowed to arithmetic.

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

## Test-only implementation boxes

Production policy and lowering remain unchanged. Add only these siblings:

```text
raw_public_cutover_parity_snapshot.rs
raw_public_cutover_parity_success_p0.rs
raw_public_cutover_parity_failure_p0.rs
raw_public_cutover_reuse_p0.rs
```

`compiler/mod.rs` receives test-module registration only. A snapshot mismatch
is a PARITY0 stop and opens a separate repair/design row; it must not be fixed
by changing production behavior in this task.

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
 bounded matrix rows complete: literal=7, unary=3, binary=16, reuse=4
 normalizer producer=1; Legacy/Raw pair helper=1
 MirPrinter/module_to_mir_json/parser parity authority=0
 unsupported fallback = 0
JSON/executor/selfhost/fastmem delta = 0
all modified source/check files < 800 lines
```

## Next row

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-PARITY0-EFFECT-REPAIR-S0
```
