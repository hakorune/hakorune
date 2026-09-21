---
Status: closed__2026-09-21__QualifiedMethodRecipeAdapterCallerZero
Task: MIRBUILDER-QUALIFIED-METHOD-RECIPE-ADAPTER-DELETE-D0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-source-residual-guard-i0-2026-09-21.md
Implementation permission: false; caller-zero census and deletion contract only
NextCard: MIRBUILDER-QUALIFIED-METHOD-RECIPE-ADAPTER-DELETE-I0
---

# Qualified method recipe adapter deletion D0

## Six-line brief

```text
Decision: evaluate removal of the unused qualified-method recipe adapter
  implementation after the LoopBreak residual guard; keep the selected Main
  adapter and the trait contract intact.
Source authority + canonical issuer: the existing
  QualifiedMethodRecipePortV1 trait and the two in-tree implementations.
Non-authority: method names, type existence, a trait implementation by itself,
  publication=None, or a warning-free compile as caller evidence.
Fail-fast boundary: deletion is allowed only after a finite caller-zero census
  proves the normal-package adapter implementation has no production/test
  entrypoint and the remaining Main implementation still owns its call.
Smallest next slice: prove the two-implementation/one-caller inventory, then
  delete only the caller-zero impl with a compile and structural guard.
Non-claims: no qualified-call semantic expansion, publication ABI change,
  Main route change, fallback repair, or old LegacyCallV0 retirement.
```

## Current finite inventory

`QualifiedMethodRecipePortV1` has two implementations in the current tree:

| implementation | observed entry | disposition |
| --- | --- | --- |
| `MainQualifiedMethodRecipePort` | `main_root.rs` constructs it and passes it to `lower_resolved_trivial_body_with_qualified_method_port_v1` | retained selected owner |
| `NormalCallableSemanticPackagePortAdapterV1` | implementation exists in `normal_callable_semantic_loan_port.rs`; no call to the qualified-port method or lowerer passes this adapter | caller-zero candidate |

The trait has one lowering consumer, the canonical trivial SSA lowerer. The
next D0 must prove the adapter implementation is not reached through a trait
object, generic forwarding helper, test-only constructor, or re-exported
entrypoint. The implementation's ignored `site` and always-`None` publication
are design clues, not caller-zero evidence.

Census boundary: trait declaration -> all implementations -> all calls to the
qualified-port method and lowerer; includes production and `cfg(test)` code;
excludes unrelated Raw child ports and the selected Main implementation.

## Required guard and deletion tuple

Before implementation, record exact `rg`/symbol census results and identify a
stable structural guard that fails if the deleted impl or a second caller
returns. The I0 deletion may remove only the adapter impl block; it must not
change the trait, lowerer, Main adapter, publication handoff, or argument-site
checks. Compile failure or a newly discovered caller reopens this D0.


## Caller-zero census and accepted decision

The finite census was re-run on the current tree with:

```text
rg -n "QualifiedMethodRecipePortV1|impl .*QualifiedMethodRecipePortV1" src/mir/builder
rg -n "lower_resolved_trivial_body_with_qualified_method_port_v1|new_with_qualified_method_port|take_qualified_method_recipe_v1" src
```

It found exactly two implementations and one lowerer entrypoint. The only
construction and call of `new_with_qualified_method_port` is in
`normal_callable_semantic_loan_port/main_root.rs:224-232`, where
`MainQualifiedMethodRecipePort` is passed. The normal-package adapter is used
by other Raw/Root ports, but no call passes it to the qualified-method
lowerer, and no other trait-object or generic forwarding entry was found.
The `site` discard and `None` publication are therefore inside a caller-zero
implementation, not a live selected path.

Decision: the adapter impl block at
`src/mir/builder/normal_callable_semantic_loan_port.rs:348-390` is an accepted
caller-zero delete-set. The trait, lowerer, Main adapter, publication handoff,
and argument-site checks remain shared live assets. The next I0 may remove only
that impl and imports used solely by it, then prove the remaining one
implementation and lowerer caller with a structural guard.
