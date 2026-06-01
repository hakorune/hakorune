# Build Conditional `gate`

Status: Provisional reference
Decision: accepted-for-implementation-slices
Date: 2026-06-01

## Purpose

Hakorune uses AST-level build conditionals instead of C-style token
preprocessing.

Accepted direction:

```hako
gate Build.test {
    import "HakoTest"

    test resetToFresh_smoke() {
        ...
    }
} else {
    import "HakoReleaseHooks"
}
```

Rejected direction:

```text
#if / #else / #endif
#define
token paste
conditional partial syntax
```

`gate` is a build-time selector. It is not a runtime `if`, not a macro, and
not a Rune metadata row.

`gate` is a parser-contextual keyword. It is recognized only at build
conditional item heads, so existing ordinary identifiers named `gate` remain
valid in expressions and local bindings.

## Meaning

```text
if:
  runtime value branch

gate:
  build configuration branch
```

Inactive `gate` branches:

- are parsed
- may be formatted and inspected by tooling
- are not resolved
- are not typechecked
- do not produce MIR
- do not reach lowering

Syntax errors in inactive branches are still errors. Missing imports, missing
types, or backend-only names in inactive branches are not errors.

## Predicate Surface

`gate` predicates are a small build-configuration DSL. They are not ordinary
`.hako` expressions.

Accepted v0 predicate atoms:

```text
Build.test
Build.debug
Build.release
Feature("name")
Target.os == linux
Target.arch == x86_64
Backend.kind == c
```

Accepted combinators:

```text
not(...)
all(...)
any(...)
```

Policy:

- runtime values are not readable from `gate`
- function calls are not allowed, except the built-in predicate form
  `Feature("name")`
- environment variables are not read directly from source
- unknown features are compile errors, not silent false

Default parser configuration:

```text
Build.release=1
Build.test=0
Build.debug=0
Target.os=current host OS
Target.arch=current host arch
Backend.kind=unknown
known_features=[]
enabled_features=[]
```

Tooling/CLI integration may pass an explicit build config. Source files do not
read environment variables directly.

## Slice Order

### LANG-CFG-001: item/import level

Owner: parser + build-cfg prune before resolution.

Scope:

```text
program item
import / using
box declaration
function declaration
test declaration when present
```

Acceptance:

```text
inactive_branch_mir_count=0
inactive_branch_lowering_count=0
unknown_feature_is_error=1
inactive_import_not_resolved=1
```

### LANG-CFG-002: member level

Scope:

```text
box member declarations
method declarations
stored fields
```

Constraint:

Public ABI/layout changes behind `gate` are rejected by default unless the
branch pair preserves the same public signature. In v0, member-level `gate`
is only accepted inside box bodies for declaration members, and paired
branches must match on declaration surface before the selected branch is
merged into the box layout.

Example:

```hako
box ChoiceBox {
    gate Build.test {
        value: i64
        choose() { return 1 }
    } else {
        value: i64
        choose() { return 2 }
    }
}
```

The branch bodies above are allowed because their declaration signatures are
identical. Bodies may differ; field/method surface may not.

### LANG-CFG-005: statement level

Scope:

```text
method body statement blocks
```

Use case:

```hako
resetToFresh() {
    ...

    gate Build.test {
        me.test_reset_count = me.test_reset_count + 1
    }
}
```

Statement-level `gate` is intentionally later than item/member level because
it touches MIR construction and control-flow shape.

### LANG-CFG-006: optional Rune sugar

`@rune Gate(...)` is accepted as single-declaration sugar only on top-level
declarations:

```hako
@rune Gate(Build.test)
test resetToFresh_smoke() {
    ...
}
```

It desugars to:

```hako
gate Build.test {
    test resetToFresh_smoke() {
        ...
    }
}
```

`gate` remains the semantic owner. This sugar is parser-local and does not
become a stored rune metadata family. It is intentionally not accepted on box
members or ordinary statements.

## Style Rules

- Prefer separate test modules for large test-only code.
- Use `gate Build.test { ... }` for small grouped test-only declarations.
- Do not put test-only fields in allocator hot-core boxes unless a dedicated
  layout row accepts the public/private layout change.
- Do not use `gate` to hide production behavior changes inside hot paths.
- Do not use `gate` as an optimization knob. Fast paths must still be selected
  by facts/plans/proofs, not by source branch names.
- Observer counters may use `gate` only after their role is classified:
  proof/test payloads may move behind `Build.test`, diagnostic-only payloads may
  move behind a declared feature predicate, and public semantics must remain
  available in production builds. In particular, `gate` must not silently remove
  a public stats accessor or change a hot-core box layout.

## Explain Report

Build conditional pruning should expose a compact report:

```text
output_contract=hakorune-build-cfg-explain-v0
build_mode=test
target_os=linux
target_arch=x86_64
backend=c

conditional_group_count=...
active_branch_count=...
inactive_branch_count=...
inactive_branch_mir_count=0
summary=ok
```
