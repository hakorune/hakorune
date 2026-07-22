# CUT0-I0 ROOT0 design-stop brief

Status: **ROOT0-RAW0-D0 design stop after BRAND0**
Date: 2026-07-22
Decision: **ROOT0 R-prime selected; RAW0 scope is unresolved**
Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-t-prime-r1-execution-task-2026-07-22.md`
- `src/mir/builder/module_invocation_identity.rs`
- `src/mir/builder/module_invocation_owner_chain.rs`
- `src/mir/builder/module_lowering_invocation_state.rs`
- `src/mir/builder/module_invocation_drain.rs`
- `src/mir/builder/root_draft_batch.rs`

## Why execution stops here

The disconnected ID0/SESSION0 vocabulary is not yet the same physical owner
chain as the real root/collector products. ROOT0 cannot safely add a typed
completion wrapper until these three boundaries are selected:

```text
actual Builder session + actual shell + actual source/collector set
  -> one invocation brand

Raw root body + condition receipt + callable-main disposition
  -> one retained root witness

route-specific complete state
  -> private source-derived drain plan
  -> route-valid drained candidate
```

The current placeholder brand chain carries `()` payloads, while the actual
Builder session has no invocation brand. The raw receipt ledger also has its
own owner ordinal. Combining these without a decision would create multiple
identity authorities.

## Source authority

The following are the only future ROOT0 source authorities:

```text
Raw:
  SealedRawExpansionReceiptLedgerV1
  + retained CompletedRootBodyV1 / required condition receipt
  + RawCallableMainCompatibilityDispositionV1

Canonical single:
  VerifiedResolvedOwnerHeaderV1
  + exact collected owner row

Callable batch:
  VerifiedResolvedCallableModuleV1 / catalog
  + exact collected batch receipt
  + recursive shell capability when selected
```

`RouteOwnedInvocationInventoryV2` remains the policy SSOT for family,
inventory authority, root policy, and condition policy. It must project the
drain plan; callers must not supply symbols, `require_main: bool`, or
`ConditionFnPolicyV1::Optional`.

## Non-authority and forbidden shortcuts

The following remain disconnected compatibility vocabulary until ROOT0 is
resolved:

- `module_invocation_owner_chain.rs` placeholder `()` payloads;
- the independent raw-ledger owner ordinal;
- `ModuleLoweringInvocationStateV1::MainPending` for canonical routes;
- `InvocationDrainExpectationV1::new` caller inventory;
- `ModuleLoweringInvocationDrainOwnerV1::new(shell, collector)`;
- `DrainedModuleCandidateV1` unconditional `MissingMain`;
- `PreparedRootDraftBatchV1::prepare` with caller-selected condition policy;
- `MainRootWiringPlanV1::new(bool)` caller-selected compatibility;
- re-observing a completed root body after collector collection;
- fallback, retry, or canonical synthetic `main`/`condition_fn` insertion.

## Fail-fast boundary

Before any ROOT0 complete or drain product is issued, reject:

```text
foreign invocation brand
source family / route mismatch
missing or surplus physical receipt
raw root-body activity still open
missing required condition receipt
selected callable-main receipt absent
canonical synthetic main or condition_fn
caller-authored inventory or Optional condition policy
```

Every rejection leaves the unpublished collector prefix and live Builder
unchanged. No sibling descent, retry, drain, finalizer, or external commit
may follow.

## Candidate implementation slices

The following order is recommended, with production consumers remaining zero:

```text
ROOT0-BRAND0
  select one real session/shell/source/collector brand owner;
  retire placeholder and second-owner construction in the disconnected path

ROOT0-RAW0
  make root-batch collection retain CompletedRootBodyV1, exact condition
  receipt, and callable-main disposition in one Raw completion product

ROOT0-CANON0
  add exact-owner and callable-batch completion products, preserving the
  recursive capability marker and forbidding synthetic roots

ROOT0-DRAIN0
  derive a private drain plan from the route/source proof and quarantine
  caller inventory, `require_main`, Optional, and universal MissingMain

ROOT0-P0/G0
  exercise five families/nine rows, success/failure/foreign/panic cases, and
  guard production consumer count plus the below-800-line boundary
```

## Explicit non-claims

This brief does not claim that:

- the real production ingress is ready for CUT0;
- the existing placeholder brand is the final identity owner;
- raw root evidence can be reconstructed from current collector receipts;
- canonical `finalize_module` already has exact-owner/catalog parity;
- the old drain candidate may be generalized by adding more booleans.

The next executable owner is `ROOT0-BRAND0` only after the three boundary
decisions above are accepted. Until then, code implementation is paused.

## ROOT0-RAW0-D0 — scope boundary (design stop)

The next row has two possible scopes and must not be silently narrowed:

```text
Candidate A (recommended)
  collector-bound branded receipt provenance
  -> raw root-batch preflight
  -> retained CompletedRootBodyV1
  -> exact condition receipt + callable-main disposition
  -> one Raw completion witness

Candidate B
  ROOT0-RAW0-RECEIPT: branded receipt provenance only
  ROOT0-RAW0: root witness retention as a separate later row
```

The brief defines `ROOT0-RAW0` as Candidate A. Receipt provenance is a
prerequisite seam, not evidence that Candidate A is complete. No implementation
or SSOT closeout may claim receipt-only completion until this boundary is
explicitly locked. The receipt-only WIP is recoverable in
`stash@{0}: wip/ROOT0-RAW0 receipt provenance design scope`.
