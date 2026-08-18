---
Status: SSOT
Decision: accepted
Date: 2026-08-18
Scope: Common contract-region envelope for fast memory and future fastpath profiles.
Related:
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/design/value-corridor-generic-optimization-contract.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
  - docs/development/current/main/phases/phase-296x/archive/296x-458-CONTRACT-REGION-V0-DOCS.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/reference/language/low-level-capabilities.md
---

# ContractRegionV0

## Current Capsule

- **Current decision:** keep `ContractRegionV0` as the generic envelope and
  profile payloads as separate authorities. `fastmem` remains the only
  implemented profile. A portable Text-scan kernel region is recorded only as
  a parked future idea.
- **Current implementation status:** memory-profile `FastMemRegion` exists;
  Text-kernel source syntax, profile metadata, verifier, plan issuer, backend
  consumer, and production activation are all zero.
- **Next ordered task:** continue the currently selected lane. Do not open the
  parked Text-kernel premise until the automatic S6C corridor reaches
  production cutover and the prerequisites below are observable.
- **Production stop line:** no contract-region document authorizes a new
  parser spelling, source leaf, MIR dialect, backend route, raw pointer, or
  fallback.
- **Retirement finish line:** if the Text-kernel idea is later accepted, its
  explicit source producer and the automatic corridor producer converge on
  one verified plan and one physical consumer; no dedicated second
  MIRBuilder or helper-name route remains.

## Decision

`fastmem` is not a separate MIRBuilder. It is the memory-profile instance of a
contract-bound region model.

Long-term shape:

```text
ContractRegionV0:
  common envelope

FastMemRegion:
  memory-profile wrapper over that envelope
```

Do not rename the current `FastMemRegion` code or report fields yet.

Short form:

```text
Envelope is generic.
Payload is profile-specific.
```

## Common Envelope

The common part is:

```text
region_id
profile
contract_id
source_span
origin
flags
obligations
report identity
verifier envelope
producer identity
```

Profiles:

```text
memory:
  current fastmem / mimalloc path

simd:
  future vector fast path

io:
  future buffer/socket fast path
```

The current implementation has only the memory profile.

## Obligations

Obligations are shared vocabulary, but each profile decides which ones are
required:

```text
no_alloc
no_safepoint
no_escape
no_type_abi_hot_lookup
no_provider_abi_hot_dispatch
no_unverified_layout_access
```

Use a stateful obligation model in future code:

```text
required
forbidden
allowed
profile_defined
```

Do not flatten obligations into permanent booleans too early. `fastmem` will
mostly require the strict memory-hot-path set, but future IO/SIMD profiles may
need different blocking, safepoint, or allocation policies.

## Profile-Specific Payloads

Memory-specific payloads stay memory-specific:

```text
MemOpKind
MemValueKind
MemLayoutContract
MemTableContract
MemFieldContract
VerifiedMemAccessPlan
LLVM GEP/load/store lowering
memory-specific escape rules
memory-specific alignment rules
```

Future profiles should add their own payloads instead of forcing memory
concepts into generic names:

```text
memory:
  MemOp
  VerifiedMemAccessPlan

simd:
  SimdOp
  VerifiedVectorPlan

io:
  IoOp
  VerifiedBufferAccessPlan
```

Rejected genericization:

```text
generic RegionOp that hides memory/vector/io semantics
generic VerifiedRegionAccessPlan replacing VerifiedMemAccessPlan
```

## Current Repository Reading

For the current mimalloc lane:

```text
FastMemRegion:
  current memory-profile implementation

FunctionMetadata.fastmem_regions[]:
  current memory-profile region metadata

FunctionMetadata.fastmem_access_plans[]:
  memory-profile verified access plan surface
```

Do not introduce `FunctionMetadata.contract_regions[]` until one of these is
true:

```text
second profile enters implementation
FastMemRegion common header extraction is behavior-preserving
contract_region_* report fields are needed by an active checker
FastMemRegion naming blocks a real implementation task
```

## MIRBuilder Boundary

MIRBuilder remains the common AST-to-MIR representation layer.

MIRBuilder may:

```text
record ContractRegion-style header facts
record memory-profile FastMemRegion metadata
emit MemOp instructions for memory-profile dialect operations
preserve source span / origin / contract id
```

MIRBuilder must not:

```text
choose profile producer routes
choose LLVM vs C
compute layout offsets
choose table representation
choose fast/slow paths
open Type ABI hot lookup
open Provider ABI hot dispatch
claim product activation
```

## Report Reading

Common report fields may be added later:

```text
contract_region_model=1
contract_region_count
contract_region_profile_memory_count
contract_region_profile_simd_count=0
contract_region_profile_io_count=0
contract_region_verifier_pass_count
```

Do not add those fields merely for inventory if the current active row does not
consume them. The active memory lane should continue to use memory-specific
fields:

```text
fastmem_region_count
fastmem_access_plan_count
fastmem_layout_contract_verified
fastmem_table_contract_verified
fastmem_layout_ref_escape_count=0
fastmem_unverified_layout_access_count=0
```

## Task Order

Current order:

```text
1. Define ContractRegionV0 docs-only.
2. Keep FastMemRegion as memory-profile wrapper.
3. Continue MIR-FMEM-008B concrete layout/table contract resolution.
4. Verify field offsets, field types, alignment, table representation, stride,
   and bounds proof for VerifiedMemAccessPlan.
5. Open LLVM GEP/load/store from verified memory plans only.
6. Extract a shared ContractRegionHeader only after it stops being speculative.
```

## Commonization Boundary

Commonization is useful only where it removes duplicated envelope mechanics.
Do not genericize the memory dialect before the first memory producer is
finished.

Commonize now or soon:

```text
region header vocabulary
contract id / profile / source origin
obligation names
verifier/report envelope
shared escape-barrier classifier inputs
small verifier traversal utilities when several verifiers need the same walk
```

Keep profile-specific:

```text
MemOpKind
MemValueKind
MemLayoutContract
MemTableContract
VerifiedMemAccessPlan
FastMemory layout/table proofs
LLVM GEP/load/store lowering
memory-specific no-escape rules
```

Explicitly defer:

```text
generic AllowlistGate abstraction
generic RegionOp
generic VerifiedRegionAccessPlan
owner concept unification across allocator / page / language ownership
```

Rationale:

```text
Envelope is generic.
Payload is profile-specific.
Owner words may rhyme across layers, but their invariants do not.
```

## Parked Text-Scan Kernel Region Idea

This section records a future design candidate, not an accepted language
surface or an active implementation row.

```text
idea = contract-bound portable Text-scan kernel region
status = parked
current_profile_implementation_count = 1  # memory only
text_kernel_source_syntax = 0
text_kernel_profile_metadata = 0
text_kernel_plan_issuer = 0
text_kernel_backend_consumer = 0
text_kernel_production = 0
```

The motivation is to let advanced `.hako` code state a small, portable set of
low-level Text-scan operations inside a verifier-owned lexical region without
exposing raw pointers, backend instructions, or runtime lifetime mechanics.
The candidate is deliberately narrower than a general `ScalarKernelRegion`:
the first profile, if accepted, is Text scan only.

No spelling is selected yet. Forms such as `kernel TextScanV1 { ... }`, a
future `fastpath ProfileName { ... }`, or another one-keyword envelope remain
design alternatives. The current `fastmem ContractName { ... }` spelling is
memory-profile-only and must not be reused to imply that Text is raw memory.

### Authority and convergence

The explicit region may become a second producer of an already-defined
portable corridor plan. It must not become a second semantic or physical
authority.

```text
ordinary source
  -> source / Facts / Recipe
  -> automatic corridor issuer
                         \
                          -> one verified Value Corridor plan
                         /   -> one canonical physical consumer
explicit kernel region  /    -> profile-specific backend leaf emission
  -> source contract / ordinary body Facts / Recipe
  -> Text-kernel region issuer
```

The source-facing leaf laws must be documented independently of Rust enum,
MIR instruction, helper, and LLVM names. An existing MIR leaf may implement a
source law, but the existence or spelling of that leaf is not source
authority.

The explicit producer and automatic producer must share:

```text
ValueFamily / Carrier contract
root and provenance proof
consumer capability
use and escape census
publication and identity demand
effect / fault law
proof region and exit lifetime
target capability check
physical plan consumer
```

A hand-written kernel is useful demand, benchmark, and prioritization
evidence. It is not proof that an ordinary source expression may be replaced
by that kernel; automatic optimization retains its own source/Facts/Recipe
equivalence proof.

### Region character

This is a stricter checked region, not a C-style `unsafe` region. The initial
candidate obligations are:

```text
allowed source leaf vocabulary = closed
raw pointer / ptr-len source values = forbidden
allocation in the kernel body = forbidden
safepoint / await / nowait = forbidden
dynamic or Provider ABI dispatch = forbidden
Type ABI hot lookup = forbidden
kernel-local value escape or publication = forbidden
runtime fallback / retry = forbidden
unverified UTF-8 boundary or width = forbidden
missing normal-exit cleanup = forbidden
```

