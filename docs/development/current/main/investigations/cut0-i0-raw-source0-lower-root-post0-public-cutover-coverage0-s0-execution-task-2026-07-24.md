# RAW public cutover COVERAGE0 execution task

Decision: `RAW-PUBLIC-CUTOVER-prime-r1`

Status: held WIP after worker audit. The first implementation is stashed;
`COVERAGE0-REPAIR-CONSULT0` must close before this card can become executable.

## Goal

Close the unbounded App-helper hole before any bounded NarrowV1 parity claim.

## Exact first profile

```text
StaticHelper0
  static
  non-override
  arity = 0
  params = []
  param_decls = []
  return_type_name = None
  uses = []
  attrs = empty
  contracts = []
  body = []
```

Every mismatch is a typed eligibility/coverage rejection before physical
open, reservation, child descent, or Builder effects. The legacy child
lowerer remains a physical mechanism, not helper-grammar authority.

## Structure

Add a small sibling coverage module. Do not grow:

```text
src/mir/compiler/raw_root_source_facts.rs   currently near 800 lines
src/mir/compiler/raw_root_eligibility.rs    already large
```

The coverage module consumes the existing source locator/declaration facts
and produces one sealed `StaticHelper0` witness. CHILDREN0 consumes the
witness and does not re-decide the grammar.

## Fixtures

```text
zero helper
one exact-empty helper
two exact-empty helpers retain lexical order

reject before physical effects:
  helper parameter
  helper return metadata
  helper uses / attrs / contracts
  helper non-empty body
  instance or override helper
```

## Guard

```text
StaticHelper0 coverage producer = 1
CHILDREN0 coverage consumer = 1
helper legacy-AST grammar authority = 0
helper mismatch physical effects = 0
HelperLinear0 activation = 0
normal-entry consumer = 0
all modified source/check files < 800 lines
```

## Non-claims

```text
non-empty helper lowering
helper parameters/returns
source Call
normal-entry cutover
JSON/executor/selfhost/CUT0
```

## Closeout evidence

```text
StaticHelper0 producer = 1
exact empty body/params/return/uses/attrs/contracts = sealed
instance/override helper rejection = pre-physical
CHILDREN0 consumes witness-owned lexical schedule = 1
non-empty helper physical effects = 0
HelperLinear0 activation = 0

python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_public_cutover_coverage0_guard.py
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_root_helper_coverage -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_root_children -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_root_eligibility_p0 -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_public_ingress_p0 -- --test-threads=1
cargo check -q --lib
git diff --check
```

Next row: `RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-PARITY0-S0`.

## Next row

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-PARITY0-S0
```
