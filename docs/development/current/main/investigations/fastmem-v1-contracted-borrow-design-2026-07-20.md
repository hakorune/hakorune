---
Status: Design lock (parked)
Decision: accepted for task planning; production activation remains forbidden
Date: 2026-07-20
Scope: FastMem naming, capability semantics, backend boundary, V1 task order, and V0 retirement
Current-lane effect: none; FINALIZE0 CFGSTREAM0 remains authoritative
Related:
  - docs/development/current/main/investigations/fastmem-v1-execution-task-2026-07-22.md
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/design/contract-region-v0-ssot.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/design/fastmem-source-syntax-smoke-taxonomy-ssot.md
  - docs/reference/language/low-level-capabilities.md
  - docs/reference/language/EBNF.md
  - src/mir/builder/fastmem.rs
  - src/mir/contracts/fastmem_ops.rs
  - src/mir/fastmem_access_plan.rs
---

# FastMem V1 Contracted-Borrow Design

## Decision

Keep the public name `fastmem`.

Define its eventual canonical meaning as:

> A FastMem region is a scoped, non-owning, non-escaping raw-view capability
> borrowed from one explicitly branded pinned-memory anchor and constrained by
> one target-specific layout contract.

`raw view` is explanatory terminology. It is not a new public source type or
keyword. In particular, do not rename the current memory profile to `rawview`,
`rawmem`, `memview`, `contractmem`, or `fastpath`.

The selected canonical V1 surface is an explicit role-binding header plus one
mandatory compile-time capability alias:

```hako
fastmem PageMapV1(
    anchor = arena,
    table = arena.page_table,
    length = arena.page_count,
) as page_map {
    prove 0 <= key && key < page_map.length

    local page = page_map.table[key]
    local used = page.used
}
```

This is a language-design lock, not an active grammar change. Before V1 syntax
is implemented, the required `docs/reference/**` decision must be recorded and
both the Rust parser and selfhost parser/JSON route must be covered in one
grammar row.

The current surface remains classified as:

```text
fastmem PageMapV0 { ... }
  = LegacyAmbientFastMemRegionV0
  = migration-only compatibility surface

fastmem PageMapV1(...) as alias { ... }
  = canonical explicit-capability surface
```

V1 never synthesizes missing bindings or an implicit alias. A V1 failure never
falls back to V0.

## Why the public name stays `fastmem`

`fastmem` already names the explicit region envelope and is established in
source, documentation, fixtures, MIR metadata, and diagnostics. The problem is
not the word. The problem is that the current region is ambient and its value
classes are under-specified.

The rejected alternatives are worse:

```text
rawview:
  conflicts conceptually with the existing safe ownership/view surface

rawmem:
  suggests general raw-pointer or general unsafe-memory authority

memview:
  sounds like a safe span/buffer abstraction

contractmem:
  exposes the implementation mechanism and overlaps ContractRegion naming

fastpath:
  remains a parked candidate for a future generic multi-profile envelope;
  it does not rename the current memory profile
```

The precise meaning belongs in the versioned contract (`PageMapV1`) and in the
verified capability products, not in a longer keyword.

The contracted-borrow description shares no representation or proof authority
with the ownership subsystem's `AnchoredView` ABI. Reusing `view` vocabulary
must never make an ownership view a FastMem anchor or raw-role proof.

## Repository findings

The existing implementation is beyond parse-only but is not yet the desired
contracted-borrow model.

### Confirmed useful foundations

```text
public explicit fastmem region syntax
versioned contract spelling
FastMemRegionId side metadata
closed MemOpKind vocabulary and arity/effect tables
field mutability classes
atomic-field plain-store rejection
Verified / Symbolic / Rejected access-plan vocabulary
some escape rejection for MemOp-derived values
llvmlite GEP/load/store implementation
```

These foundations remain useful.

### Confirmed drift or debt

1. `src/parser/statements/fastmem.rs` still describes the surface as
   parse-only, while MIRBuilder already registers a region and lowers its body.
