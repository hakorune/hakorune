# CUT0-I0 P0-R1 Failure Matrix Consultation

Status: **Design stop — real-authority failure injection and rejected-owner policy undecided**
Date: 2026-07-23
Scope: decide the remaining P0-R1 failure evidence before any production
outer-executor or atomic CUT0 wiring.

Related task:

- `docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-prod-activation-consultation-2026-07-23.md`

## Evidence already closed

The disconnected real-authority chain is green for five success routes:

```text
Raw
CanonicalAPlus
BindingSsaTrivial
BindingSsaAcyclic
BindingSsaRecursive
```

The following commit-zero evidence is also green:

```text
Builder readiness failure
published-shell drain failure
foreign callable capability
canonical final-verifier failure
Raw reportable pre-transform verifier error
Raw root-batch late admission failure
```

All evidence remains test-only. Production capture, drain, finalizer,
postprocess, external commit, and public ingress consumers remain zero.

## Why the next code row is stopped

The remaining P0-R1 rows are not ordinary missing fixtures. They require a
failure authority that the current source-bound plans deliberately exclude.

### CHILD

The verified canonical plan promises draft-producing lowering. A malformed
plan or an arbitrary lowering error injected after source preflight would
break that contract. Existing child/session tests exercise typed primary,
cleanup, admission, and panic failures, but they do so through lower-level
test owners rather than the real canonical module owner.

Adding a generic `Fault` field or a hidden test-only branch to the production
plan would create a second failure authority and could be mistaken for a
production semantic path.

### PANIC

The disconnected all-route adapter already proves panic => no external
commit. The real canonical owner chain has no sanctioned panic injection
terminal. Wrapping the whole chain in `catch_unwind` is an outer policy
decision, not a local fixture helper.

### Rejected-owner retention

Some existing terminals retain rejected owners, but these do not all do so:

```text
complete_raw_root       -> maps collector rejection to a bare typed error
ModulePostprocessOwner  -> returns a bare stage error
Canonical finalizer     -> returns a bare validation error
```

The current P0-R1 acceptance can prove publication-zero without claiming
recovery or retry. It cannot honestly claim that every failure retains the
entire unpublished owner until this policy is decided.

### Typed postprocess failures

Canonical final-verifier failure is reproducible with an invalid CFG edge.
Optimizer diagnostics depend on global environment toggles, and contract/RC
failures need a deterministic invalid module-level fact. Introducing a broad
fault hook merely to manufacture these errors would enlarge the owner chain
before CUT0.

## Questions for decision

### Q1 — child failure authority

Which evidence is allowed for CHILD?

```text
1. Real-owner fault product:
   source-bound package carries one sealed test-only failure disposition;
   lower consumes it by value and returns the normal typed child error.

2. Existing lower-level owner proof only:
   keep real canonical plans infallible and cite the existing child/session
   matrix; P0-R1 claims module publication-zero only for failures observable
   at module terminals.

3. Lowerer contract split:
   make verified plans explicitly fallible at a new draft-failure boundary;
   this is a larger semantic design row before CUT0.
```

### Q2 — panic evidence

Which boundary owns panic-to-no-commit proof?

```text
1. MirCompiler outer executor catches unwind once and converts it to a
   typed invocation failure proof.

2. Keep panic evidence in the disconnected outer adapter only; do not claim
   real-authority panic injection in P0-R1.

3. Add route-specific panic terminals to every owner, which is rejected as
   duplicated failure authority unless a new design proves necessity.
```

### Q3 — rejected owner requirement

How strong is the failure product law for P0-R1?

```text
1. Every fallible terminal returns a rejected owner retaining the full
   unpublished chain. This requires new products for root batch, finalizer,
   and postprocess errors.

2. Publication-zero only: typed error plus structural absence of later
   terminals is sufficient; no retry/recovery claim is made.

3. Split the rows: close publication-zero now, then run a separate
   OWNER-RETENTION0 before production activation.
```

### Q4 — postprocess failure injection

How should optimizer/contract/RC failure be made deterministic?

```text
1. Use only naturally invalid MIR/module facts already expressible by the
   real route, with no new fault API.

2. Add a compiler-private, test-only postprocess fault product whose owner is
   sealed at the outer test ingress.

3. Retain the existing disconnected stage-order proofs and defer typed
   postprocess failure coverage to a dedicated POST-FAILURE0 row.
```

## Non-claims while stopped

```text
real-authority child failure matrix = 0
real-authority panic matrix = 0
full rejected-owner retention = 0
optimizer/contract/RC failure matrix = 0
production outer executor = 0
atomic CUT0/G0 = 0
```

The already-green success and bounded failure evidence must not be widened
into a full P0-R1 closeout. No production code or public ingress wiring may
be added until Q1-Q4 are decided.

## Required response

Select one candidate for Q1-Q4, identify which failure claims remain
disconnected-only, and define the smallest next executable row. The decision
must preserve the existing source-bound plan contract and must not introduce
silent fallback, retry, or an ambient environment failure switch.
