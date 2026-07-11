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
unregistered_baseline = 568

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
path = "hako-alloc-segment-allocation-readiness-scalar-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-S3:hako-alloc-segment/allocation-local-reuse; proof-only readiness and explicit inactive substrate boundaries"
sidecars = [
  "hako-alloc-segment-allocation-readiness-scalar-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "allocation readiness proof is explicitly replaced and its closeout sidecar retires with it"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-consume-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-S3:hako-alloc-segment/allocation-local-reuse; scalar modeled consume proof with real allocation explicitly closed"
sidecars = [
  "hako-alloc-segment-allocation-modeled-consume-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "modeled consume proof is explicitly replaced and its closeout sidecar retires with it"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "hako-alloc-segment-allocation-modeled-consume-ssot.md"
classification_basis = "C2-S3:hako-alloc-segment/allocation-local-reuse; deterministic modeled allocation ledger, not execution authority"
sidecars = [
  "hako-alloc-segment-allocation-modeled-ledger-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "modeled allocation ledger is replaced by an explicit durable inventory owner and its closeout sidecar retires"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-S3:hako-alloc-segment/local-free; deterministic candidate ledger with page/free-list mutation explicitly closed"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "local-free candidate inventory is replaced explicitly and dependent apply rows are reparented"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-local-free-apply-plan-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-ssot.md"
classification_basis = "C2-S3:hako-alloc-segment/local-free; deterministic apply-plan ledger with page mutation explicitly closed"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "local-free apply-plan inventory is replaced explicitly and page-apply dependents are reparented"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-local-free-page-apply-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "hako-alloc-segment-allocation-modeled-local-free-apply-plan-ssot.md"
classification_basis = "C2-S3:hako-alloc-segment/local-free; bounded page-model pilot with existing page owner"
sidecars = [
  "hako-alloc-segment-allocation-modeled-local-free-page-apply-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "page-model apply pilot is replaced explicitly and its closeout sidecar retires with it"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-local-free-integration-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "hako-alloc-segment-allocation-modeled-local-free-page-apply-ssot.md"
classification_basis = "C2-S3:hako-alloc-segment/local-free; composition boundary over existing scalar owners"
sidecars = [
  "hako-alloc-segment-allocation-modeled-local-free-integration-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "local-free composition is replaced explicitly and its closeout sidecar retires with it"

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

[[documents]]
path = "hako-alloc-segment-allocation-blocked-substrate-matrix-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2 residual Q3 accepted: severable proof-only blocked-substrate matrix (MIMAP-149A); deterministic blocker record, not execution authority"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-ledger-release-span-facts-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-ledger-release-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-allocation-modeled-ledger-release-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-applied-recycle-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-applied-recycle-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-released-span-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-handle-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-apply-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-apply-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-derived-lookup-prerequisite-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-remaining-execution-prerequisite-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-remaining-execution-prerequisite-ledger-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-allocation-plan-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-allocation-plan-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-allocation-plan-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-arena-slot-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-arena-slot-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-residence-arena-binding-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-residence-arena-binding-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-residence-arena-binding-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-source-accounting-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-source-accounting-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-modeled-source-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-arena-backing-modeled-source-bridge-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-no-escape-address-capability-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-arena-backing-no-escape-address-capability-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-no-escape-pointer-residence-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-pointer-derived-lookup-execution-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-readiness-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-readiness-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-readiness-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-backing-requirement-matrix-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-backing-requirement-matrix-closeout-ssot.md",
  "hako-alloc-segment-arena-backing-requirement-matrix-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-arena-bitmap-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-arena-backing-and-residence; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-arena-bitmap-inventory-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-map-modeled-consume-ledger-closeout-ssot.md",
  "hako-alloc-segment-map-modeled-consume-ledger-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-apply-plan-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-map-local-free-apply-plan-bridge-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-integration-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-map-local-free-integration-bridge-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-page-apply-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-map-local-free-page-apply-bridge-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-bridge-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-bridge-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-closeout-ssot.md",
  "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-closeout-ssot.md",
  "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-lookup-guarded-readiness-composition-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-modeled-consume-ledger-release-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-map-modeled-consume-ledger-release-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = [
  "hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-mutation-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-released-span-local-free-candidate-bridge-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; proof/pilot/bridge row connecting lanes or proving a scalar capability, not a mutable ledger"
sidecars = [
  "hako-alloc-segment-map-released-span-local-free-candidate-bridge-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof row is explicitly replaced and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-map-scalar-lookup-boundary-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-content-review:hako-alloc-segment/segment-map-and-release; scalar/model inventory-ledger-matrix-prerequisite row recording deterministic modeled state, not execution authority"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3 and its owned sidecars retire with it"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-local-free-reuse-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2 residual Q2 accepted: historical closure record whose direct base MIMAP-126A row is absent from the design root; independent status-ledger row"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2 residual Q2 accepted: historical closure record whose direct base MIMAP-130A row is absent from the design root; independent status-ledger row"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2 residual Q2 accepted: historical closure record whose direct base MIMAP-138A row is absent from the design root; independent status-ledger row"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2 residual Q2 accepted: historical closure record whose direct base MIMAP-134A row is absent from the design root; independent status-ledger row"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3"

[[documents]]
path = "hako-alloc-segment-allocation-modeled-local-free-scalar-lane-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2 residual Q1 accepted: multi-base historical closure record closing MIMAP-107A/109A/111A together; independent status-ledger row, not a single-base sidecar"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3"

[[documents]]
path = "hako-alloc-segment-map-readiness-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2 residual Q1 accepted: multi-base historical closure record closing the MIMAP-149A/151A/153A pack together; independent status-ledger row, not a single-base sidecar"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a replacement row records superseded_by at C3"
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