2. `fields.rs` and `indexing.rs` route by ambient active-region state. An
   unrelated managed field/index expression can therefore be reinterpreted as
   raw merely because it appears inside a FastMem body.
3. the source header owns only a contract name and body. It does not bind an
   anchor, table, length, lifetime, or alignment authority.
4. the nine `mem.assume*` intrinsics directly inject facts. They are not
   runtime checks and are not canonical proof evidence.
5. the generic value-MemOp path publishes `MirType::Integer`. FieldLoad may
   reserve an already-declared type, but falls back to Integer compatibility
   when it has none. Table views, layout references, addresses, booleans, and
   numeric scalars therefore still lack one exact durable value-class owner.
6. verified access plans are associated with `(block, instruction_index)` and
   reconstructed by a completed-MIR scan.
7. the PageMap layout path still uses host `usize::BITS` in places where target
   data layout must be authoritative.
8. the escape verifier is a useful seed, but it tracks only a subset of
   MemOp-derived capabilities and aliases.
9. `FastMemBackend::LlvmNative` does not accurately identify the daily
   `ny-llvmc` consumer. The llvmlite compatibility path is not daily-mainline
   completion evidence.
10. the Rust MIR interpreter has no MemOp execution owner.
11. the design SSOT names `MemValueKind`, but no durable Rust value-class owner
    currently implements that contract.

The current `lang/src` users are narrowly concentrated under the allocator
memory tree and use `PageMapV0`. This is a tractable migration corpus, not
evidence that ambient FastMem should become a general language rule.

## Canonical authority chain

The selected V1 chain is:

```text
trusted allocator/runtime provider
  -> FastMemAnchorV1
  -> VerifiedFastMemRoleBindingsV1
  -> VerifiedFastMemCapabilityV1
  -> VerifiedFastMemProofV1
  -> producer-sealed FastMem access sites
  -> VerifiedFastMemRegionV1
  -> VerifiedFastMemAccessPlanV1
  -> supported backend consumer
```

No later owner may reconstruct a missing earlier authority from names, target
strings, ValueIds, block order, runtime tags, or final MIR metadata.

### `FastMemAnchorV1`

The header spelling `anchor = arena` is not proof by itself. Canonical V1
accepts only an opaque, contract-compatible anchor issued by a trusted
allocator/runtime/FFI boundary.

The anchor seals at least:

```text
provider identity
storage identity
pinning/lifetime root
contract compatibility
target data-layout brand
permitted role projections
```

Forbidden producers:

```text
arbitrary Integer
arbitrary Box
method or owner spelling
runtime class name
final metadata inference
raw success on a legacy route
```

The first PageMapV1 profile should use a contract-specific anchor rather than
introducing a general public `RawPtr` type.

### `VerifiedFastMemRoleBindingsV1`

The complete header is co-sealed in one construction transaction. It verifies:

```text
exact role set
no missing or duplicate role
role representation
all table/length roles belong to the exact anchor
contract brand and target-layout brand equality
source evaluation/side-effect law
foreign anchor/role pairing rejection
```

For the first profile, role expressions must be exact contract-admitted
projections or sealed scalar authorities. Arbitrary calls and heuristic field
spelling are not admitted as role proof.

Header lifecycle law:

```text
1. seal the complete semantic role plan before Builder/MIR/fact mutation
2. reject Call, property getter, mutable lookup, or other effectful role input
3. evaluate admitted physical inputs exactly once in source role order
4. snapshot table/length authorities at region entry
5. activate the capability only after every physical role succeeds
6. require the anchor definition to dominate every access
7. forbid anchor move, release, replacement, or unpin while the region is live
```

Header evaluation occurs on an owned candidate transaction. Failure publishes
no capability, region, site, access plan, or partial role fact, and the same
partially created V1 session is not retried through V0.

### Mandatory capability alias

`as page_map` creates a compile-time capability namespace, not a runtime value.

