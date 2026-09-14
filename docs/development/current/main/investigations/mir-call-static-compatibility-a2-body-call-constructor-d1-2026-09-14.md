---
Status: design_closed__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-A2-BODY-CALL-CONSTRUCTOR-D1
Date: 2026-09-14
Parent: mir-call-static-compatibility-a2-body-call-constructor-d0-2026-09-14.md
NextCard: MIR-CALL-STATIC-COMPATIBILITY-I0-A2-BODY-CALL-CONSTRUCTOR-GUARDS
Implementation permission: false; owner inventory and guard boundary only
---

# A2 body, direct-call, and constructor handoff D1

## Six-line brief

```text
Decision: reuse the existing final-source -> resolver batch -> semantic-package chain; add no new semantic product.
Source authority + canonical issuer: parser final syntax loan owns callable bodies/generated rows; constructor catalog owns Birth rows; resolver index/canonicalization owns direct-call targets/headers.
Non-authority: AST/name/arity rescans, slot reconstruction, MIR/ValueId, compatibility fallback, publication, StringBox, and import lineage.
Fail-fast boundary: parser transform/loan, resolver batch, and package pre-install reject cardinality, owner/root, site/target/header, arity, constructor-parent, and generated-origin drift.
Smallest next slice: add focused positive/negative guards in the existing owners while preserving the current production caller.
Non-claims: no MixedProgram admission, nested-owner direct-call policy change, publication/cutover, fallback, Windows, or R7.
```

## Finite owner and terminal inventory

| order | owner and production caller | existing terminal | evidence scope |
| --- | --- | --- | --- |
| 1 | `parser/normal_callable_program_source/{semantic_syntax_loan,transform}.rs`, materialized by `runner/modes/common_util/normal_callable.rs` | `FinalCallableSemanticSyntaxLoanErrorV1` and `FinalCallableProgramSourceRejectV1` coverage, declaration, constructor, ordinary-box and root-preservation rejects | callable/body and constructor/slot drift |
| 2 | `mir/callable_semantic_batch/issuer.rs`, reached through the package free-static or App-Main batch caller | `ResolvedCallableSemanticBatchIssueV1` source coverage, resolver/deferred, root, body-shape, duplicate-owner, projection and unissued-observation rejects | body/forest co-seal |
| 3 | resolver canonicalization/index plus `normal_callable_semantic_package/direct_call_co_seal.rs` and App-Main relation helper | resolver callable/verification errors and existing package `UnissuedDirectCallObservation` / target-header rejects | source site, target, header, declaration identity and arity |
| 4 | `parser/constructor_source_catalog.rs` -> `normal_callable_semantic_package/instance_constructor_semantic.rs` | constructor catalog coverage/duplicate/changed/parent rejects and semantic source, root, body-shape, residual and completion rejects | ordinary constructor and Birth rows |
| 5 | parser generated callable anchors and constructor generated-Birth rows | generated-origin/provenance and callable coverage rejects | generated methods and generated Birth remain disjoint |

## Ordered implementation task

1. Add or tighten focused positive/negative tests in the parser owner for
   callable/body/constructor source drift, without issuing a new receipt.
2. Add resolver-batch evidence for exact body owner/root and source-site
   cardinality, reusing the existing `ResolvedCallableSemanticBatchIssueV1`
   terminals.
3. Exercise the existing direct-call co-seal with one source-index target and
   negative unissued, nested-owner, wrong-arity, missing-header and foreign
   cases. Nested-owner observations remain rejected by the current root-only
   index policy; changing that policy is outside this row.
4. Exercise constructor catalog and semantic rows with one Birth/no-Birth
   pair, foreign-parent and transformed-parent drift cases.
5. Keep generated callable provenance and generated-Birth provenance separate;
   no combined issuer is allowed.
6. Record focused results and the existing production caller in the I0 card;
   add no CI lane and no compatibility retry.

## Boundary and non-claims

The selected window is the existing parser final source through the package
pre-install terminal. It excludes MixedProgram source admission, nested-owner
direct-call policy changes, import-lineage validation, publication/caller
cutover, fallback retirement, StringBox readers, Windows proof and R7. Every
failure consumes the existing affine product at its named terminal.

## Worker audit receipt — 2026-09-14

The read-only audit confirmed the owner/terminal table above and found no
authority gap requiring a new product. The existing production caller remains
the normal root catalog lifecycle path. The smallest executable slice is
focused guard coverage through the existing owners; the audit made no code
edits and ran no Cargo command.
