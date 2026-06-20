# 296x-1480 RUST-TO-HAKO-LIFECYCLE-EMITTER-SURFACE-001

Status: closed
Date: 2026-06-20

## Purpose

Select and render one existing verified lifecycle plan fixture into a bounded
`.hako` surface.

This row must not add direct Rust syntax ownership decisions.

## Selected By

```text
296x-1479-RUST-TO-HAKO-CONVERTER-TWO-INPUT-BOUNDARY-001
```

## Scope

```text
input:
  RustSubsetModule-v0 structure fixture
  verified HakoLifecyclePlan-v0 fixture

output:
  one bounded `.hako` lifecycle surface

allowed:
  fixture-only renderer/helper
  parser/MIR check for emitted surface

not allowed:
  rustc adapter integration
  converter-core rewrite
  backend lowering
  crate-wide lifecycle parity claim
```

## Acceptance

```text
one_verified_plan_fixture_selected=1
one_hako_surface_rendered=1
parser_or_mir_surface_check_green=1
direct_rust_syntax_ownership_decision=0
rustc_integration_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
one_verified_plan_fixture_selected=1
selected_fixture=carrier-info-merge-from-emitter-verifier-result-v0.json
one_hako_surface_rendered=1
surface=carrier-info-merge-from-emitter-surface-v0.hako
parser_or_mir_surface_check_green=1
direct_rust_syntax_ownership_decision=0
rustc_integration_started=0
backend_behavior_changed=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_emitter_probe_guard.sh
bash tools/checks/rust_lifecycle_emitter_surface_mir_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-emitter-probe-v0
emitter_probe_surface=green
verified_result_required=green
summary=ok

output_contract=rust-lifecycle-emitter-surface-mir-v0
surface_parse_or_mir_emit=green
summary=ok
```

Next:

```text
296x-1481-RUST-TO-HAKO-LIFECYCLE-PARITY-GATE-001
```

## Stop Line

```text
do_not_render_from_unverified_plan=1
do_not_choose_record_box_cleanup_in_emitter=1
do_not_claim_crate_wide_lifecycle_parity=1
do_not_change_backend_behavior=1
```