```text
authority identity:
  FunctionLoweringSession brand
  + FastMemCapabilityBindingIdV1
  + exact FastMemAnchorV1 brand
  + contract/target-layout fingerprint

not authority:
  source spelling `page_map`

runtime ValueId:
  none

Return / Store / Call / Capture:
  impossible
```

These identities are sealed together in one non-Clone,
function-session-lifetime-bound `VerifiedFastMemCapabilityV1`. The capability
and `VerifiedFastMemRegionV1` cannot be independently re-paired after sealing.

Only exact projections from this handle may create raw roots:

```text
page_map.table:
  RawTableView<PageMeta>

page_map.length:
  exact length authority
```

An unrelated expression inside the region remains ordinary:

```hako
local page = page_map.table[key]       // FastMem TableIndex
local x = ordinary_box.value           // ordinary FieldGet
local y = ordinary_array[key]           // ordinary Index
```

The alias is fresh for the first V1 profile and cannot be redeclared anywhere
under its active region. Nested FastMem regions reject in the first profile;
their containment, shared-anchor, and inner-value escape laws belong to a later
`FASTMEM-NESTED0` row. Unknown role projections reject; they never fall back to
ordinary field lookup.

### Provenance after projection

After `page_map.table[key]`, later field access is selected from the value's
sealed FastMem provenance, not from ambient region membership:

```text
page_map.table
  -> RawTableView<PageMeta>

page_map.table[key]
  -> LayoutRef<PageMeta>

page.used
  -> FieldLoad<PageMeta.used, usize>
```

This is the structural replacement for `current_fastmem_region()` as a raw
route selector.

## Proof vocabulary

Debug and release builds must have identical language semantics.

### First V1: `prove`

```hako
prove 0 <= key && key < page_map.length
```

Meaning:

```text
verified before dependent MIR emission
runtime instruction delta = 0
failure = typed compile-time rejection
debug/release meaning = identical
```

The first V1 vertical admits only `prove`. Existing `mem.assume*` facts are not
proofs for V1.

The proof authority is a pre-Builder `VerifiedFastMemProofV1`; final
`semantic_refresh`, final MIR scans, and legacy `RangeIndexFact` injected by
`mem.assume*` are not V1 authorities.

Admission widens in two explicit steps:

```text
FASTMEM-PROVE0-LITERAL0:
  exact compile-time nonnegative integer index
  exact entry-snapshot length
  direct arithmetic comparison only
  no branch/loop inference

FASTMEM-PROVE0-RANGE0:
  sealed source-level induction/range fact
  exact dominance over every selected access
  required before the hot-loop performance row
```

The functional FieldLoad executable uses LITERAL0. The performance fixture does
not start until RANGE0 closes. This avoids accepting a legacy injected range
fact merely to obtain a benchmark loop.

### Later runtime check: `require`

The existing bare `guard` spelling already has language meaning as an
early-exit statement. It must not be overloaded as a FastMem bounds check.

If runtime proof is later needed, open a separate `FASTMEM-REQUIRE0` language
row with an explicit, always-on check/trap contract:

```hako
require 0 <= key && key < page_map.length
```

`require` is parked until the language and MIR have a selected fail-fast
runtime-check owner. It is not part of the first FieldLoad slice.

### Later trusted assumption

Expert-only `trusted assume` is parked. If opened, it requires an explicit
trusted boundary and may permit diagnostic shadow checks, but debug checks do
not change its language meaning.

## Value classes and escape law

Do not expose FastMem capabilities as ordinary `MirType::Integer` or as a
general source-level pointer type.

Selected internal vocabulary:

```rust
pub(crate) struct FastMemValueBrandV1 {
    session: FunctionLoweringSessionBrandV1,
    capability: FastMemCapabilityBindingIdV1,
    anchor: FastMemAnchorBrandV1,
    region_generation: FastMemRegionGenerationV1,
    contract: FastMemContractFingerprintV1,
}

pub(crate) enum FastMemValueClassV1 {
    RawTableView {
        brand: FastMemValueBrandV1,
        layout: FastMemLayoutIdV1,
    },
    LayoutRef {
        brand: FastMemValueBrandV1,
        layout: FastMemLayoutIdV1,
    },
    RawAddress {
        brand: FastMemValueBrandV1,
    },
    Scalar(FastMemScalarTypeV1),
    AllocOwnerId(FastMemTargetBrandV1),
    ProofToken {
        brand: FastMemValueBrandV1,
        kind: FastMemProofKindV1,
    },
}
```

`LayoutRef` is the V1 implementation of the existing docs-only
`MemValueKind::LayoutRef` concept; there is not a second `RawLayoutRef` truth.
`AllocOwnerId` stays distinct from language object ownership and ContractRegion
producer identity.

Raw value compatibility requires exact brand equality, not merely equal layout
names. A raw value from another function session, anchor, region generation, or
contract fingerprint rejects before instruction emission.

`FastMemScalarTypeV1` preserves both physical storage and ordinary-language
projection:

```text
target storage width / signedness / alignment
target-layout brand
contract field identity
exposed MirType
projection law = exact identity | proven lossless conversion
```

V1 does not silently turn target `usize` or `u64` into Hakorune `Integer`. A
scalar may leave the region only when the contract carries an exact identity or
a proof that the value is losslessly representable. The first fixture must
name that projection explicitly.

Region-exit law:

| Value class | May leave region |
|---|---:|
| `RawTableView` | no |
| `LayoutRef` | no |
| `RawAddress` | no |
| `ProofToken` | no |
| exact contract scalar | yes |
| managed Box | ordinary ownership law |

Consequently, returning a scalar load may be accepted while returning a page
layout reference is rejected. Copy/Phi aliases of raw classes retain the same
escape restriction.

Recipe and atomic rows are parked. Each future recipe result must map to one
already-selected exact value class or open a separate value-class decision; no
implicit `RecipeToken` class exists in V1.

## MemOp vocabulary freeze

No new MemOp kind is admitted before the first V1 FieldLoad vertical closes.

Classify the existing 17 kinds exactly once:

| Class | Existing kinds |
|---|---|
| permanent primitive | `TableIndex`, `FieldLoad`, `FieldStore` |
| proof/observation | `CurrentAllocOwnerId`, `OwnerEq` |
| verified PageMap recipe | `LocalFreePush`, `LocalFreePop`, `FreeHeadPush`, `FreeHeadPop`, `AtomicRemoteHeadPush`, `AtomicRemoteHeadDrain`, `DrainRemoteListToLocal` |
| migration-only | `AddrOf`, `LogicalShr`, `BitAnd`, `Add`, `Sub` |

Normal arithmetic, shifts, bit operations, and comparisons use ordinary MIR.
Free-list and remote-free operations are PageMap allocator recipes rather than
generic memory primitives. `AddrOf` does not become a general source API.

The nine `mem.assume*` intrinsics are frozen as legacy fact-injection
vocabulary and are forbidden in PageMapV1.

## Target and access-plan law

### Target-specific layout

Every V1 contract fingerprint includes:

```text
target triple
pointer width
endianness
alignment rules
contract version
layout/table schema
```

Host `usize::BITS` is not a target-layout authority.

### Stable access identity

Every physical MemOp carries a stable access identity:

```rust
MemOp {
    region: FastMemRegionId,
    site: FastMemAccessSiteIdV1,
    kind: MemOpKind,
    dst: Option<ValueId>,
    operands: Vec<ValueId>,
    ...
}
```

`(block, instruction_index)` is a transient location, not durable semantic
identity.

Site update law:

```text
ValueId remap / instruction move:
  preserve site id

instruction duplication:
  same site id reuse forbidden
  issue a fresh site + fresh verified plan, or reject

instruction deletion:
  invalidate the corresponding active plan in the same transaction

coherence:
  one active physical MemOp <-> one active verified plan

post-optimizer:
  site/plan freshness verifier required
```

Block and instruction indexes may remain diagnostic locations only.

