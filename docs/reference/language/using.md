# using — Runner-side import contract

Status: Accepted
Scope: current implemented contract only.

`using` is a runner-side import mechanism. This page describes the current
contract only; it does not define future namespace systems or broader type-root
semantics.

## SSOT

- Manifest alias and module resolution live in `hako.toml`.
- Imported static-box alias binding lives in the runner text-merge strip path.
- Static receiver / type-name lowering lives in the MIR builder and applies only
  to `Alias.method(...)`.

## Resolution flow

```hako
using apps.std.string as S

static box Main {
  main() {
    return S.string_length("abc")
  }
}
```

1. **Manifest lookup**
   - The runner resolves `apps.std.string` through `hako.toml`.
   - In the current public sugar story, that manifest alias points to
     `apps/std/string.hako`.

2. **Runner text merge + alias binding**
   - The runner loads the target file, strips `using` lines, and merges the text
     before parse / execution.
   - The imported alias is then bound to an exported public static box name.
   - For `apps/std/string.hako`, the exported static box is `StdStringNy`, so
     `S` binds to `StdStringNy`.
   - If a target exports multiple static boxes, alias binding is accepted only
     when the alias already matches one exported box name exactly; otherwise the
     runner fails fast instead of inventing namespace behavior.

3. **MIR static-call lowering**
   - The MIR builder consumes the explicit alias-to-box binding for
     `Alias.method(...)`.
   - No namespace object or type-root object is created.

## Current guarantees

- `apps.std.string` is a manifest alias, not a semantic owner and not a type
  name.
- `apps/std/string.hako` is public sugar.
- `StdStringNy` is the exported static box used for the current sugar smoke.
- Imported aliases are valid for static calls such as `S.string_length(...)`
  after the runner merge step.

## Not supported by this contract

- `new Alias.BoxName()`
- `new apps.std.string.BoxName()`
- treating imported aliases as general namespace roots
- widening `using` into a broader package/type system on this page

## Notes

- Keep repository manifest edits in `hako.toml`.
- This contract is documentation-only: it describes the current runner + MIR
  behavior and does not introduce new resolution behavior.
