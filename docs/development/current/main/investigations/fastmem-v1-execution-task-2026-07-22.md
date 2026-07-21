---
Status: Parked execution board with a reserved activation handoff
Decision: task order selected; production activation remains forbidden
Date: 2026-07-22
Scope: FastMem V1 foundation, first ny-llvmc FieldLoad vertical, and retirement dependencies
Current-lane effect: none; D-prime HEADERPORT0 remains authoritative
Reserved activation: after `MODULE-FINALIZE-VERIFY-CUT0`
First active row: `FASTMEM-BASELINE0`
Parent:
  - docs/development/current/main/investigations/fastmem-v1-contracted-borrow-design-2026-07-20.md
Related:
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# FastMem V1 Execution Task Board

## Outcome

The contracted-borrow design is already accepted.  A three-worker read-only
audit found no remaining semantic choice that requires another external
consultation before task planning.

FastMem is now reserved as the next dedicated lane after
`MODULE-FINALIZE-VERIFY-CUT0`. The handoff is still conditional: the current
HEADERPORT cutover, FACTSESSION activation, producer closures, module
transaction integration, module guard, and final verification cut must all be
green. At that point `CURRENT_STATE.toml` switches the active lane, and the
sole first row is:

```text
FASTMEM-BASELINE0
```

It removes one test-only nondeterministic block-iteration expectation before
any FastMem source or contract is touched.  It is followed by one SSOT sync,
one vocabulary freeze, and the backend/target foundation.
The first executable feature remains one exact TableIndex plus scalar
FieldLoad on daily non-replay ny-llvmc.  Existing llvmlite success is reference
evidence only.

This board does not change the current D-prime blocker and does not authorize
FastMem production work before the reserved handoff.

## Scheduled handoff dependency

```text
WIRING-I0-HDR0-M0/P0/G0
-> CUT0-COMPAT-POLICY-CONSULT0
-> WIRING-I0-CUT0-S0/P0/I0/G0
-> FACTSESSION0-ACTIVEBIND0-S0/P0
-> FACTSESSION0-I0/G0
-> REMATFACT0 / individual producer receipt closures
-> FINALIZE0-PHI-SPLIT0-MODULETX0-P0
-> FINALIZE0-PHI-SPLIT0-I0
-> FINALIZE0-PHI-SPLIT0-MODULE-G0
-> MODULE-FINALIZE-VERIFY-CUT0
-> FASTMEM-BASELINE0
```

The reservation prevents starvation without permitting parallel mutation of
the same MirBuilder/finalization surfaces. No FastMem foundation row may be
pulled before the handoff merely because its disconnected vocabulary could be
built earlier.

## Current truth

```text
V0 source/lowering:
  implemented
  ambient active-region selection
  17 MemOp kinds
  nine mem.assume* fact injectors

V0 transport:
  MIR JSON implemented
  access identity = block + instruction index

V0 llvmlite:
  TableIndex/FieldLoad implementation exists
  compatibility/reference only

V1 capability/anchor/value/site/plan:
  production owners = 0

ny-llvmc non-replay MemOp producer:
  0

Rust MirInterpreter MemOp execution:
  0
```

Known V0 facts must not be promoted into V1 authority:

```text
generic value MemOp result:
  MirType::Integer compatibility

FieldLoad result:
  declared type or post-success Integer compatibility

access-plan identity:
  BasicBlockId + instruction_index

layout authority:
  host usize::BITS in surviving paths

raw route selection:
  current_fastmem_region ambient state
```

## Macro order

```text
FASTMEM-BASELINE0
  -> FASTMEM-SSOT-DRIFT0
  -> FASTMEM-VOCAB-FREEZE0
  -> FASTMEM-BACKEND-ID0
  -> FASTMEM-BACKEND-PREFLIGHT0
  -> FASTMEM-TARGET0
  -> FASTMEM-CONTRACT0
  -> FASTMEM-FOUNDATION0
  -> FASTMEM-V1-PARSE0
  -> FASTMEM-FIELDLOAD-VERTICAL0
  -> later family rows
  -> retirement DAG
```

Only one working card is opened when the lane resumes.  The names below are
sections of that card unless a slice changes a durable semantic owner.

## 0. `FASTMEM-BASELINE0`

Root reruns found one pre-existing nondeterministic test expectation:

```text
fixture:
  mir::builder::fastmem::tests::branch::
  fastmem_source_lowers_owner_eq_branch_cfg_pilot

cause:
  MirFunction.blocks is a HashMap
  the fixture flattens blocks.values() and asserts cross-block instruction order

observed:
  FieldLoad(owner_worker_id), FieldStore(used)
  or
  FieldStore(used), FieldLoad(owner_worker_id)
```

The test already proves one occurrence of each MemOp kind.  BASELINE0 changes
only the pair assertion to an order-independent exact set comparison.  It must
not sort or mutate production MIR, declare HashMap iteration semantic, or alter
Builder block allocation.

Acceptance:

```text
isolated fixture repeated >= 10 times = green
FastMem Rust tests = 93/93
production source delta = 0
grammar/MIR/runtime/backend delta = 0
```

## 1. `FASTMEM-SSOT-DRIFT0`

One atomic BoxShape/truth-sync commit.  Do not split it into four docs-only
cards.

### Exact corrections

```text
parser truth:
  src/parser/statements/fastmem.rs must stop claiming parse-only
  syntax/AST transport remains parser-owned
  existing V0 lowering remains unchanged

backend label truth:
  FastMemBackend::LlvmNative is a legacy aggregate/self-test classifier
  it proves neither llvmlite nor daily ny-llvmc completion
  enum/support-table redesign remains BACKEND-ID0

MemValueKind truth:
  landed Rust owner count = 0
  current V0 facts are Integer compatibility plus partial MemOpKind escape checks
  docs must mark MemValueKind as future vocabulary, not landed authority

current route truth:
  fastmem.rs is the thin region shell
  shared Builder descent plus ambient region selection is current V0
  dedicated-lowerer historical text must not describe the current route
```

### Allowed delta

```text
comments/docs only
grammar/AST/JSON delta = 0
MemOp enum/support delta = 0
Builder/lowering delta = 0
V1 producers/consumers = 0
```

### Acceptance

```text
stale parser "parse-only" claim = 0
Rust MemValueKind definitions = 0
LlvmNative production callers outside fastmem_ops.rs = 0
ambient V0 selection remains explicitly documented
```

No new one-off shell guard is added.  Use card-local static checks and the
existing FastMem gates.

## 2. `FASTMEM-VOCAB-FREEZE0`

Behavior-neutral semantic classification.  It may use a short refactor series,
but the whole series changes no source acceptance, MIR wire shape, backend
support, or emitted fact.

### `S0` — baseline matrix

Seal exactly 17 MemOps and nine legacy assume intrinsics from source.

```text
permanent primitive = 3
  TableIndex FieldLoad FieldStore

proof/observation = 2
  CurrentAllocOwnerId OwnerEq

verified PageMap recipe = 7
  LocalFreePush LocalFreePop FreeHeadPush FreeHeadPop
  AtomicRemoteHeadPush AtomicRemoteHeadDrain DrainRemoteListToLocal

migration-only = 5
  AddrOf LogicalShr BitAnd Add Sub
```

Legacy fact injectors:

```text
assumeTableLength/2
assumeIndexInRange/2
assumeSameOwner/2
assumeRemoteOwner/1
assumeLocalFreeBlockNext/1
assumeFreeHeadBlockNext/1
assumeRemoteFreeBlockNext/1
assumeLocalFreeNonEmpty/1
assumeFreeHeadNonEmpty/1
```

The tool-only spellings `mem.load`, `mem.store`, `mem.atomicCas`,
`mem.atomicExchange`, and `mem.atomicFetchAdd` are `ReservedClosed`.  They are
not current `MemOpKind` or Builder vocabulary.

### `M0` — MemOp classifier

Add one exhaustive no-wildcard `FastMemMemOpClassV1` classifier under a small
`src/mir/contracts/fastmem_ops/` child module.

```text
every MemOpKind::ALL row classified exactly once
new enum variant without classification fails compilation/test
old display/json/arity/effect/backend tables unchanged
old backend subsets remain projections, not semantic class authority
```

Do not place this policy in `instruction.rs`.

### `A0` — assume registry

Add one typed `LegacyFastMemAssumeKindV0` registry in a small
`src/mir/builder/fastmem/` child module.  It owns exact source name, arity,
fact family, and `PageMapV1 = Forbidden`.  The existing function/method lookup
consumes it without changing error order or emitted facts.