Because `MirInstruction::MemOp` is shared, SITE0/MIRJSON0 must update every V0
producer, remapper, pattern match, MIR JSON encoder/decoder, and llvmlite
consumer before V1 production uses the field. No V1-only side map may hide a
missing instruction transport. V0 may carry the stable site during migration,
but that does not make its current access-plan container V1 authority.

Site reservation is a function-session-generation-scoped typestate:

```text
Reserved
  -> EmittedPending
  -> Completed(VerifiedFastMemAccessPlanV1)
  or Aborted
```

Dependent lowering cannot observe `Reserved` or `EmittedPending`. Failed
emission consumes the token as `Aborted` and publishes no site, fact, region
membership, or plan. Session close rejects an unconsumed token. Clone,
rematerialization, and compiler reuse cannot reuse a site id across a distinct
physical instruction or function-session generation.

### Producer-first publication

The selected publication order is:

```text
verified source access request
  -> reserved stable site
  -> successful physical MemOp
  -> region proof closure
  -> VerifiedFastMemAccessPlanV1
```

Finalization may verify, snapshot, or publish an already-sealed plan. It may
not infer the first layout, offset, bounds, mutability, or site correspondence
by rescanning completed MIR.

The plan holds exact contract fingerprint, layout/field/table identity, byte
offset, size, alignment, bounds/overflow proof, mutability, alias class, and
anchor brand. Backends consume it without name or offset reconstruction.

The current `FastMemAccessPlan` (`SymbolicOnly | Verified | Rejected`) is a V0
migration/decision container only. `VerifiedFastMemAccessPlanV1` is the sole V1
backend-lowering authority:

```text
V1 backend reads current FastMemAccessPlan:
  0

one V1 physical site:
  exactly one active VerifiedFastMemAccessPlanV1
```

## Backend decision

### Daily AOT

The daily completion owner is the non-replay `ny-llvmc` path. The existing
llvmlite implementation is a compatibility/reference implementation, not proof
that the daily backend supports a V1 operation.

Backend identities must eventually distinguish at least:

```text
LlvmLiteKeep
NyLlvmcMainline
HakoMirInterpreter
```

The current broad `LlvmNative` label must not be used to claim all three.
`NyLlvmcMainline` remains closed initially; `FASTMEM-NYLLVMC-FIELDLOAD0` opens
exactly the selected TableIndex + FieldLoad family.

### Rust VM / future Hako interpreter

Do not implement MemOp in the retiring Rust MIR interpreter.

Until the future `.hako` MIR interpreter owns FastMem semantics, any module
containing an executable FastMem site must fail before VM execution:

```text
[fastmem/backend-unsupported] backend=mir-interpreter
MIR execution delta = 0
```

This rejection must be unconditional for the selected backend and must not
depend on a dev/planner flag.

The stable tag must be registered in the diagnostic/debug contract SSOT before
code uses it. Preflight applies to every executable V0 or V1 MemOp-bearing
module selected for the retiring Rust interpreter; it intentionally changes
the error timing from execution-time unsupported instruction to pre-execution
rejection.

The future interpreter must not dereference arbitrary host pointers. Its
reference representation is:

```text
FastMemArenaId + checked byte offset + layout id
```

The later interpreter row opens one operation family at a time and compares
normalized results with AOT. VM parity is therefore not a blocker for the
first AOT FieldLoad slice; early unsupported-backend rejection is.

## First vertical slice

The first executable V1 slice is exactly:

```text
PageMapV1
  -> one branded anchor and exact role set
  -> one compile-time capability handle
  -> one statically proved index
  -> TableIndex
  -> LayoutRef<PageMeta>
  -> FieldLoad<usize>
  -> scalar result
```

Not admitted in this slice:

```text
FieldStore
runtime require
trusted assume
owner comparison
free-list recipes
remote/atomic recipes
general raw pointer
pointer arithmetic
source-defined contract declarations
Rust VM execution
nested FastMem regions
```

