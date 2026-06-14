---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-MATCH-RETURN-RELEASE-TAG-001
Scope: Keep match-return FlowBox tags strict/dev-only while preserving release CorePlan adoption.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md
  - src/mir/builder/stmts/return_stmt.rs
  - tools/smokes/v2/profiles/integration/joinir/match_return_strict_shadow_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/match_return_release_adopt_vm.sh
---

# COREPLAN-MATCH-RETURN-RELEASE-TAG-001

## Decision

`match_return` may use the CorePlan/Seq lowering in release, but FlowBox
observability tags remain strict/dev-only.

The drift was in the return-statement adoption helper: the helper accepted an
`emit_tag` knob and emitted `[flowbox/adopt box_kind=Seq ... via=shadow]` even
on the release path. That violates the FlowBox observability contract, which
states that FlowBox tags are strict/dev diagnostics and release remains silent.

## Implementation

```text
match_return_release_coreplan_adopt=1
match_return_release_flowbox_tag=0
match_return_strict_flowbox_tag=1
match_return_timeout_secs=30
accepted_shape_added=0
fallback_route_added=0
```

## Evidence

```text
match_return_release_adopt_vm.sh -> PASS
match_return_strict_shadow_vm.sh -> PASS
```

## Acceptance

```text
match_return_release_exit_code=20
match_return_release_flowbox_tag_count=0
match_return_strict_flowbox_seq_tag=1
match_return_timeout_secs=30
accepted_shape_added=0
fallback_route_added=0
```

## Proof

```bash
bash tools/smokes/v2/profiles/integration/joinir/match_return_release_adopt_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/match_return_strict_shadow_vm.sh
```

## Stop Line

```text
do not disable release CorePlan adoption for match_return
do not allow FlowBox tags in release output
do not move match_return observability into smoke-only filtering
```
