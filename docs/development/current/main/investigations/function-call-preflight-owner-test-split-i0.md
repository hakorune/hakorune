# FunctionCall Preflight Owner Test Split I0

Status: landed
Scope: behavior-neutral extraction of inline preflight tests  
Parent: `function-call-direct-vs-value-call-compat-census-d0.md`  
Row: `FUNCTION-CALL-PREFLIGHT-OWNER-TEST-SPLIT-I0`

## Execution brief

Decision: Extract the inline `function_call_preflight_route` test module into one
sibling test file without changing production code, visibility, or behavior.
Source authority + canonical issuer: The existing production preflight owner and
its exact test module remain unchanged in meaning; Rust module inclusion owns the
one test location.
Non-authority: Line count, file placement, test names, green output, and the split
do not issue call semantics, target identity, or migration permission.
Fail-fast boundary: Production bytes/source outside the module declaration,
test inventory, names, and assertions must remain mechanically identical.
Smallest next slice: Replace the inline module with one `#[path]` child and run the
focused library tests plus current pointer and reusable MirBuilder guard.
Non-claims: No parser/resolver/Builder behavior, receipt, Recipe, Script activation,
compat retirement, diagnostic change, production switch, fallback, or retry.

## Acceptance

- `function_call_preflight_route.rs` falls below 760 lines.
- The child test file remains below 760 lines and uses the parent module's private
  surface without widening production visibility.
- The exact prior test function-name inventory is unchanged.
- Focused preflight tests and the reusable MirBuilder in-place guard are green.
- `git diff --check` and current pointer guard are green.
- After closeout, return to a resolver-owned lexical callee classification D0.

## Closeout evidence

```text
production owner LOC: 790 -> 329
test child LOC:       443
focused tests:        5 passed
test-name inventory:  unchanged
rustfmt/diff check:   green
current pointer:      green
```

`mirbuilder_inplace_replacement_guard.sh` remains red at
`selected normal lifecycle caller must be exactly one`; the exact same failure
reproduces at parent `ada2373fb2`, so it is classified as known baseline debt
and not attributed to this behavior-neutral split.