### `P0/G0` — proof and guard

```text
17 rows = 3 + 2 + 7 + 5
nine assume rows are unique
unknown/near-miss calls retain existing forbidden-call failure
function and method routes retain pre-argument failure timing
backend support matrices unchanged
Hako formatter/classifier pilots remain non-authority
Python lowerer map remains backend implementation, not vocabulary SSOT
```

New reusable checks belong in a small manifest-driven
`tools/checks/lib/fastmem_v1/` family.  Do not extend the existing 947-line
`fastmem_capability_inventory_common.py`; if it must change, split it first as
an independent behavior-neutral BoxShape commit.

## 3. Backend safety foundation

### `FASTMEM-BACKEND-ID0`

Replace the broad completion claim with explicit identities:

```text
MirJsonTransport
LlvmLiteKeep
NyLlvmcMainline
RustMirInterpreter
HakoMirInterpreter
CArtifact
```

This row changes identity/reporting only.  Support matrices and executable
behavior remain unchanged.

### `FASTMEM-BACKEND-PREFLIGHT0`

Register the stable diagnostic first, then make the selected Rust interpreter
reject every executable V0/V1 MemOp-bearing module before instruction effects:

```text
[fastmem/backend-unsupported] backend=mir-interpreter
```

The rejection is unconditional for that selected backend.  It is not gated by
dev/strict/planner flags and never attempts host-pointer execution.  The future
Hako interpreter remains parked and will use arena id plus checked offset, not
host raw pointers.

## 4. Representation foundation

### `FASTMEM-TARGET0`

Seal target triple, pointer width, endianness, alignment rules, and contract
version into one target/layout fingerprint.  Synthetic 32/64-bit mismatch
fixtures must reject rather than inherit host `usize::BITS`.

### `FASTMEM-CONTRACT0`

Add one compiler/stdlib-owned sealed `PageMapV1` registry.  Source-defined
contract declarations remain parked.

### `FASTMEM-FOUNDATION0`

Build disconnected products in this exact order:

```text
ANCHOR0-S0
  opaque pinned anchor and foreign-anchor errors

BIND0-S0
  exact anchor/table/length role set and mandatory compile-time alias

VALUECLASS0-S0
  RawTableView/LayoutRef/RawAddress/Scalar/OwnerId/ProofToken
  exact session/anchor/region/target brands

SITE0-S0
  stable access-site id and Reserved -> Pending -> Completed|Aborted typestate

REGION0-S0
  CFG members/exits, pinning/lifetime, and no-escape closure

PROVE0-S0
  static proof schema only
```

Production producers and consumers remain zero throughout these S0 products.

## 5. `FASTMEM-V1-PARSE0`

Record the required `docs/reference/**` language decision, then change the Rust
parser, selfhost parser, AST/Program JSON, and parity fixtures together.

```text
fastmem PageMapV1(anchor=..., table=..., length=...) as alias { ... }
```

No V1 lowering consumer is connected in this row.  Missing/duplicate roles,
missing alias, effectful role expressions, nested region, and V0 fallback all
reject.

## 6. `FASTMEM-FIELDLOAD-VERTICAL0`

Open only the first exact slice:

```text
ANCHOR0-I0
-> BIND0-I0
-> VALUECLASS0-I0
-> PROVE0-LITERAL0
-> SITE0-I0
-> ACCESSPLAN0-S0
-> MIRJSON0
-> FIELDLOAD0
-> REGION0-I0
-> ACCESSPLAN0-I0
-> NYLLVMC-FIELDLOAD0
-> FIELDLOAD0-EXE0
-> PROVE0-RANGE0
-> FIELDLOAD0-PERF0
```

### Physical slice

```text
one controlled/FFI pinned anchor
one exact PageMapV1 role set
one capability alias
one proven table index
one TableIndex -> LayoutRef<PageMeta>
one FieldLoad<PageMeta.used, usize>
one proven-lossless Hakorune Integer projection
```

TableIndex/LayoutRef must not publish ordinary `MirType::Integer` as its
representation authority.  Failed emission leaves site, fact, region, and plan
delta zero.

### Transport and backend gates

MIR JSON preserves one stable access-site id, target/contract fingerprint, and
one producer-sealed plan for every surviving physical site.  The plan is not
first-published by completed-MIR scanning or semantic refresh.