Unsupported operations or proof gaps are compile-time rejection. A backend
may implement the same portable law differently, but it must not rediscover
legality or silently select a slow compatibility helper. The first design row
must choose whether every public backend needs a semantic consumer or whether
an unsupported backend rejects the profile at compile time.

Region entry may acquire immutable Text roots and lifetime ownership once.
The steady-state loop must not acquire a root, lock a host table, allocate,
invoke a callback, validate a generation, or enter/finish Residence per
iteration. Normal and early exits must discharge the one region lifetime
obligation exactly once before physical return. Recoverable unwind remains
closed unless a later design supplies an explicit cleanup proof.

### MIRBuilder boundary

Do not repeat the transitional FastMem dedicated-lowerer split.

```text
ordinary parser / resolver / MIRBuilder:
  owns normal loop, if, local, and return meaning
  records region contract and source origin only

Text-kernel issuer / verifier:
  validates the profile allowlist and obligations
  issues the same corridor plan used by automatic optimization

physicalizer / backend:
  consumes the verified plan only
```

There is no kernel-specific CFG builder, loop builder, return builder, or
helper-name matcher. If a proposed leaf needs a new source semantic law, that
law is designed before the region admits it.

### Opening prerequisites

All of these must be observable before the first Text-kernel design row may be
selected:

```text
automatic S6C scalar-scan corridor is production-selected
its superseded physical route is retired with fallback/retry zero
the generic corridor contract has at least two real consumer shapes
non-application diagnostics name exact missing proof obligations
at least two independent source use cases need explicit checked-kernel control
portable source laws can be stated without raw pointer semantics
an external design review is scheduled for syntax, fault, ABI, and lifetime
```

Missing prerequisites keep this idea parked. They do not create a
`NoSafeSlice` in the active S6C lane and do not update `CURRENT_STATE.toml`.

### Parked task ladder

The names below reserve order only. None is selected by this document.

```text
TEXTKERNEL-CONTRACT-D0
  one external-reviewed decision for source spelling, Text-only leaf laws,
  effect/fault behavior, backend availability, region result/early-exit
  cleanup, and the exact shared corridor-plan boundary

TEXTKERNEL-SHADOW-I0
  parser/resolver/source-ledger metadata plus a report-only region issuer;
  ordinary body lowering stays canonical and backend behavior is unchanged

TEXTKERNEL-PLAN-I0
  co-seal the explicit producer into the same passive Value Corridor plan as
  the automatic producer; no raw pointer, runtime frame, or emission yet

TEXTKERNEL-VERIFY-I0
  enforce the closed leaf allowlist, no-escape/effect/fault/exit obligations,
  foreign-cohort rejection, and unsupported-backend rejection

TEXTKERNEL-LOWER-I0
  let the canonical physicalizer/backend consume only the verified shared
  plan; prove no helper-name or raw-MIR rediscovery and no fallback/retry

TEXTKERNEL-PROMOTE-R0
  publish reference docs only after semantic conformance, structural hot-loop
  gates, exact/meso/whole performance evidence, and production caller proof
```

`TEXTKERNEL-CONTRACT-D0` is the only docs-first decision row. If accepted, its
next row must produce code or a verifier-visible artifact; do not create a
chain of additional consultation documents.

### Promotion acceptance

The future promotion gate must include both semantic and structural evidence:

```text
one source meaning per leaf
one corridor plan shape below both producers
one canonical physical consumer
kernel body raw pointer count = 0
kernel-local value escape/publication count = 0
hot-loop lock/allocation/callback/runtime-call count = 0
backend legality rediscovery count = 0
fallback/retry count = 0
normal and early exit cleanup = exactly once
automatic replacement authority still comes from source/Facts/Recipe
```

Performance comparison is exact / meso / whole and uses the same target and
optimization level as the C reference. A numerical win cannot waive any
structural or semantic failure.

### Non-claims

This parked idea does not currently accept or activate:

```text
kernel / fastpath / ScalarKernel source syntax
new Text, Scalar, SIMD, IO, or generic RegionOp dialect
source-visible PinnedText root or access-plan ids
raw pointer, ptr-len, borrow, or unsafe syntax
Text-kernel VM, AOT, LLVM, or runtime behavior
automatic corridor eligibility from a hand-written kernel
performance guarantee or C-speed claim
S6C task-order change
CURRENT_STATE pointer change
```

## Non-Goals

```text
FastMemRegion mass rename
generic RegionOp
generic VerifiedRegionAccessPlan replacing memory plans
new fastpath / fastsimd / fastio parser behavior
new profile implementation
product allocator activation
Python-template C bridge retirement
```
