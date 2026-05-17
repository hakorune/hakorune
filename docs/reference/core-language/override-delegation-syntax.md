# Override And Delegation Syntax

Status: Historical
Scope: legacy override / `from Parent` design page kept as a redirect stub.

The old page described `override` and `from Parent.method()` as active
delegation syntax. That is no longer the canonical Hakorune direction.

Current direction:

```text
no inheritance
no `extends`
no `super`
no `override` surface for new code
no `from Parent.method()` surface for new code
explicit field delegation via `delegate field exposes { ... }`
```

Use these current references instead:

- `docs/reference/language/field-visibility-and-delegation.md`
- `docs/development/current/main/design/delegation-no-inheritance-ssot.md`
- `docs/development/current/main/design/language-minimal-surface-ssot.md`
- `docs/reference/language/stage-profiles.md`

Historical details remain in git history. New examples must use the current
reference surface.
