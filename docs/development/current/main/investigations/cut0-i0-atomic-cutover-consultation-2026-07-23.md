# CUT0-I0 Atomic CUT0/G0 Consultation

Status: **Decision locked — Candidate SOURCE-FIRST-prime-r1 selected; RAW-SOURCE0-CONSULT0 next**
Date: 2026-07-23
Scope: decide the production authority boundary before wiring the single
atomic CUT0 executor.

Related:

- `docs/development/current/main/investigations/cut0-i0-prod-activation-consultation-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-prod-activation-execution-task-2026-07-23.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Worker census result

The disconnected canonical chain already exists:

```text
exact canonical plan
-> compiler token
-> physical session/shell/collector
-> lower/collect/complete/drain
-> route finalization
-> family postprocess
-> paired external commit
```

It is exercised only by test/disconnected callers. No
`execute_preflighted_module_invocation` exists yet.

Current non-test production surfaces are:

```text
compile_resolved* in src/mir/compiler/mod.rs
  -> old CanonicalModuleLoweringSessionV1
  -> old build_*_module_candidate / build_resolved_*_function_module
  -> old finish + session.commit

compile_with_source_internal
  -> live source-file hint mutation
  -> MirBuilder::build_module
  -> old finish_built_module

runtime/mirbuilder_emit.rs
  -> AST JSON -> MirBuilder::build_module

host_providers/mir_builder/lowering/ast_json.rs
  -> test-only direct AST JSON bridge
```

The major missing authority is Raw. Raw root/ledger completion has a strong
disconnected owner chain, but no production source-bound preflight/issuer
turns a legacy AST or Program(JSON v0) request into that chain. The P0-R1 Raw
fixture still uses a test-only issuer. Therefore an all-five-route atomic
claim would be false if CUT0 were wired now.

## Questions for decision

### Q1 — atomic executor scope

Which routes may the first production executor activate?

```text
1. All five families in one patch:
   canonical A+/trivial/acyclic/recursive plus Raw. This preserves the
   ACT-prime policy, but requires Raw source-bound ingress before any wiring.

2. Canonical four-route executor first:
   add a compiler-private executor for the exact canonical package and keep
   every public ingress disconnected until a later Raw-inclusive CUT0. This
   gives a partial production cutover and conflicts with the atomic all-route
   policy unless a new policy explicitly allows staged activation.

3. Keep all production consumers zero:
   implement only a disconnected outer executor/proof, then return to this
   boundary after Raw source binding is designed.
```

### Q2 — Raw source authority

How does a production legacy request enter the retained Raw owner chain?

```text
1. New Raw source-bound ingress:
   preflight the owned AST/Program(JSON v0), seal compatibility policy and
   source inventory, issue the compiler-owned token, then open the existing
   Raw collector/ledger/root chain. This is a new Raw semantic row, not a
   generic adapter.

2. Preserve the legacy builder path until a later Raw row:
   leave `MirBuilder::build_module` as a compatibility production owner and
   explicitly defer atomic CUT0. No false all-five activation claim.

3. Reuse the test-only Raw issuer/factory:
   rejected because it duplicates identity authority and cannot prove
   production source provenance.
```

### Q3 — AST JSON and Program(JSON v0) bridges

What owns `runtime/mirbuilder_emit.rs` and other direct JSON-to-MIR paths?

```text
1. Delegate to the same compiler-owned legacy executor:
   parse/resolve/merge at the bridge boundary, snapshot imports/source hints,
   and hand one sealed request to the outer executor. Direct Builder calls
   become zero.

2. Keep JSON emit as a compatibility lane outside CUT0:
   document a separate owner and do not claim all production Builder callers
   are retired. This requires a revised atomic policy.

3. Add a second JSON-specific executor:
   rejected unless its source authority and commit evidence are proven to be
   the same identity chain; otherwise it creates a second production owner.