Required daily AOT evidence:

```text
Rust parser + selfhost parser/AST/JSON parity
exact role/anchor binding
target layout/fingerprint seal
static bounds proof
stable access site
raw capability escape count = 0
ny-llvmc pure/mainline path
compat replay = none
generated executable result
controlled C fixture result parity
```

The first anchor issuer is a controlled test/FFI fixture only. Activating the
allocator mainline or global allocator provider is not part of FieldLoad0.
For `PageMeta.used`, that fixture seals the exact projection:

```text
target usize storage
  + invariant 0 <= used <= capacity <= Hakorune Integer::MAX
  -> proven-lossless Hakorune Integer
```

Without this invariant, the scalar result remains non-exportable and the
fixture rejects rather than truncating or reinterpreting signedness.

Required assembly shape:

```text
table-slot address calculation = 1
page pointer load = 1
field scalar load = 1
helper call = 0
Box dispatch = 0
boxing = 0
runtime contract lookup = 0
string field lookup = 0
per-access runtime bounds check = 0
```

Performance judgment follows the owner-first performance SSOT. Exact-front
instruction shape is primary evidence; wall time alone does not select a
keeper. A controlled median within 3% of the matching C baseline is the target,
not a substitute for the assembly gate.

## Parked task order

This design does not replace the active FINALIZE0 blocker. When FastMem is
explicitly reopened by `CURRENT_STATE.toml`, use this order:

The normalized execution board, audit anchors, gates, and stop conditions live
in `fastmem-v1-execution-task-2026-07-22.md`.  This section remains the compact
dependency order; the execution board is the code-facing restart surface.

```text
FASTMEM-V1-D0                       this design lock; docs only

FASTMEM-BASELINE0
  remove one test-only HashMap block-order expectation
  production behavior delta = 0

FASTMEM-SSOT-DRIFT0
  correct parse-only/backend-label/MemValueKind/current-route drift
  behavior delta = 0

FASTMEM-VOCAB-FREEZE0
  classify all 17 MemOps and nine assume intrinsics
  reject unclassified/new vocabulary

FASTMEM-BACKEND-ID0
  explicit llvmlite / ny-llvmc / Rust-VM / Hako-interpreter identities
  behavior delta = 0

FASTMEM-BACKEND-PREFLIGHT0
  register the stable diagnostic contract first
  reject every V0/V1 MemOp-bearing module before Rust-VM execution
  behavior delta = yes; execution delta = 0

FASTMEM-TARGET0
  target data layout and fingerprint
  retire host-layout authority from V1

FASTMEM-CONTRACT0
  compiler/stdlib-owned sealed PageMapV1 registry
  source-defined contract syntax remains parked

FASTMEM-ANCHOR0-S0
  opaque pinned-anchor schema and foreign-anchor errors
  production producers/consumers = 0

FASTMEM-BIND0-S0
  role-binding and mandatory capability-alias products
  production consumers = 0

FASTMEM-VALUECLASS0-S0
  opaque table/ref/scalar/proof classes
  exact session/anchor/region/target brands
  production consumers = 0

FASTMEM-SITE0-S0
  stable access-site typestate, generation, and update law
  production consumers = 0

FASTMEM-REGION0-S0
  pending/verified region schemas
  CFG membership, exit edges, pinning/lifetime, and escape law
  production consumers = 0

FASTMEM-PROVE0-S0
  static proof schema only
  production consumers = 0

FASTMEM-V1-PARSE0
  accepted docs/reference language decision
  Rust parser + selfhost parser + AST/Program JSON parity
  production V1 lowering consumers = 0

FASTMEM-ANCHOR0-I0
  controlled test/FFI anchor producer only

FASTMEM-BIND0-I0
  exact role seal + capability handle activation
  V1 ambient selection = 0

FASTMEM-VALUECLASS0-I0
  V1 TableIndex/FieldLoad use opaque branded value classes

FASTMEM-PROVE0-LITERAL0
  exact literal index + exact entry length only
  runtime require / trusted assume / range inference = 0

FASTMEM-SITE0-I0
  one reservation token per planned physical access

FASTMEM-ACCESSPLAN0-S0
  producer-sealed success-only V1 plan schema
  current V0 FastMemAccessPlan consumers = 0

FASTMEM-MIRJSON0
  stable site + contract fingerprint + sealed-plan transport
  disconnected synthetic V1 rows before physical production

FASTMEM-FIELDLOAD0
  one TableIndex + FieldLoad physical producer
  failed emission leaves site/fact/plan delta 0

FASTMEM-REGION0-I0
  close exact member sites, CFG exits, anchor lifetime, and escape proof

FASTMEM-ACCESSPLAN0-I0
  complete one verified plan per surviving physical site

FASTMEM-NYLLVMC-FIELDLOAD0
  open exactly TableIndex + scalar FieldLoad on daily non-replay AOT

FASTMEM-FIELDLOAD0-EXE0
  generated executable and controlled result parity

FASTMEM-PROVE0-RANGE0
  sealed source induction/range + exact dominance proof

FASTMEM-FIELDLOAD0-PERF0
  exact assembly and controlled C comparison

then, one family at a time:
  FASTMEM-FIELDSTORE0
  FASTMEM-OWNER0
  FASTMEM-FREELIST0
  FASTMEM-REMOTE0

later interpreter:
  HMI-X0-FASTMEM-FIELDLOAD0

future nested regions:
  FASTMEM-NESTED0
```

