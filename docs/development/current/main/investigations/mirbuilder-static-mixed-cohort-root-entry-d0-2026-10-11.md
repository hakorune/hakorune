# MIRBUILDER-STATIC-MIXED-COHORT-ROOT-ENTRY-D0

Status: D0 accepted; S0 package checkpoint verified, physical acceptance open
Date: 2026-10-11
Scope: unchanged `SizeClassBox.size_to_bin_usize/1` Static Return
Predecessor: `MIRBUILDER-STATIC-SCALAR-FORMAL-ROOT-ENTRY-S0`

## Physical frontier

The preceding S0 validated the root I64 Invoke and normal result for
`good_size_usize/1` in the unchanged SizeClassBox published-view probe.
The probe remains red at `size_to_bin_usize/1` Body(0) with
`call-entry-missing`: the Static source is not selected. This is the next
first stop, not a whole-source or EXE success claim. The probe used the
protected Eq WIP in a disposable worktree; the production checkout did
not absorb that WIP.

The original source has three calls into `size_to_bin(size)`: one from
`size_to_bin_usize(size: usize)`, one from `good_size(size)`, and one from
`accepts(size)`. The first appears to stage a Scalar(Integer) actual from
an exact `USIZE` formal; the latter two appear to forward opaque formals.
The source spelling alone did not establish the candidate classification.

The focused package census on the unchanged source now confirms exactly
three rows for target owner slot 5, all CurrentOwner Static and source-only
(`definitions=false`, `target_static=false`). The target formal is the same
opaque binding in each row, with `checked_static_input=true`; all three
original sources have an empty required-I64-arguments list. Every pending
actual is `SourceStatic` with integer evidence and ordered `BorrowedActual`
Home. The wrapper in slot 6 has `Scalar(binding,Integer)` from its declared
`ExactTrivial(USIZE)` formal. `good_size` slot 7 and `accepts` slot 10 have
`SelfRooted` candidates from their declared `OpaqueHandle` formals.
The diagnostic test passed 1/1 in a disposable detached worktree; it is
not a permanent guard or an executable admission. Its log is
`/tmp/hako-mixed-cohort-census-20261011.log`.

The read-only worker found the current boundaries:

- `static_incoming_cohort_v1` over `source_incoming` is the canonical
  whole-callee inventory. `SourceStatic` staged actuals hold per-row input
  candidates.
- The scalar finisher requires the entire cohort to be Scalar(Integer)
  rooted in exact `USIZE` formals. It therefore correctly leaves this
  mixed target unselected.
- The forwarded `StaticOneInput` finisher is bounded to one or two rows;
  three rows do not enter. The packet lender and route preflight have
  matching homogeneous checks. Broadening only route selection would
  leave packet lending unproved.

## Decision

Join this exact complete target cohort through the existing Scalar and
Forwarded evidence issuers, with a common target
Completion, result/publication, signature and ordered Home check. Issue one
Static packet only when all original rows complete. Recheck the whole
cohort at demand-time lending and retain the independent final MIR verifier.
Do not allow one successful caller to grant target entry or weaken the
existing homogeneous paths.

Source authority and canonical issuer: `source_incoming` and
`static_incoming_cohort_v1` own the whole original caller inventory;
`SourceStatic` owns per-row actual candidates. Use the existing exact
scalar-formal checks for slot 6 and
`issue_original_static_forwarded_actual_v1` for slots 7 and 10. The
existing Completion/result/signature and ordered Home issuers remain the
only publication authority. The Static packet/root Call owner performs
physical emission. The new mixed join is a composition at the existing
actual finisher and packet lender, not a new caller census or ABI.

Failure boundary: absent, extra, drifted or rejected cohort row; incorrect
candidate/formal/source identity; missing checked forward input, Completion,
result, signature or ordered Home must refuse before route selection and
again at demand-time lending. Unselected SourceStatic stays passive.

Production caller and replacement: `size_to_bin_usize/1` Body(0) is the
observed first stop. The mixed target cohort also includes `good_size/1`
and `accepts/1`, so the same bounded slice must audit all three. Replace
only the source-only mixed-candidate dead end with the existing Static
packet. No Plain route, .hako rewrite or homogeneous-path fallback.

S0 acceptance: original-source positive and focused negative coverage for
an absent/drifted mixed sibling; unchanged-source physical root Call with
a later exact first stop; no source rewrite or parallel authority. Reuse
existing family tests and guard. The focused positive and physical result
are completion evidence, not construction prerequisites. Whole-source
published view and mimalloc-lite EXE remain separate obligations.

## S0 evidence in progress

The original-source package test now selects all three original calls into
`size_to_bin` and obtains one packet actual per call: one Scalar(Integer),
two Forwarded. Removing the `good_size` sibling actual after issuance makes
packet lending refuse the wrapper call. The focused test passed 1/1 and
the existing `static_packet_entry::tests` family passed 12/12 under the
quick profile. The first implementation attempt wrongly demanded mixed
source identity from unrelated passive three-call owners; moving that
check behind exact mixed-proof presence resolved the regression.

The unchanged-source published-view probe in the disposable worktree with
the protected Eq WIP copied in advanced past the former
`size_to_bin_usize/1` `call-entry-missing` first stop. It is still red:
`SizeClassBox.accepts/1` now stops at
`[ordinary-new/borrowed-entry/entry-values-missing]` before the Bool return
boundary (`/tmp/hako-static-mixed-physical-20261011.log`). This is a new
caller-entry obligation, not evidence that the whole published view works.
The probe uses a diagnostic snapshot of S0; final-revision physical evidence
remains open. The compiler fails while building `accepts`'s lexical body,
before the published physical ABI callback, so this movement alone does not
prove that `size_to_bin_usize` emitted its root Call. The missing-sibling
negative passed at packet lending; a route-selection recheck was added and
its first focused run was red: after one sibling was removed, route
re-selection returned `Ok` instead of refusing. The selected mixed-cohort
checker treated the missing pending row as passive. It now treats a selected
cohort with a missing sibling or changed source authority as a contract
error. The corrected focused run passed 1/1 and the reused Static packet
family passed 12/12 (`/tmp/hako-mixed-route-negative-fixed-20261011.log`).
This was a change-induced red, not baseline debt. The Eq-overlay probe
repeated the `accepts` first stop after the final pre-negative S0 source
copy (`/tmp/hako-static-mixed-physical-final-20261011.log`, quick build/link
7m45s); the strict selected-row fix has not had a new physical probe.
Do not close S0 on the package tests.

## Exposed dependency: BOOL-CURRENTOWNER-ENTRY-D0

Read-only audit found one incoming `accepts_usize(size: usize) ->
me.accepts(size)` CurrentOwner call. Its result is `ExactBool`, while the
existing static entry seed accepts exact I64 or a narrow I64 local initializer.
The mixed `size_to_bin` packet proves the forwarded call's *actual*, but
cannot issue the `accepts` caller's entry values. The borrowed-entry issuer
needs a source-backed entry for `accepts` before its body can use `size`.

Decision for the next bounded design row: join the original incoming row,
`ExactBool` result disposition, declared `USIZE` formal, target opaque formal,
Completion and physical signature at the existing borrowed-entry issuer.
Reject a wrong site, target, formal, ABI, result, or missing incoming row.
Do not reinterpret Bool as I64 or infer entry from the mixed packet. Keep
Bool physical return/Invoke as a separate obligation. The next acceptance
is an original-source entry receipt and movement to the next named Bool
boundary, plus focused wrong-identity and missing-entry refusal. This is
not yet an implementation authorization for a new Bool route; select its
own row/card before construction.
