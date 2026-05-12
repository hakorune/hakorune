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
- VM is not a product owner. VM rows in this phase are semantic reference
  execution only: they may execute MIR-owned facts/contracts, but they do not
  make VM-only behavior a completion criterion.
- VM green is not hako_alloc/mimalloc green. hako_alloc live field migration
  waits for backend fail-fast/lowering and typed-object storage boundaries.
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
- `294x-06f`: unsupported non-VM backend routes now fail fast when a module
  contains exact numeric runtime-check contracts, while MIR JSON diagnostic
  export stays available.
- `294x-07`: the MIR numeric substrate now owns checked exact numeric
  add/sub/mul policy, rejecting type mismatch and out-of-range results before
  any VM/backend lowering claims support.
- `294x-08`: the MIR numeric substrate now owns exact numeric compare and
  unsigned logical right-shift policy, including type-mismatch and invalid
  shift-count fail-fast paths.
- `294x-09`: exact numeric PHI/Select control-merge policy now lives in its
  own small module and preserves exact facts only when every incoming type is
  identical.
- `294x-09a`: VM reference-executor boundary is fixed: VM is not a product
  owner, and future VM exact numeric rows are semantic reference execution
  only.
- `294x-09b`: MIR semantic refresh now publishes exact numeric value facts for
  exact numeric field reads, `Copy`, and conservative `Phi`/`Select` merges;
  rejected exact/dynamic or exact/exact mismatches stay visible as metadata.
- `294x-09c`: MIR function metadata now preserves declared parameter/return
  annotation text from AST lowering; exact numeric params seed value facts and
  exact numeric returns publish function-level advisory facts.
- `294x-09d`: exact numeric `BinOp::Add` now publishes MIR-owned route facts
  and exact numeric result value facts when both operands share the same exact
  numeric type; mismatches stay visible as rejection metadata.
- `294x-09e`: `dev_gate.sh quick` is now the slim daily gate; full
  allocator/mimalloc/provider proof coverage moved to the explicit
  `allocator-wide` profile, while quick keeps a provider inactive sentinel.
- `294x-09f`: quick first-row guards now share a cargo filter grouping helper
  and use contract-family filters to reduce repeated cargo startup without
  changing route/file locks.
- `294x-10`..`294x-10e`: VM reference execution consumes MIR-owned exact
  arithmetic, compare, and logical right-shift route facts without making the
  VM a product owner.
- `294x-11`: decimal integer suffixes such as `0usize` now parse as typed
  integer literals, range-check through exact numeric metadata, and publish MIR
  exact const facts while still emitting current-lane `Integer(i64)` constants.
- `294x-12`: typed-object layout plans now preserve exact numeric storage names
  such as `usize` instead of collapsing them to `i64`, while current execution
  remains on the dynamic integer lane.

## First Implementation Direction

Start with metadata preservation before runtime behavior:

1. attach exact numeric metadata to MIR facts/signature consumers;
2. add VM reference exact `usize` behavior, with backend fail-fast/lowering
   kept visible;
3. migrate hako_alloc non-negative fields.

This keeps the source truth available before any lowerer claims exact
semantics.