ny-llvmc must implement a genuine non-replay MemOp producer.  Acceptance
requires:

```text
owner = boundary
recipe = pure-first
compat_replay = none
Python child/harness path = 0
TableIndex/FieldLoad helper call = 0
```

llvmlite layout-ref lowering is reference/parity evidence only.

### Executable and performance gates

```text
generated executable result = C fixture result
table-slot calculation = 1
page-pointer load = 1
field scalar load = 1
helper call = 0
Box dispatch/boxing = 0
runtime contract/string-field lookup = 0
per-access runtime bounds check = 0
controlled median target <= matching C + 3 percent
```

Exact instruction shape is the primary keeper gate.  Wall time alone does not
select a keeper.

## Later families

After the first vertical is fully green, open exactly one family at a time:

```text
FASTMEM-FIELDSTORE0
FASTMEM-OWNER0
FASTMEM-FREELIST0
FASTMEM-REMOTE0
```

Later, independently:

```text
HMI-X0-FASTMEM-FIELDLOAD0
FASTMEM-NESTED0
FASTMEM-CONTRACTDECL0
FASTMEM-REQUIRE0
trusted-assume support
```

## Retirement DAG

Retirement is dependency-driven:

```text
AMBIENT-RET0
  after V0 syntax callers are zero or independently proven disconnected

ARITH-RET0
  after migration-only MemOps have ordinary-MIR/branded-role replacements

ASSUME-RET0
  after PROVE/OWNER/FREELIST/REMOTE and all V0 assume callers are zero

OWNEROP-RET0
  after branded OwnerId plus ordinary Compare owns the surviving law

RECIPE-TRANSPORT-RET0
  after FREELIST and REMOTE own their recipes

V0-SYNTAX-RET0
  after source/parser/JSON/backend callers are zero

ACCESSPLAN-REFRESH-RET0
  after every surviving family has producer-sealed SITE/ACCESSPLAN coverage

CHECK-RET0
  after small manifest guards cover the same inventory
```

The first FieldLoad vertical cannot retire plan refresh for FieldStore, owner,
free-list, or remote families.

## Baseline and row gates

Run the light baseline before and after every foundation row:

```bash
cargo test -q fastmem --lib
bash tools/smokes/v2/profiles/integration/parser/fastmem_parser_parity_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

Current audited baseline before `FASTMEM-BASELINE0`:

```text
FastMem Rust tests = 92/93 on failing hash seed
isolated branch fixture = nondeterministic pass/fail across reruns
parser parity smoke = PASS
capability inventory smoke = PASS
current-state pointer guard = PASS
```

Every touched source/check file must remain below 800 lines.  Existing oversized
files are not extended.

## Stop conditions

Stop the active FastMem row if any step requires:

1. changing the current D-prime blocker without explicit lane activation;
2. accepting a new MemOp or source intrinsic during foundation work;
3. treating a Hako parity pilot, llvmlite, or Python map as semantic authority;
4. using ambient region state as V1 provenance;
5. publishing TableIndex/LayoutRef as ordinary Integer authority;
6. using host `usize::BITS` as V1 target proof;
7. deriving a V1 access plan from final MIR or `(block, instruction_index)`;
8. retrying V1 failure through PageMapV0 or another backend;
9. adding Rust-interpreter raw-pointer execution;
10. enabling FieldStore, owner, free-list, remote, nested, require, or trusted
    assume in the first FieldLoad vertical;
11. extending the oversized legacy inventory checker instead of splitting or
    adding a small manifest-driven V1 owner;
12. touching a source/check file at 800 lines or more.

## First-vertical claim boundary

After `FASTMEM-FIELDLOAD-VERTICAL0` is green, implementation may claim only:

```text
one explicit branded PageMapV1 capability selects one raw table root
unrelated region expressions retain ordinary semantics
TableIndex yields an opaque LayoutRef rather than Integer representation truth
one proved scalar FieldLoad executes on daily non-replay ny-llvmc
Rust VM rejects before execution
raw capabilities do not escape
backend lowering consumes target-specific producer-sealed plans
```

It must not claim general unsafe blocks, raw pointers, runtime checks, trusted
assumptions, FieldStore/recipe/atomic completion, general user-defined
contracts, interpreter parity, V0 retirement, or allocator migration.
