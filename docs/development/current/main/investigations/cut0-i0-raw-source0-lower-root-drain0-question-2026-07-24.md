# RAW-SOURCE0 LOWER ROOT0 — DRAIN0 設計相談

Status: **Open design stop — ROOTBATCH0-S0 complete; Raw DRAIN0 not selected**  
Date: 2026-07-24  
Related: `cut0-i0-raw-source0-lower-root-rootbatch0-s0-execution-task-2026-07-24.md`

ROOTBATCH0 now produces a route-specific `RawRootBatchCompleteInvocationV1`
that owns the Builder session, shell, collector, sealed Raw ledger, completed
root witness, helper receipts, and callable-Main evidence.  It deliberately
does not publish a module.  The next boundary is a Raw-specific one-shot drain;
the older canonical DRAIN0 card is not an authority for this route.

## Evidence already closed

```text
BODY0 -> unpublished main draft + CompletedRootBodyV1
ROOTBATCH0 -> collector Main/condition admissions + sealed Raw ledger
ROOTBATCH0 shell published functions = 0
ROOTBATCH0 ledger open reservations = 0
RawRootBatchCompleteInvocationV1 production consumer = 0
public ingress / finalization / postprocess / external commit = 0
```

The Raw collector may contain, in one exact invocation order:

```text
pre-root static helper drafts
callable-Main compatibility draft (Selected only)
physical root Main draft
required condition_fn draft
```

The sealed ledger is the provenance witness for those admissions.  A drain
implementation must not reconstruct this inventory from `current_module`, a
new AST scan, caller-authored symbol vectors, or the old Main-only drain
candidate.

## Questions to decide

### Q1 — sole drain owner

Where is the only consuming entry from
`RawRootBatchCompleteInvocationV1`?

1. **Compiler-owned `prepare_drain(self)`** that derives the exact Raw
   inventory from retained ledger/collector evidence and calls one Builder
   sibling physical terminal.
2. Builder-owned source-aware drain that reacquires source authority.
3. Reuse `ModuleLoweringInvocationDrainOwnerV1` or
   `DrainedModuleCandidateV1`.

Candidate 2 risks duplicating compiler authority; candidate 3 accepts
caller-authored inventory and legacy synthetic-root policy.

### Q2 — exact inventory authority

Should the drain manifest be projected from:

1. **The sealed Raw ledger plus collector keyed drafts**, with one neutral
   exact row manifest retaining role, key, symbol, arity, policy, and receipt
   provenance.
2. The source continuation/catalog, requiring a second physical lookup.
3. Caller-provided symbols/`require_main`/condition policy.

The selected design must define whether helper order is ledger ordinal order,
collector key order, or a source-derived schedule, and must prove set parity,
duplicate rejection, and callable-Main disposition without re-observation.

### Q3 — physical unpack and publication

What one Builder sibling terminal consumes the branded shell/collector and
sealed ledger?

It must preflight shell state, collector indexes, ledger event parity, brands,
and exact row correspondence before mutation.  Decide whether the drained
physical product owns an unpublished `MirModule`, a sealed module shell, or a
different opaque module carrier.  Loose `(shell, collector, ledger)` tuples
are not allowed across the compiler boundary.

### Q4 — output product

Should the new product be:

1. **`RawDrainedInvocationV1::{Script, App}`**, retaining token/session,
   source continuation/runtime snapshot, exact drained physical carrier,
   sealed ledger witness, helper receipts, and callable-Main outcome.
2. An adapted `RawCompleteInvocationV1`.
3. A bare `MirModule` or `MirCompileResult`.

The product must retain enough evidence for a later Raw finalization row while
preventing a second drain or a source re-resolution.

### Q5 — failure and one-shot law

Should every inventory/brand/index/shell mismatch return a rejected complete
owner before shell mutation, followed by an infallible drain?  The candidate
law is:

```text
fallible prepare_drain(self)
-> RejectedRawDrainInvocationV1 { exact complete owner, stage, nested cause }
or PreparedRawDrainV1
-> infallible drain(self)
-> RawDrainedInvocationV1
```

No retry, fallback, second drain, partial module publication, or typed panic
claim is permitted.  Decide whether collector/ledger evidence remains in the
drained product or is converted into one non-Clone witness.

### Q6 — lifetime and re-observation

Which source-bound facts survive drain?

1. **Only the already retained continuation/runtime snapshot and exact
   manifest witness; no AST/catalog reacquisition.**
2. Reborrow the original source/catalog during finalization.
3. Reconstruct inventory from the drained module.

The selected option must preserve Raw callable-Main Selected/NotSelected
evidence and must not let physical symbols choose route policy.

### Q7 — route and root semantics

Does Raw DRAIN0 keep Script and App as separate products while treating
`main`/`condition_fn` as the already committed root pair?  The new row must
state whether helper/callable drafts remain separately addressable, whether
the module shell is still unpublished until finalization, and whether the
old canonical synthetic-root assumptions are explicitly rejected.

## Non-claims before this consultation closes

```text
Raw physical drain / module publication = 0
Raw finalization / postprocess / external commit = 0
public compiler ingress / JSON bridge = 0
source or catalog re-resolution after ROOTBATCH0 = 0
retry / fallback / second drain = 0
canonical DRAIN0 adapter = 0
production consumers / CUT0 activation = 0
```

## Required answer shape

```text
Q1 sole drain owner
Q2 exact inventory authority and ordering
Q3 physical unpack/publication boundary
Q4 drained product shape and retained evidence
Q5 failure/one-shot law
Q6 source lifetime and re-observation law
Q7 Script/App/root semantics
```

Do not implement Raw DRAIN0 until these questions are selected and a new
execution task fixes the smallest executable slice and its guard.
