# Core Language Design Philosophy

Status: Historical
Scope: legacy explicit-delegation rationale kept as a redirect stub.

This page used to describe the old `from Parent` / override-era delegation
design as if it were the active language surface. The current Hakorune direction
is smaller:

```text
no inheritance
no extends / super
no implicit field merge
explicit field delegation only
```

Use these current references instead:

- `docs/reference/language/field-visibility-and-delegation.md`
- `docs/development/current/main/design/delegation-no-inheritance-ssot.md`
- `docs/development/current/main/design/language-minimal-surface-ssot.md`
- `docs/reference/language/stage-profiles.md`

Historical rationale remains available in git history. Do not copy old
`from Parent` examples into active docs or source.
