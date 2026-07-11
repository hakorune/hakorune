# P381GO Global Call Override Visibility Cleanup

Date: 2026-05-06
Scope: shrink the Stage0 global-call lowering override API surface.

## Context

`cargo check --bin hakorune` reported two `private_interfaces` warnings because
public `GlobalCallRoute` builder methods accepted the route-plan-internal
`GlobalCallLoweringOverride` type.

The lowering override is not a public route-plan API. It is owned inside
`src/mir/global_call_route_plan` and currently used only by the Stage1
emit-program runtime-helper route classification.

## Change

- Removed the unused exact `with_lowering_override(...)` builder.
- Narrowed `with_optional_lowering_override(...)` to `pub(super)`.

## Result

The route-plan public surface no longer exposes an internal lowering override
type, and `cargo check --bin hakorune` drops the two `private_interfaces`
warnings without changing route metadata or lowering behavior.

Observed warning count:

```text
before: 43 warnings
after:  41 warnings
```

## Validation

```bash
cargo check --bin hakorune
rg -n "private_interfaces|with_lowering_override|GlobalCallLoweringOverride" \
  /tmp/hakorune_p381go_cargo_check2.log || true
git diff --check
```
