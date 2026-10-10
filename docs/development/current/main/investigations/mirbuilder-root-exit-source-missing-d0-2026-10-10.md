# MIRBUILDER-ROOT-EXIT-SOURCE-MISSING-D0

Status: D0 accepted; selected MIRBUILDER-STATIC-CALL-TERMINAL-RETENTION-S0
Date: 2026-10-10
Scope: first-stop census of the unchanged SizeClassBox published-view probe
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-borrowed-mul-static-entry-d0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_home.rs
  - src/mir/normal_callable_semantic_package/ordinary_new_normal_return_projection.rs

## Observed boundary

After the completed scalar incoming cohort and source-only Mul entry S0,
the unchanged `lang/src/hako_alloc/memory/size_class_box.hako` published-view
probe (with protected, uncommitted Eq WIP in a detached diagnostic checkout)
stops at `ordinary-new/local-commit/root-exit-source-missing`. The prior
`borrowed-mul/source-only-entry` first stop is no longer reached. This is a
red integration observation, not a published-view PASS. Probe log:
`/tmp/hako-static-scalar-eq-probe4.log`.

An env-gated diagnostic placed only at the refusal in the detached checkout
identified `FunctionOwnerIdV1 { compilation: 1, slot: 9 }`, exit
`[Body(0)]`: the original `SizeClassBox.bin_size_usize(bin: usize)` return
`me.bin_size(bin)`. The Completion root-flow exit row exists; the exact
terminal relation is absent; no Normal disposition is recorded. This is a
different owner from the earlier `accepts` Bool exits. Diagnostic log:
`/tmp/hako-root-exit-diag.log`. The temporary print and S0 overlay were
removed; the detached checkout again contains only its three protected Eq
WIP files.

`validate_root_home_exit` reaches this refusal only after object-return
construction readiness, Completion root flow and all-exits readiness.
`normal_exit_projection_v1(owner, expected_exit)` needs the exact terminal
relation. The source scanner can issue a `TerminalRelationV1::Call` for a
direct Static scalar return, but the child retention filter currently keeps
Call only when it carries a Lexical argument; this `bin_size` call has one
Scalar formal actual. The missing source relation, not the Home exit row, is
the immediate gap. Retaining it still would not prove physical return or
the later canonical scalar actual read.

## Decision

```text
Decision: retain the already source-issued exact Call terminal in a child
  only when its call site belongs to the original caller-keyed CurrentOwner
  Static source row with ExactI64 result. Keep the existing Lexical-call and
  Map paths unchanged. This is terminal relation retention, not a new call
  classification or physical result admission.
Source authority + canonical issuer: the Home verified walk issues
  TerminalRelationV1::Call from its exact Return/value site after the
  qualified Static source predicate. QualifiedStaticCallClaimIndexV1 owns
  the original caller key, call-site source and ExactI64 result; the child
  coseal filter retains that relation only after exact key/site matching.
  Completion and Home root flow remain the independent exit authority.
Non-authority: owner-wide `has_current_owner_i64_source`, the freeze string,
  function name, zero-argument call shape, emitted ValueId or all-caller
  inference. None may authorize a different Call site.
Fail-fast boundary: missing or mismatched caller key/site/result keeps the
  Call relation unretained. The existing unqualified zero-argument child
  Call remains rejected. Exact terminal owner/return site and later
  physical packet/return checks remain independent.
Smallest next slice: MIRBUILDER-STATIC-CALL-TERMINAL-RETENTION-S0, switch
  the child coseal retention for the original bin_size_usize/1 Body(0)
  return, then re-run the unchanged published-view probe to the next named
  boundary. Pin one positive exact source and one negative unqualified or
  mismatched-site case in the existing terminal family.
Non-claims: no whole SizeClassBox publication, no Bool physical result,
  no Eq physical completion, no mimalloc-lite EXE PASS.
```

The read-only worker traced the current filter and ruled out treating this as
the earlier `accepts` Bool stop. The detached diagnostic confirmed its
different owner and missing relation. The existing filter test already pins
the unqualified zero-argument Call refusal; reuse it rather than adding a
separate guard. Do not broaden retention to every Call.

No Bool physical result, whole SizeClassBox publication, or mimalloc-lite EXE
acceptance is claimed by the preceding S0.

## S0 closeout (2026-10-10)

The child coseal filter now retains its Home-issued direct Call terminal only
for the original caller-keyed CurrentOwner Static source row at that exact
call site with ExactI64 result. It neither creates a new terminal nor admits
an arbitrary Call. The existing unqualified zero-argument Call refusal stays
in the filter family.

At the primary source revision based on `d73dc97b21`, the focused Static
scalar-return terminal test passed 1/1, the existing unqualified Call negative
passed 1/1, static-source family passed 15/15, previous scalar cohort test
passed 1/1, and Normal projection family passed 9/9. Build/test log:
`/tmp/hako-static-call-terminal-s0.log`; quick test binary build 7m47s,
focused execution 0.01s. The initial zero-match Normal projection filter was
not counted; the corrected `normal_projection_` filter passed 9/9.

The unchanged `size_class_box.hako` published-view probe, with protected Eq
WIP in the detached diagnostic checkout, moved from
`root-exit-source-missing` to `ordinary-new/local-commit/call-entry-missing`.
It remains a red integration observation, not a published-view PASS. Log:
`/tmp/hako-static-call-terminal-integration.log`; quick build 7m42s, test
execution 0.01s. The diagnostic overlay was removed afterward, leaving only
the three protected Eq WIP files. No `.hako` source was changed. The next
first stop belongs to executable root Call entry and gets a separate D0.