```

Worker audit note: AST-JSON and Program(JSON v0) are not interchangeable
compatibility inputs. The AST-JSON bridge currently lowers through
`MirBuilder::build_module`, while `json_v0_bridge::lower_program` is an
independent ProgramV0-to-`MirModule` lowerer. Redirecting both to the same
executor without a parity decision would change postprocess/output semantics
and would create an unproven source converter. Q3=1 therefore requires an
explicit parity fixture and ownership decision; otherwise Program(JSON v0)
needs a separate `PROGRAM-V0-SOURCE0` design row or a documented compatibility
lane exception.

### Q4 — live Builder configuration

Where are imports, source-file hints, REPL mode, plugin signatures, and Core
ID seed captured?

```text
1. One sealed `BuilderInvocationConfigV1` at outer ingress:
   public arguments and existing Builder state are snapshotted before the
   candidate session; live Builder is unchanged until commit.

2. Keep wrapper prewrites and copy them into the candidate:
   rejected because it leaves failure-visible live mutation before the
   transaction starts.

3. Read environment/config inside each route:
   rejected as ambient authority and route drift.
```

### Q5 — activation evidence

What must be true before the one-shot CUT0 patch?

```text
1. Every non-test direct Builder caller is zero, including runtime JSON and
   host-provider bridges; one outer executor, one postprocessor, one finalizer,
   and one external commit terminal are the only production owners.

2. Canonical callers are zero but compatibility JSON callers may remain:
   weaker staged policy; requires a new explicit exception and retirement row.

3. Caller census is passive only:
   rejected because the atomic patch needs measured zero consumers, not names
   in a documentation table.
```

## Non-claims while stopped

```text
production outer executor = 0
Raw production source-bound ingress = 0
runtime AST-JSON direct Builder retirement = 0
all-five-route atomic CUT0 = 0
canonical four-route production cutover = 0
public wrapper retry/fallback = 0
```

## Required response

Select Q1–Q5 and define the next smallest executable row. The decision must
not wire canonical routes while claiming Raw is covered, must not convert a
test token into a production token, and must keep all current production
consumers disconnected until the selected all-route policy is implementable.

Until this consultation closes, do not add `execute_preflighted_module_invocation`
to public callers, remove `MirBuilder::build_module`, or alter runtime JSON
semantics.

## Worker recommendation (not yet a decision lock)

The current evidence supports the following provisional answer:

```text
Q1 = 3
  keep every production consumer disconnected until the Raw boundary exists

Q2 = 1
  design a production Raw source-bound ingress as its own semantic row

Q3 = split policy
  AST JSON may join the compiler-owned executor only after explicit parity;
  Program(JSON v0) remains the existing explicit compatibility lane until a
  separate PROGRAM-V0-SOURCE0 decision is closed

Q4 = 1
  seal BuilderInvocationConfigV1 once at the outer ingress

Q5 = 1
  require measured zero non-test direct Builder callers before CUT0
```

The next smallest row is therefore `RAW-SOURCE0-CONSULT0`, not executor
wiring. It must decide the source-bound request for legacy AST input, the
compiler-owned identity/token issuance point, the Raw source inventory and
compatibility policy, and how the existing `BuilderInvocationConfigV1` is
captured. It must not add a public executor, change the Program(JSON v0)
compatibility lane, or retire `MirBuilder::build_module`.

## SOURCE-FIRST-prime-r1 closeout

The consultation is closed with the following decisions:

```text
Q1 = 3 for the current boundary.
  Keep every production consumer disconnected. The only future production
  executor is an all-five-family atomic cutover; canonical partial activation
  is not permitted.

Q2 = 1.
  Add a compiler-owned Raw source-bound ingress as its own semantic row. The
  test issuer is never promoted and no generic family-selected Raw token API
  is introduced.

Q3 = split.
  AST JSON may join the Raw executor only after explicit parity. Program(JSON
  v0) remains the existing explicit compatibility lane until a separate
  PROGRAM-V0-SOURCE0 decision is closed.

Q4 = 1.
  Seal BuilderInvocationConfigV1 once at the outer ingress; live Builder
  configuration is unchanged until external commit.

Q5 = 1.
  Atomic CUT0 requires measured zero non-test direct Builder callers and one
  production owner for executor, finalizer, postprocessor, commit, token
  issuance, and MirCompileResult construction.
```

The next row is `RAW-SOURCE0-CONSULT0`. It is Builder-side design and proof
work only. It must not add an executor consumer, alter JSON semantics, or
retire `MirBuilder::build_module`.
