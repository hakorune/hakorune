# Callable Loop Production Source/Facts Bridge D0

Status: design stop after `CALLABLE-LOOP-PRODUCTION-ADMISSION-D0` closed as
`NoSafeSlice`.

Decision: design one production bridge from resolver-backed callable
source/facts authority to an AST-free, owner-branded Loop relation. This row
is docs-only. It must not expose a physicalizer, selector, retry, fallback,
Generic G0 substitution, or legacy route.

## Problem boundary

The production host is known:

```text
NormalCallableSemanticLoanPortV1::lower_normal_top_level_function
```

It owns outer callable orchestration, but its current loan exposes only
lineage and lowering state. The callable single-loop Recipe, source map,
Prelude/Tail relation, and operation/effect ledger are currently issued by
`cfg(test)` products. Removing the test gate or copying a fixture would create
a second semantic authority and is forbidden.

## Sole source authority

The bridge may consume only existing resolver/source/facts products:

```text
VerifiedNormalCallableSemanticSourceV1
VerifiedNormalCallableSemanticLoanV1
ResolvedFunctionLoweringInputV1
resolver-issued callable header/index and source lineage
```

It may not re-walk AST nodes, recover names from route labels, infer Recipe
keys from raw MIR, or read a legacy manifest/scheduler. If the existing
products cannot issue one exact relation, the bridge returns typed
`NoSafeSlice` and does not open a function session.

## Target product

The bridge must issue one move-only, AST-free product whose only new fact is
the exact relation between already verified capabilities:

```text
VerifiedCallableLoopSourceFactsBridgeV1 {
  owner / compilation brand / source frame
  resolver source/facts lineage
  Loop membership and logical Recipe/JoinSig relation
  operation/input source relations
  Prelude/Tail source relation
  exact Scope/Region relation
}
```

This is a source/facts relation, not a Recipe replacement and not a physical
plan. It must not contain `ValueId`, `BasicBlockId`, Builder references,
Completion consumption, DraftSeal, runtime route names, or module state.

The later prepared adapter consumes this bridge exactly once together with
the existing exact ABI and `VerifiedFunctionCompletionV1` to issue
`PreparedCallableLoopPhysicalizationV1`. The bridge itself does not issue
ABI or Completion authority.

## Required correspondence table

The design must map each production input to exactly one output receipt:

| Existing authority | Bridge receipt | Reject when |
| --- | --- | --- |
| callable owner/brand/frame | same owner/brand/frame | foreign or missing lineage |
| resolver callable header/index | exact callable target/source site | ambiguous or route-derived |
| source loop membership | logical Loop key/frame/Scope/Region | missing/duplicate site |
| source operation facts | item-keyed operation/effect relation | missing/foreign/duplicate item |
| input/Prelude facts | exact preheader source relation | no producer or mismatched BindingRef |
| terminal/Tail facts | exact terminal source BindingRef | confused with Loop After |
| source Recipe/JoinSig evidence | co-sealed logical relation | second owner or re-derived key |

The table is complete only when every output field has one named issuer and
every issuer has one production source. A partial table is a design failure,
not an invitation to add a fallback.

## Failure and transaction boundary

```text
bridge preflight:
  no Builder/session effect
  typed NoSafeSlice on missing/foreign/incomplete evidence

prepared ingress:
  open one fresh CanonicalFunctionLoweringSessionV1
  move Completion into CanonicalSsaFunctionSessionV2 exactly once

physical execution:
  Prelude -> common Loop -> After -> Tail
  finish_for_draft_seal -> DraftSeal prepare/commit

failure after session open:
  discard_unpublished
  restore caller once
  no same-session retry/fallback
```

The bridge is not allowed to publish a collector/module draft. The existing
function session, DraftSeal, collector, and module transaction remain their
sole owners.

## Required evidence before implementation

1. positive production source/facts fixture with exact owner/brand/frame;
2. missing, duplicate, foreign, and cross-brand rejection fixtures;
3. source-to-Recipe/JoinSig membership totality receipt;
4. Prelude and terminal Tail relation receipts, with Loop After kept separate;
5. caller-zero bridge construction with no Builder effects;
6. fresh-session parity evidence after later physical preparation.

## Non-claims

```text
production bridge implementation = 0
PreparedCallableLoopPhysicalizationV1 production issuer = 0
physical Loop emission = 0
I0 caller switch = 0
Generic G0 parity = 0
retry/fallback/legacy deletion = 0
```

Implementation, when authorized, must update the affected source README,
`docs/reference/**`, diagnostics, migration note, guards, and current pointer
in the same commit. Reference synchronization is an acceptance condition.