`FASTMEM-CONTRACTDECL0`, `FASTMEM-REQUIRE0`, and trusted-assume support remain
independent language-widening rows.

During FINALIZE0, the existing completed-MIR plan refresh must be inventoried
under the explicit retirement owner:

```text
FASTMEM-ACCESSPLAN-REFRESH-RET0
```

It must not be silently blessed as permanent normalization.

## Retirement ledger

The V1 work does not claim retirement until repository callers and backend/JSON
parity are zero/green. Required retirement owners are:

```text
FASTMEM-AMBIENT-RET0
  current_fastmem_region-only raw selection
  after V0-SYNTAX-RET0, or an independent zero-caller proof

FASTMEM-ARITH-RET0
  AddrOf and arithmetic MemOps
  after every surviving source producer uses ordinary MIR or a branded role
  and repository callers = 0

FASTMEM-ASSUME-RET0
  all mem.assume* source fact injection
  after PROVE0 + OWNER0 + FREELIST0 + REMOTE0
  and every V0 assume caller = 0

FASTMEM-OWNEROP-RET0
  CurrentAllocOwnerId/OwnerEq to branded AllocOwnerId + ordinary Compare
  after OWNER0

FASTMEM-RECIPE-TRANSPORT-RET0
  free-list/remote operations from primitive-core treatment
  after FREELIST0 + REMOTE0

FASTMEM-V0-SYNTAX-RET0
  bare PageMapV0 compatibility surface
  after source/parser/JSON/backend caller count = 0

FASTMEM-ACCESSPLAN-REFRESH-RET0
  completed-MIR semantic reconstruction and block/index identity
  after producer-sealed SITE0/ACCESSPLAN0 covers every surviving family
  and refresh consumers = 0

FASTMEM-CHECK-RET0
  legacy oversized one-off check/report surfaces after replacement guards land
  after replacement manifests cover the same inventory
  and legacy script callers = 0
```

This is a dependency DAG, not a cleanup checklist. In particular, the one
FieldLoad vertical cannot retire completed-MIR refresh for FieldStore, owner,
free-list, or remote families.

New reusable guards belong under a small manifest-driven
`tools/checks/lib/fastmem_v1/` family. Do not append new V1 policy to existing
oversized check scripts.

## Implementation claims after the first vertical

The first V1 FieldLoad closeout may claim only:

