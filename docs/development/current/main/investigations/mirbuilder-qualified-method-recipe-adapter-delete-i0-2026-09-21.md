---
Status: closed__2026-09-21__QualifiedMethodRecipeAdapterDelete
Task: MIRBUILDER-QUALIFIED-METHOD-RECIPE-ADAPTER-DELETE-I0
Date: 2026-09-21
Parent: mirbuilder-qualified-method-recipe-adapter-delete-d0-2026-09-21.md
Implementation permission: true for the caller-zero adapter impl and imports used only by it
NextCard: none__after_closeout
---

# Qualified method recipe adapter deletion I0

## Six-line brief

```text
Decision: delete the caller-zero QualifiedMethodRecipePortV1 implementation
  on NormalCallableSemanticPackagePortAdapterV1 after the finite D0 census.
Source authority + canonical issuer: the existing trait and the selected
  MainQualifiedMethodRecipePort implementation.
Non-authority: the adapter type's other Raw/Root port impls, method names,
  publication=None, or a warning count without the caller census.
Fail-fast boundary: structural guard and quick compile reject any second
  implementation, lowerer caller, or accidental Main/trait change.
Smallest next slice: remove one impl block plus imports used only by it, run
  the guard and focused compile, then record the exact zero-caller evidence.
Non-claims: no qualified-call expansion, publication ABI change, fallback,
  production route switch, or LegacyCallV0 retirement.
```

## Delete-set

Delete only the `QualifiedMethodRecipePortV1` impl for
`NormalCallableSemanticPackagePortAdapterV1` in
`src/mir/builder/normal_callable_semantic_loan_port.rs`. Remove imports that
become unused only because that block disappears:
`SourceExprSiteV1`, `CanonicalSameModuleCallableKeyV1`, and
`QualifiedStaticMethodHandoffPortV1` if the post-edit census confirms they have
no other use in the file. Keep the adapter type and every other port impl;
they still have live callers.

The retained selected owner is `MainQualifiedMethodRecipePort` in
`normal_callable_semantic_loan_port/main_root.rs`, and the sole lowerer call
remains the existing Main-root call. No trait signature or publication value
may change.

## Structural guard

Add `tools/checks/lib/mir_call_qualified_recipe_adapter_delete_guard.py` in this
slice. It must read the two owner files and fail unless:

- exactly one `impl ... QualifiedMethodRecipePortV1` remains, on
  `MainQualifiedMethodRecipePort`;
- `NormalCallableSemanticPackagePortAdapterV1` has no qualified-port impl;
- the lowerer entrypoint still has exactly one repository caller, in
  `main_root.rs`.

The guard is structural evidence for this private delete-set, not a claim that
external users of the trait are impossible. A new in-tree caller reopens the
D0 instead of being removed to manufacture caller-zero.

## Acceptance

Run the guard, `cargo fmt --all -- --check`, and
`cargo check --profile quick --lib -j4` sequentially. Record the exact
post-edit `rg` census and the check result here. The expected result is one
remaining qualified-port implementation, one lowerer caller, and exit 0.


## Closeout evidence

The caller-zero adapter impl was removed without changing the trait, the
selected `MainQualifiedMethodRecipePort`, the lowerer, publication handling,
or any other port on `NormalCallableSemanticPackagePortAdapterV1`. The shared
`SourceExprSiteV1` import was retained because `ordinary_new.rs` imports the
parent module's names; it is not part of the deleted block.

| check | result | evidence |
| --- | ---: | --- |
| qualified recipe delete guard | **pass**: one Main impl, one lowerer caller | `python3 tools/checks/lib/mir_call_qualified_recipe_adapter_delete_guard.py` |
| `cargo fmt --all -- --check` | **exit 0** | local closeout run |
| `cargo check --profile quick --lib -j4` | **exit 0; 1,756 warnings** | `/tmp/hakorune-qualified-recipe-adapter-delete-check-20260921.log` |
| post-edit caller census | **one implementation / one caller** | `rg` census recorded in parent D0 and guard output |

The first post-edit check caught the shared `SourceExprSiteV1` import and
failed before acceptance; restoring that import produced the green result
above. No supported caller was removed to manufacture caller-zero.
