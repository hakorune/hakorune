# Rune Declaration Metadata

Status: current language reference.

`@rune` attaches declaration-local metadata to boxes, methods, functions,
constructors, interface methods, extern declarations, and other supported
declaration targets. Runes are not a second language and not a backend command
surface. They are source metadata transported into AST/MIR facts, where
verifiers, planners, diagnostics, and later backend-facing routes may consume
the derived facts.

## Basic Form

```hako
@rune Name
@rune Name(value)
@rune Name("string value")
```

Examples:

```hako
static box FastMath {
    @rune Inline(prefer)
    @rune Hint(hot)
    add1(x: i64): i64 {
        return x + 1
    }
}
```

```hako
static box AllocLeaf {
    @rune Inline(required)
    @rune Contract(no_alloc)
    @rune Contract(no_safepoint)
    fastPath(size: i64): i64 {
        return size + 16
    }
}
```

The parser gate for current rune metadata is:

```bash
NYASH_FEATURES=rune
```

Some bootstrap tools enable this gate for allocator proof apps. New examples
should show `@rune`, not the legacy annotation aliases.

## Canonical Families

| Family | Accepted values | Current meaning |
| --- | --- | --- |
| `Public` / `Internal` | no argument | Declaration visibility metadata. |
| `Ownership(...)` | `borrowed`, `owned`, `shared` | Ownership metadata. |
| `Inline(...)` | `prefer`, `avoid`, `required` | Canonical inline request family. |
| `Hint(...)` | `hot`, `cold` | Advisory tuning metadata. |
| `Contract(...)` | `pure`, `readonly`, `no_alloc`, `no_safepoint` | Verifier-backed or reserved contract metadata. |
| `IntrinsicCandidate("...")` | non-empty string | Candidate intrinsic replacement identity. |
| `Profile(...)` | reserved names such as `allocator.fast` | Authoring sugar that expands to primitive MIR facts. |
| ABI/export rows | `FfiSafe`, `Symbol("...")`, `CallConv("c")`, `ReturnsOwned`, `FreeWith("...")` | ABI-facing metadata where supported by the target declaration. |

`@rune Capability(...)` is not accepted parser surface yet. Capability facts
currently come from reserved profiles and metadata-only `uses ...` rows where
the relevant design card explicitly owns them.

## Inline Requests

Use `Inline(...)` for inline policy:

```hako
@rune Inline(prefer)    // best effort; unsupported shapes keep the call
@rune Inline(avoid)     // avoid soft inline
@rune Inline(required)  // fail fast unless verifier accepts the required shape
```

`Inline(required)` is not just an optimization hint. It requires both:

```hako
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
```

and the narrow leaf-inline verifier shape. If those checks fail, the compiler
must fail fast instead of silently keeping the call.

## Contracts

`Contract(no_alloc)` and `Contract(no_safepoint)` are live narrow verifier rows.
They populate MIR `effect_plans`; the verifier consumes those plans rather than
letting a backend infer behavior from raw rune strings.

Multiple distinct `Contract(...)` runes may appear on the same declaration:

```hako
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
```

Duplicate identical contract rows are rejected.

## Profiles

Profiles are convenience bundles:

```hako
@rune Profile(allocator.fast)
```

The profile name is not a backend contract. It expands to primitive MIR facts
such as `InlinePlan`, `EffectPlan`, and `CapabilityPlan`, and consumers must
read those facts. Backend route selection must not read profile names directly.

Reserved profile names are documented in:

```text
docs/reference/mir/rune-profile-registry.md
```

## Compatibility Spellings

These spellings remain accepted during the migration window:

```text
@hint(inline)              -> @rune Hint(inline) -> Inline(prefer)
@hint(noinline)            -> @rune Hint(noinline) -> Inline(avoid)
@contract(no_alloc)        -> @rune Contract(no_alloc)
@intrinsic_candidate("x")  -> @rune IntrinsicCandidate("x")
@rune Lowering(inline_required) -> Inline(required)
```

New code should use:

```hako
@rune Inline(prefer)
@rune Inline(avoid)
@rune Inline(required)
@rune Hint(hot)
@rune Hint(cold)
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
```

`Hint(inline)` and `Hint(noinline)` are compatibility spellings only. `Hint`
stays canonical for `hot` / `cold`.

## Placement Rules

Rune metadata is declaration-local. It is allowed only on supported declaration
targets. It is not allowed on ordinary statements or expression blocks.

General rule:

```hako
@rune Inline(prefer)
functionName(): i64 {
    return 1
}
```

Not allowed:

```hako
functionName(): i64 {
    @rune Inline(prefer) // invalid: body-position rune
    return 1
}
```

Target support is intentionally narrow. ABI/export rows such as `FfiSafe`,
`Symbol`, `CallConv`, `ReturnsOwned`, and `FreeWith` are not accepted on all
declaration targets. Parser diagnostics should fail fast with the supported
target list.

## Consumer Boundary

Runes follow this path:

```text
source @rune
  -> declaration-local attrs.runes
  -> MIR metadata refresh
  -> facts / contracts / plans
  -> verifier / optimizer / diagnostics / backend-facing routes
```

Backends must not rediscover legality from raw rune strings, helper names,
profile names, app names, or owner names. They consume already-derived MIR
route and plan facts.

## Related References

- `docs/reference/language/EBNF.md`
- `docs/reference/language/low-level-capabilities.md`
- `docs/reference/mir/hints.md`
- `docs/reference/mir/metadata-facts-ssot.md`
- `docs/reference/mir/rune-profile-registry.md`
- `docs/development/current/main/design/inline-plan-ssot.md`
