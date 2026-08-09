# Dynamic Invocation Contract Owner

This directory owns the atomic, selector-independent semantic execution
envelope for exact source-bound Dynamic member targets.

```text
VerifiedSourceCallTargetCatalogV1
  -> VerifiedDynamicInvocationEnvelopeCatalogV1
```

The envelope catalog owns the complete route-neutral target catalog. It does
not copy Dynamic keys into a second authoritative map: each Dynamic arm has
exactly one borrow-scoped envelope view, while Static arms remain retained and
unselected. Missing and duplicate envelope rows are therefore impossible by
construction after successful issue.

The language-wide envelope is fixed by
`docs/reference/language/dynamic-invocation.md`:

```text
OpaqueObservable
SynchronousNonDetached
MaySuspend
CallableBounded Normal(SelfContainedDynamicCarrier) | Fault
BorrowedNoEscapeForInvocation inputs
EndExactlyOnceUnlessForwarded result lifecycle
```

Every normal result carries one opaque `EndExactlyOnceUnlessForwarded`
obligation. This is not a Home claim: runtime payload kind selects only the
physical end mechanism, while source-visible Home classification remains a
separate resolver/Home Flow authority.

This owner must not import or infer from Recipe, Builder mutation, MIR effects,
providers, ABI, runtime tags, executable plans, or fallback routes. It issues
no public partial-axis receipt and performs no selector-specific refinement.

If a valid unchanged source row is outside the current compiler boundary,
widen the compiler or stop at an explicit design question. Do not narrow the
fixture or add a method-name exception.
