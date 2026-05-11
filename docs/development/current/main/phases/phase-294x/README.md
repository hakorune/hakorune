# Phase 294x: usize semantic foundation

- Status: Active
- Purpose: make pointer-sized unsigned integer semantics real enough for the
  mimalloc `.hako` port to use `usize` without lying about runtime behavior.
- Active lane token: `phase-294x usize semantic foundation`
- Current blocker token:
  `phase-294x exact usize semantics before mimalloc migration`
- Design SSOT:
  `docs/development/current/main/design/usize-semantic-foundation-ssot.md`
- Taskboard:
  `docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md`
- Parent app lane:
  `docs/development/current/main/phases/phase-293x/README.md`

## Policy

- Treat `usize` as a language/runtime completeness feature, not as a
  mimalloc-only shortcut.
- Do not migrate hako_alloc live state to `usize` until exact semantics and the
  relevant verifier/lowering rows are green.
- Keep sentinel-bearing indexes signed.
- Keep BoxShape and BoxCount separate: metadata/schema work, runtime semantics,
  backend lowering, and hako_alloc migration land as separate slices.
- Unsupported backends must fail fast. Silent fallback to `Integer(i64)` is not
  allowed once an exact `usize` row claims support.
- Allocator-provider activation, host allocator replacement, hook install, and
  `#[global_allocator]` remain out of scope.

## Reading Order

1. `docs/development/current/main/design/usize-semantic-foundation-ssot.md`
2. `docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md`
3. `docs/reference/language/types.md`
4. `docs/reference/runtime/substrate-capabilities.md`
5. `docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md`

## Current Status

- `294x-00`: phase lock and full visible task inventory landed.
- `294x-01`: code-side target pointer-width and target-resolved numeric kind
  SSOT landed in `src/mir/numeric_substrate.rs`.
- `294x-02`: parser and AST now preserve parameter declared-type metadata and
  accepted return type annotations while keeping `params` as the names-only
  compatibility surface.
- `294x-03`: AST JSON and Stage1 Program(JSON) now transport declared
  parameter metadata and accepted return type annotations without changing
  runtime semantics.
- `294x-04`: MIR now has a side-car exact numeric type model that preserves
  source spelling plus target-resolved signedness/width distinctly from
  `MirType::Integer`.
- `294x-05`: exact numeric constant metadata and dynamic `Integer(i64)`
  conversions now range-check through the MIR numeric substrate model.
- `294x-06`: the MIR verifier now rejects statically known out-of-range writes
  to exact numeric declared fields, including `usize` fields initialized with
  `-1`.
- `294x-06b`: the MIR verifier now rejects unchecked dynamic writes to exact
  numeric fields whose range does not cover the full dynamic `Integer(i64)`
  lane, keeping `i64` compatible while blocking `usize` until runtime-check
  lowering exists.
- `294x-06c`: function metadata now owns
  `ExactNumericRuntimeCheckContract::DynamicIntegerRange`; the verifier accepts
  dynamic exact numeric field writes only when a matching contract is present.
- `294x-06d`: the VM interpreter now executes existing `DynamicIntegerRange`
  contracts at `FieldSet` sites and rejects non-integer, negative-unsigned, and
  out-of-range dynamic values before field mutation.
- `294x-06e`: MIR semantic refresh now attaches `DynamicIntegerRange`
  contracts for real exact numeric `FieldSet` producers after optimization and
  before verification, with verifier checks consuming the same field-write
  facts owner.

## First Implementation Direction

Start with metadata preservation before runtime behavior:

1. attach exact numeric metadata to MIR facts/signature consumers;
2. make unsupported backend routes fail fast when exact numeric contracts are
   present;
3. add VM/backend exact `usize` behavior;
4. add checked arithmetic / unsigned compare / logical shift;
5. migrate hako_alloc non-negative fields.

This keeps the source truth available before any lowerer claims exact
semantics.
