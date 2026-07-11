---
Status: SSOT
Date: 2026-07-11
Decision: accepted
Scope: design root membership, role, precedence, sidecar, and retirement registry
---

# Design Authority Registry

This file is the sole membership and precedence owner for direct files in
`docs/development/current/main/design/`. The language charter remains the
normative language-law precedence owner; this registry only classifies design
artifacts and their relationships.

`README.md` is a navigation view. It does not grant authority. During warning
rollout, unregistered files remain in place and the baseline may only decrease.

<!-- design-registry-v0:begin -->
```toml
schema_version = 0
mode = "warning"
unregistered_baseline = 732

[[documents]]
path = "INDEX.md"
role = "authority"
owner = "repository-artifact-lifecycle"
precedence_parent = "docs/reference/language/charter.md"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "design root no longer uses file-owned registry"

[[documents]]
path = "README.md"
role = "navigation"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "generated navigation replaces this view"

[[documents]]
path = "agent-current-entry-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "AGENTS.md"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "root agent entry contract is replaced explicitly"

[[documents]]
path = "current-docs-archive-policy-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "DOCS_LAYOUT.md"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "repository artifact lifecycle policy is replaced explicitly"

[[documents]]
path = "final-metal-split-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Current Kernel Replacement Docs"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "joinir-legacy-fixture-pin-inventory-ssot.md"
role = "supporting"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Retirement / Inventory Supporting Docs"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "joinir-smoke-legacy-stem-retirement-ssot.md"
role = "supporting"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Retirement / Inventory Supporting Docs"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "current-docs-update-policy-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "CURRENT_STATE.toml"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "current documentation update policy is replaced explicitly"
[[documents]]
path = "ai-handoff-and-debug-contract.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "artifact-policy-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "atomic-tls-gc-truthful-native-seam-inventory.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "auto-specialize-box-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "builder-emit-facade-visibility-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "code-retirement-history-policy-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "compiler-cleanliness-campaign-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "compiler-expressivity-first-policy.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "compiler-pipeline-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "compiler-pipeline-thinning-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "compiler-task-map-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "condition-observation-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "constructor-birth-new-lifecycle-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "control-tree.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "copy-emission-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "coreloop-continue-target-slot-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "coreplan-skeleton-feature-model.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "current-optimization-mechanisms-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "de-rust-compiler-thin-rust-roadmap-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "de-rust-post-g1-runtime-plan-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "edgecfg-fragments.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "exception-cleanup-async.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "execution-lanes-and-axis-separation-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "execution-lanes-legacy-retirement-inventory-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "execution-lanes-migration-task-pack-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "feature-helper-cross-pipeline-map.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "fini-cleanup-execution-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "gc-tls-atomic-capability-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "generic-case-a-trim-thinning-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "generic-loop-v1-acceptance-by-recipe-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hako-alloc-mimalloc-port-identity-boundary-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hako-alloc-segment-lifecycle-scalar-state-ssot.md"
role = "supporting"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-S3:hako-alloc-segment/lifecycle; proof-only scalar state and explicit inactive stop lines"
sidecars = [
  "hako-alloc-segment-lifecycle-scalar-state-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "lifecycle scalar proof is explicitly replaced and its closeout sidecar retires with it"

[[documents]]
path = "hako-alloc-segment-page-membership-scalar-ssot.md"
role = "supporting"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-S3:hako-alloc-segment/lifecycle; proof-only page membership relation and explicit inactive stop lines"
sidecars = [
  "hako-alloc-segment-page-membership-scalar-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "page membership proof is explicitly replaced and its closeout sidecar retires with it"

[[documents]]
path = "hako-alloc-policy-state-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hako-fullstack-host-abi-completion-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hako-host-facade-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hako-mirbuilder-load-store-minimal-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hako-option-null-no-match-policy-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hako-runtime-c-abi-cutover-order-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hakoruneup-release-distribution-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "helper-boundary-policy-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hotline-core-method-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "inline-boundary-builder-thinning-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "inline-plan-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "join-explicit-cfg-construction.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "joinir-design-map.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "joinir-extension-dual-route-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "joinir-frontend-legacy-fixture-key-retirement-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "joinir-target-lowerer-thinning-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "kernel-implementation-phase-plan-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "kernel-replacement-axis-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "lego-composability-policy.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "lifecycle-typed-value-language-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "llvm-line-ownership-and-boundary-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "local-patch-prevention-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "loop-body-local-init-thinning-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "loop-canonicalizer.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "loop-update-analyzer-thinning-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mimalloc-capability-taskboard-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mimalloc-hako-port-implementation-plan-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "hako-alloc-mimalloc-port-identity-boundary-ssot.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2:hako-alloc-segment; explicit enum lifecycle, transition, guard, and capability boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "lifecycle authority is replaced explicitly and dependent rows are reparented"

[[documents]]
path = "minimal-capability-modules-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "minimum-verifier-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mir-callsite-retire-lane-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mir-canonical-callsite-lane-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mir-diagnostics-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mir-instruction-diet-ledger-ssot.md"
role = "status-ledger"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "mutable status ledger is replaced by an explicit registry row"

[[documents]]
path = "mir-vm-llvm-instruction-contract-fix-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "normalized-dev-removal-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "normalized-expr-lowering.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "optimization-portability-classification-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "optimization-ssot-string-helper-density.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "optimization-task-card-os-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "phi-input-strategy-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "phi-lifecycle-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "plan-lowering-entry-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "planner-entry-guards-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "primitive-family-and-user-box-fast-path-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "raw-array-substrate-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "raw-map-substrate-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "raw-map-truthful-native-seam-inventory.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "recipe-first-entry-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "recipe-tree-and-parts-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "record-and-packed-array-lowering-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "repl-mir-interpreter-interactive-session-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "return-proof-vocabulary-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "rune-profile-effect-capability-plan-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "runtime-hot-lane-optimization-patterns-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "rust-kernel-export-surface-strata-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "scope-exit-surface-cleanup-canonicalization-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "selfhost-lift-boundary-and-task-order-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "selfhost-parser-mirbuilder-migration-order-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "selfhost-smoke-retirement-inventory-ssot.md"
role = "status-ledger"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "mutable status ledger is replaced by an explicit registry row"

[[documents]]
path = "stage0-llvm-line-shape-inventory-ssot.md"
role = "status-ledger"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "mutable status ledger is replaced by an explicit registry row"

[[documents]]
path = "stage1-mir-authority-boundary-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "stage1-mir-dialect-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "stage2-aot-core-proof-vocabulary-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "stage2-aot-native-thin-path-design-note.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "stage2-fast-leaf-manifest-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "stage2-hako-owner-vs-inc-thin-shim-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "stage2-optimization-debug-bundle-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "stage2-selfhost-and-hako-alloc-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "static-const-table-syntax-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "substrate-capability-ladder-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Read First Now"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "thread-and-tls-capability-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "type-system-policy-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "user-method-policy-thinning-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "value-repr-and-abi-manifest-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "vm-fallback-lane-separation-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "vm-hako-array-shim-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:Diagnostics / Contracts（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "wasm-hako-only-output-roadmap-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "README:現役の設計図（入口）"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"
```
<!-- design-registry-v0:end -->

## Role Vocabulary

```text
authority    normative design owner under its declared precedence parent
navigation   generated or checked human-facing view
supporting   durable explanatory evidence without normative precedence
status-ledger mutable implementation/status evidence
superseded   retained only until reference closure and physical archive
```

Strict mode is allowed only when every direct design file is represented by a
row or owned as a sidecar and the unregistered count is zero.
