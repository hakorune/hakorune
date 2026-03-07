---
Status: SSOT
Scope: FlowBox tag coverage map (strict/dev observability)
Related:
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/design/flowbox-adopt-tag-migration-ssot.md
- docs/development/current/main/phases/phase-29aw/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# FlowBox tag coverage map (SSOT)

## Goal

Fix a minimal, stable mapping from regression scenarios to FlowBox schema tags,
so observability stays aligned to route/semantic coverage instead of pattern-number labels.

This map targets **strict/dev** only (release remains silent).

## Interpretation

- `features` is a **required subset**. Extra features may appear as Facts expand.
- `via` is `shadow` today (release does not emit tags in strict/dev).
- `box_kind` is derived from the CorePlan shape that actually emits the tag.
- FlowBox adopt tag は **Verified CorePlan の lowering 時に 1 回だけ**出す（router 集約）。

## Coverage map (Stage-2)

Note:
- smoke 名 / tag suffix には pattern-era token が残るが、これは traceability-only。
- active docs は semantic alias wrapper stem を優先し、legacy stem は wrapper の転送先として保持する。
- current runtime semantics は route 名（`loop_break`, `if_phi_join`, `scan_with_init` など）で読む。

| Scenario | Smoke stem (semantic alias wrapper) | box_kind | features (required subset) | via | Notes |
| --- | --- | --- | --- | --- | --- |
| `loop_simple_while` strict shadow adopt | `loop_simple_while_strict_shadow_vm` | Loop | (empty) | shadow | loop_simple_while subset |
| `scan_with_init` strict shadow adopt | `scan_with_init_strict_shadow_vm` | Loop | return | shadow | scan_with_init subset |
| `split_scan` strict shadow adopt | `split_scan_strict_shadow_vm` | Loop | (empty) | shadow | split_scan subset |
| `loop_true_early_exit` strict shadow adopt | `loop_true_early_exit_strict_shadow_vm` | Loop | break | shadow | loop_true_early_exit subset |
| `loop_break` planner route | `loop_break_plan_subset_vm` | Loop | break | shadow | loop_break subset (generic) |
| `loop_break` body-local route | `loop_break_body_local_vm` | Loop | break | shadow | loop_break promotion (body-local) |
| `loop_break` body-local-seg route | `loop_break_body_local_seg_vm` | Loop | break | shadow | loop_break promotion (body-local + seg) |
| `loop_break` realworld route | `loop_break_realworld_vm` | Loop | break | shadow | loop_break derived-slot (realworld subset) |
| `match_return` strict shadow adopt | `phase29at_match_return_strict_shadow_vm` | Seq | return | shadow | match_return uses Seq(Effects + BranchN) |
| `nested_loop_minimal` strict shadow adopt | `nested_loop_minimal_strict_shadow_vm` | Loop | nested_loop | shadow | nested minimal |
| `if_phi_join` purity gate | `phase29as_purity_gate_vm` | Loop | (empty) | shadow | if_phi_join subset (purity gate only; tag token `pattern3_ifphi` is traceability-only) |
| `generic_loop_continue` strict shadow adopt | `phase29ca_generic_loop_continue_strict_shadow_vm` | Loop | continue | shadow | generic loop continue (strict/dev) |
| `generic_loop_in_body_step` strict shadow adopt | `phase29cb_generic_loop_in_body_step_strict_shadow_vm` | Loop | (empty) | shadow | generic loop in-body step (strict/dev) |

## Negative coverage (no FlowBox adopt tag)

These smoke stems are part of regression coverage and must **not** emit FlowBox adopt tags:

- `phase29ab_pattern2_seg_notapplicable_min_vm` (loop_break not applicable; output-only check)
- `phase29ar_string_is_integer_min_vm` (strict fail-fast reject; expects `[vm-hako/unimplemented] ... newbox(StringUtils)`)

## Gate set (minimal)

The gate for FlowBox tags should include only the rows above to keep it fast.

## P2 migration status

FlowBox checks are now asserted in these strict smokes as well (semantic alias wrapper stems):

- `loop_simple_while_strict_shadow_vm`
- `scan_with_init_strict_shadow_vm`
- `split_scan_strict_shadow_vm`
- `loop_true_early_exit_strict_shadow_vm`
- `nested_loop_minimal_strict_shadow_vm`
- `phase29at_match_return_strict_shadow_vm`
- `phase29as_purity_gate_vm`
