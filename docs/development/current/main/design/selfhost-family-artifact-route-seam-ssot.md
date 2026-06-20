# Selfhost Family Artifact Route Seam SSOT

Status: accepted
Date: 2026-06-20

## Purpose

Define the minimal route seam for selecting compiler-family implementations
during the Derived-to-Native Hako artifact migration.

This seam exists so generated `.hako` artifacts can become explicit execution
artifacts without becoming edit authority, without deleting Rust bootstrap,
and without runtime try-Hako-then-Rust fallback.

## Vocabulary

Allowed route labels:

```text
derived_hako
native_hako
rust_bootstrap
rust_compat
host_substrate
unsupported
```

Definitions:

```text
derived_hako:
  checked-in generated `.hako` artifact with provenance manifest.

native_hako:
  hand-owned `.hako` source adopted as edit/semantic authority.

rust_bootstrap:
  retained Rust implementation used for bootstrap/platform bring-up.

rust_compat:
  explicit compatibility/oracle route, not silent fallback.

host_substrate:
  host/runtime/backend substrate that is intentionally not compiler meaning.

unsupported:
  no route selected; must carry a reason.
```

## Selection Contract

Selecting a family route requires:

```text
family_id
route label
state
manifest path when route=derived_hako
guard command
rust_bootstrap retained proof
fallback policy
```

`derived_hako` selection additionally requires:

```text
artifact manifest verified
generated artifact manual edit = 0
deterministic regeneration evidence
selected family scope
```

## Non-Selection Contract

If a family is not selected, the route manifest must report:

```text
selected_on_mainline=false
not_selected_reason=<stable token>
fallback_policy=forbidden
```

The reason is diagnostic evidence. It must not trigger runtime fallback.

## Fallback Policy

Runtime fallback is forbidden:

```text
try generated Hako
  failed
  silently run Rust
```

is not allowed.

Allowed alternatives:

```text
selected route succeeds
selected route fails fast
explicit rust_bootstrap route is selected before execution
explicit rust_compat/oracle route is selected before execution
unsupported route reports a stable reason
```

## Authority Boundary

Generated `.hako` is an execution artifact only.

```text
generated artifact:
  selected implementation artifact
  manual edit forbidden

native Hako:
  edit/semantic authority only after HakoAdopted decision

Rust:
  bootstrap/platform/oracle/compat retained
```

Do not read semantic-authority demotion as Rust bootstrap removal.

## Stop Lines

```text
do_not_select_any_generated_artifact_without_manifest=1
do_not_select_any_generated_artifact_without_guard=1
do_not_runtime_fallback_from_Hako_to_Rust=1
do_not_make_generated_Hako_edit_authority=1
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost_from_derived_artifact=1
```

## First Consumer

The first intended consumer is:

```text
hakorune_mir_builder::binding_context
```

The first implementation row may update the BindingContext route manifest, but
must stay family-scoped and must not select VariableContext or MirBuilder-wide
routes.