```text
one explicit branded PageMapV1 capability selects one raw table root
unrelated expressions in the region retain ordinary semantics
TableIndex produces an opaque layout reference, not an Integer fact
one statically proved scalar FieldLoad executes on daily non-replay AOT
the Rust VM rejects the module before execution
raw capability values do not escape
backend lowering consumes target-specific producer-sealed access plans
```

It must not claim:

```text
general unsafe blocks
general raw pointers or pointer arithmetic
runtime checked FastMem
trusted assumptions
FieldStore/free-list/atomic completion
general user-defined FastMem contracts
VM/interpreter parity
PageMapV0 retirement
all legacy access-plan refresh retired
all allocator code migrated
```

## Stop conditions

Stop implementation and reopen design consultation if any row requires:

1. constructing an anchor from an arbitrary Integer or Box;
2. treating region membership alone as raw-route authority;
3. publishing a table/layout reference as ordinary `MirType::Integer`;
4. inferring contract, field, table, or offset from spelling in a backend;
5. using host pointer width or alignment as target truth;
6. restoring block/instruction index as durable site identity;
7. letting final semantic refresh create the first access plan;
8. accepting a `mem.assume*` fact in PageMapV1;
9. changing proof meaning between debug and release;
10. adding runtime `require` or trusted assume to the first slice;
11. implementing MemOp in the retiring Rust VM;
12. counting llvmlite compatibility replay as daily AOT completion;
13. opening FieldStore/free-list/atomic before FieldLoad closes;
14. introducing a general public raw pointer or pointer arithmetic;
15. falling back or retrying through an ordinary/V0 route after V1 failure;
16. opening source contract declarations with use-site binding in one row;
17. replacing the current FINALIZE0 blocker without an explicit lane switch;
18. adding policy to a source/check file that reaches 800 lines;
19. pairing a raw value with a foreign capability, anchor, region, or session;
20. re-evaluating a role expression or admitting a Call/property getter role;
21. observing mutable table/length drift instead of the sealed entry snapshot;
22. allowing anchor non-dominance, move, release, replacement, or unpin;
23. lowering with a backend target different from the contract fingerprint;
24. leaving an orphan site, fact, region member, or plan after failed emission;
25. retrying the same partially mutated V1 session after failure;
26. duplicating, mutating, or cross-generation-reusing a stable site id;
27. bypassing stable-site/fingerprint/plan transport in MIR JSON;
28. silently projecting target `usize`/`u64` into Hakorune `Integer`.

## Final decision lock

> `fastmem` remains the public name. Canonical PageMapV1 is a contract-bound
> borrow from one opaque pinned-memory anchor, not an ambient unsafe block. Its
> exact anchor/table/length roles are co-sealed before lowering and exposed only
> through one mandatory, non-value `as alias` capability handle. Only exact
> handle projections and their typed derivatives may select TableIndex,
> FieldLoad, or FieldStore; unrelated expressions remain ordinary. FastMem raw
> table, layout-reference, address, and proof values carry exact
> session/anchor/region/contract brands, are never general language pointers or
> ordinary Integer facts, and cannot escape the verified region. Scalar loads
> require an explicit exact or proven-lossless target-to-language projection;
> future recipe results receive no implicit value class. PageMapV1 initially
> admits only static `prove`; runtime
> `require`, trusted assumptions, and source-defined contracts remain separate
> widening rows with build-mode-independent semantics. Every access uses one
> generation-scoped stable site id and a target-specific producer-sealed V1
> access plan; the current mixed V0 access-plan container is not backend
> authority for V1. The first executable slice is one literal-proved TableIndex
> plus scalar FieldLoad
> through daily non-replay ny-llvmc, executable, assembly, and C-comparison
> gates, with a separate range-proof row before hot-loop performance. The
> retiring Rust VM gains no MemOp implementation and instead rejects
> FastMem before execution; one future `.hako` MIR-interpreter family will use
> arena-relative addresses. PageMapV0, ambient rawization, assume injection,
> arithmetic MemOps, recipe-as-primitive treatment, and completed-MIR plan
> reconstruction remain explicit retirement rows rather than silent cleanup.
