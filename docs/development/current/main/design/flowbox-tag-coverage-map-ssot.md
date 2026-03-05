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
so observability does not drift back to pattern-name coupling.

This map targets **strict/dev** only (release remains silent).

## Interpretation

- `features` is a **required subset**. Extra features may appear as Facts expand.
- `via` is `shadow` today (release does not emit tags in strict/dev).
- `box_kind` is derived from the CorePlan shape that actually emits the tag.
- FlowBox adopt tag は **Verified CorePlan の lowering 時に 1 回だけ**出す（router 集約）。

## Coverage map (Stage-2)

| Scenario (smoke) | box_kind | features (required subset) | via | Notes |
| --- | --- | --- | --- | --- |
| `phase29ao_pattern1_strict_shadow_vm` | Loop | (empty) | shadow | Pattern1 subset |
| `phase29ao_pattern6_strict_shadow_vm` | Loop | return | shadow | ScanWithInit subset |
| `phase29ao_pattern7_strict_shadow_vm` | Loop | (empty) | shadow | SplitScan subset |
| `phase29ao_pattern5_strict_shadow_vm` | Loop | break | shadow | Infinite early-exit (break) |
| `phase29ai_pattern2_break_plan_subset_ok_min_vm` | Loop | break | shadow | Pattern2 plan subset (generic) |
| `phase29ab_pattern2_loopbodylocal_min_vm` | Loop | break | shadow | Pattern2 promotion (loopbodylocal) |
| `phase29ab_pattern2_loopbodylocal_seg_min_vm` | Loop | break | shadow | Pattern2 promotion (loopbodylocal + seg) |
| `phase263_pattern2_seg_realworld_min_vm` | Loop | break | shadow | Pattern2 derived-slot (realworld subset) |
| `phase29at_match_return_strict_shadow_vm` | Seq | return | shadow | match_return uses Seq(Effects + BranchN) |
| `phase29ap_pattern6_nested_strict_shadow_vm` | Loop | nested_loop | shadow | nested minimal |
| `phase29as_purity_gate_vm` (pattern3_ifphi) | Loop | (empty) | shadow | if-phi subset (purity gate only) |
| `phase29ca_generic_loop_continue_strict_shadow_vm` | Loop | continue | shadow | generic loop continue (strict/dev) |
| `phase29cb_generic_loop_in_body_step_strict_shadow_vm` | Loop | (empty) | shadow | generic loop in-body step (strict/dev) |

## Negative coverage (no FlowBox adopt tag)

These smokes are part of regression coverage and must **not** emit FlowBox adopt tags:

- `phase29ab_pattern2_seg_notapplicable_min_vm` (Pattern2 not applicable; output-only check)
- `phase29ar_string_is_integer_min_vm` (strict fail-fast reject; expects `[vm-hako/unimplemented] ... newbox(StringUtils)`)

## Gate set (minimal)

The gate for FlowBox tags should include only the rows above to keep it fast.

## P2 migration status

FlowBox checks are now asserted in these strict smokes as well:

- `phase29ao_pattern1_strict_shadow_vm`
- `phase29ao_pattern6_strict_shadow_vm`
- `phase29ao_pattern7_strict_shadow_vm`
- `phase29ao_pattern5_strict_shadow_vm`
- `phase29ap_pattern6_nested_strict_shadow_vm`
- `phase29at_match_return_strict_shadow_vm`
- `phase29as_purity_gate_vm`
