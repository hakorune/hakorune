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
unregistered_baseline = 77

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
path = "mirbuilder-final-pipeline-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "recipe-first-entry-contract-ssot.md"
classification_basis = "DOCS_LAYOUT:MirBuilder final production pipeline"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "MirBuilder final production pipeline is replaced explicitly"

[[documents]]
path = "mirbuilder-inplace-replacement-policy-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "mirbuilder-final-pipeline-ssot.md"
classification_basis = "DOCS_LAYOUT:MirBuilder in-place replacement policy"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "MirBuilder replacement method is replaced explicitly"

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
path = "ai-verifiable-development-north-star-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "compiler-expressivity-first-policy.md"
classification_basis = "accepted cross-layer policy for minimizing verified convergence cost"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "AI-verifiable development North Star is replaced explicitly"

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
role = "superseded"
owner = "language-result-propagation-and-exit-transaction-ssot.md"
precedence_parent = "language-result-propagation-and-exit-transaction-ssot.md"
classification_basis = "Historical catch/Invoke proposal; C′ supersedes its failure/cleanup route"
sidecars = []
supersedes = []
superseded_by = "language-result-propagation-and-exit-transaction-ssot.md"
retire_when = "C′ failure/cleanup closeout and separate async authority no longer need this combined proposal"

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
path = "joinir-loop-selfhost-recipe-pipeline-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "recipe-first-entry-contract-ssot.md"
classification_basis = "CURRENT_STATE: active Loop replacement/cutover authority"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "Loop replacement/cutover authority is replaced explicitly"

[[documents]]
path = "joinir-generic-post-effect-debt-classification-ssot.md"
role = "authority"
owner = "joinir-loop-selfhost-recipe-pipeline-ssot.md"
precedence_parent = "joinir-loop-selfhost-recipe-pipeline-ssot.md"
classification_basis = "CURRENT_STATE: active Generic debt design stop"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "M4 Generic debt classification is closed and handed off"

[[documents]]
path = "joinir-loop-pre-effect-product-ssot.md"
role = "authority"
owner = "joinir-loop-selfhost-recipe-pipeline-ssot.md"
precedence_parent = "joinir-loop-selfhost-recipe-pipeline-ssot.md"
classification_basis = "Loop pipeline: pre-effect product boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "pre-effect product boundary is replaced explicitly"

[[documents]]
path = "joinir-pattern-selection-shadow-ssot.md"
role = "supporting"
owner = "joinir-loop-selfhost-recipe-pipeline-ssot.md"
precedence_parent = "joinir-loop-selfhost-recipe-pipeline-ssot.md"
classification_basis = "Loop pipeline: route selection shadow evidence"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "route selection shadow evidence is retired or superseded"

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

[[documents]]
path = "mimalloc-replacement-front-fidelity-ssot.md"
role = "authority"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: fidelity guard: a fast replacement front is not a keeper unless fast *through* a mimalloc-shaped route; normative acceptance law"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mimalloc-row-validation-cadence-ssot.md"
role = "authority"
owner = "current-docs-update-policy-ssot.md"
precedence_parent = "current-docs-update-policy-ssot.md"
classification_basis = "C2-singleton-mz: normative cadence: each mimalloc/hako_alloc row must use the smallest sufficient validation level"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mir-array-slot-residence-ssot.md"
role = "authority"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-mz: (Provisional) defines the C-parity ArrayBox slot residence / DirectSlotOp design owner"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mir-cleanup-policy-ssot.md"
role = "authority"
owner = "compiler-cleanliness-campaign-ssot.md"
precedence_parent = "compiler-cleanliness-campaign-ssot.md"
classification_basis = "C2-singleton-mz: normative policy: MIR cleanup is BoxShape-only unless an accepted BoxCount card says otherwise"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mir-commonality-taxonomy-ssot.md"
role = "authority"
owner = "compiler-pipeline-ssot.md"
precedence_parent = "compiler-pipeline-ssot.md"
classification_basis = "C2-singleton-mz: normative shared-boundary taxonomy (escape / allowlist-gate / owner) fixing what may be commonized"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mir-fastmem-memop-dialect-ssot.md"
role = "authority"
owner = "stage1-mir-dialect-contract-ssot.md"
precedence_parent = "stage1-mir-dialect-contract-ssot.md"
classification_basis = "C2-singleton-mz: defines the MIR representation boundary for `.hako` fastmem regions / memory dialect ops (ContractRegionV0 profile)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mir-proof-envelope-v0-ssot.md"
role = "authority"
owner = "mir-diagnostics-contract-ssot.md"
precedence_parent = "mir-diagnostics-contract-ssot.md"
classification_basis = "C2-singleton-mz: (Active) contract: share only a small proof envelope, not access-plan payloads"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mir-root-facade-contract-ssot.md"
role = "authority"
owner = "compiler-pipeline-ssot.md"
precedence_parent = "compiler-pipeline-ssot.md"
classification_basis = "C2-singleton-mz: `src/mir/mod.rs` facade export contract; facade must not own semantic metadata vocabulary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mirbuilder-authority-based-hako-migration-ssot.md"
role = "authority"
owner = "selfhost-parser-mirbuilder-migration-order-ssot.md"
precedence_parent = "selfhost-parser-mirbuilder-migration-order-ssot.md"
classification_basis = "C2-singleton-mz: normative rule: MirBuilder migration unit is authority, not Rust module/struct/file"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mirbuilder-ordering-capability-ssot.md"
role = "authority"
owner = "selfhost-parser-mirbuilder-migration-order-ssot.md"
precedence_parent = "selfhost-parser-mirbuilder-migration-order-ssot.md"
classification_basis = "C2-singleton-mz: capability boundary: generic ordering capability, not OrderedMapBox/RegionObserver special case"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mirbuilder-programjson-capability-batch-migration-policy-ssot.md"
role = "authority"
owner = "selfhost-parser-mirbuilder-migration-order-ssot.md"
precedence_parent = "selfhost-parser-mirbuilder-migration-order-ssot.md"
classification_basis = "C2-singleton-mz: migration policy: replace `1 shape = 1 card` ProgramJSON cadence with capability batching; provisional parent pending consultation on mirbuilder-rust-to-hako-converter-task-order-ssot.md"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "nested-argument-single-evaluation-ssot.md"
role = "authority"
owner = "normalized-expr-lowering.md"
precedence_parent = "normalized-expr-lowering.md"
classification_basis = "C2-singleton-mz: (Active) correctness contract: nested call arguments evaluated exactly once"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "nyash-kernel-semantic-owner-ssot.md"
role = "authority"
owner = "rust-kernel-export-surface-strata-ssot.md"
precedence_parent = "rust-kernel-export-surface-strata-ssot.md"
classification_basis = "C2-singleton-mz: final owner graph: Rust host microkernel / `.hako` semantic kernel / native accelerators + ABI facade/compat quarantine"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "object-handle-box-identity-contract-ssot.md"
role = "authority"
owner = "lifecycle-typed-value-language-ssot.md"
precedence_parent = "lifecycle-typed-value-language-ssot.md"
classification_basis = "C2-singleton-mz: ObjectHandle / BoxIdentity ownership-substrate token contract (ARC-RETIRE-003)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "object-storage-plan-boundary-ssot.md"
role = "authority"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-mz: boundary: object representation belongs to later plans / backend lowering, not MIRBuilder"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "observer-control-dce-owner-contract-ssot.md"
role = "authority"
owner = "compiler-pipeline-ssot.md"
precedence_parent = "compiler-pipeline-ssot.md"
classification_basis = "C2-singleton-mz: Observer/Control DCE owner contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "optimization-hints-contracts-intrinsic-ssot.md"
role = "authority"
owner = "optimization-task-card-os-ssot.md"
precedence_parent = "optimization-task-card-os-ssot.md"
classification_basis = "C2-singleton-mz: (Provisional) canonical `@rune` optimization-metadata surface + legacy compat aliases"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "optimization-tag-flow-ssot.md"
role = "authority"
owner = "optimization-task-card-os-ssot.md"
precedence_parent = "optimization-task-card-os-ssot.md"
classification_basis = "C2-singleton-mz: fixes the reach of optimization tags/knobs/selectors across `.hako`/MIR"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "ordered-map-box-boundary-ssot.md"
role = "authority"
owner = "raw-map-substrate-ssot.md"
precedence_parent = "raw-map-substrate-ssot.md"
classification_basis = "C2-singleton-mz: accepted design boundary for deterministic OrderedMapBox"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "pattern6-7-contracts.md"
role = "authority"
owner = "joinir-extension-dual-route-contract-ssot.md"
precedence_parent = "joinir-extension-dual-route-contract-ssot.md"
classification_basis = "C2-singleton-mz: SSOT contract boundary for `scan_with_init`/`split_scan` (NotApplicable vs Freeze)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "perf-optimization-method-ssot.md"
role = "authority"
owner = "optimization-task-card-os-ssot.md"
precedence_parent = "optimization-task-card-os-ssot.md"
classification_basis = "C2-singleton-mz: fixed measurement/decision/stop-line order for exe optimization across `.hako`/C ABI/Rust bridge"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "perf-owner-first-optimization-ssot.md"
role = "authority"
owner = "optimization-task-card-os-ssot.md"
precedence_parent = "optimization-task-card-os-ssot.md"
classification_basis = "C2-singleton-mz: front-split / owner-state / keeper-revert stop-line contract for perf lanes"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "perf-userbox-link-startup-attribution-ssot.md"
role = "authority"
owner = "optimization-task-card-os-ssot.md"
precedence_parent = "optimization-task-card-os-ssot.md"
classification_basis = "C2-singleton-mz: exact-AOT link option contract + startup attribution ladder for PERF-USERBOX"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "pinned-typed-object-arena-ssot.md"
role = "authority"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-mz: (Active) storage contract required before DirectSlotLease / NativeDirect typed-object lowering"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "plan-reject-handoff-gap-taxonomy-ssot.md"
role = "authority"
owner = "ai-handoff-and-debug-contract.md"
precedence_parent = "ai-handoff-and-debug-contract.md"
classification_basis = "C2-singleton-mz: makes 'where to add the next lego box' mechanical; every `reject:<reason>` must imply a taxonomy outcome"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "planfrag-freeze-taxonomy.md"
role = "authority"
owner = "joinir-design-map.md"
precedence_parent = "joinir-design-map.md"
classification_basis = "C2-singleton-mz: JoinIR plan/normalize/lower freeze-tag taxonomy (phase-agnostic)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "post-phi-final-form-ssot.md"
role = "authority"
owner = "phi-input-strategy-ssot.md"
precedence_parent = "phi-input-strategy-ssot.md"
classification_basis = "C2-singleton-mz: final representation + local verification of pred-varying join (PHI-equivalent) values"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "post-store-observer-facts-ssot.md"
role = "authority"
owner = "condition-observation-ssot.md"
precedence_parent = "condition-observation-ssot.md"
classification_basis = "C2-singleton-mz: (provisional) compile-time facts contract for observers after a store boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "private-doc-boundary-migration-ssot.md"
role = "authority"
owner = "current-docs-archive-policy-ssot.md"
precedence_parent = "current-docs-archive-policy-ssot.md"
classification_basis = "C2-singleton-mz: public/private doc boundary policy + staged migration"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "provider-abi-v1-ssot.md"
role = "authority"
owner = "hakoruneup-release-distribution-ssot.md"
precedence_parent = "hakoruneup-release-distribution-ssot.md"
classification_basis = "C2-singleton-mz: common Hakorune provider ABI v1 vocabulary (metadata/manifest/load; activation & replacement parked)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "provider-package-artifact-ssot.md"
role = "authority"
owner = "provider-abi-v1-ssot.md"
precedence_parent = "provider-abi-v1-ssot.md"
classification_basis = "C2-singleton-mz: provider package artifact contract + manifest layout"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "provider-runtime-load-ssot.md"
role = "authority"
owner = "provider-abi-v1-ssot.md"
precedence_parent = "provider-abi-v1-ssot.md"
classification_basis = "C2-singleton-mz: staged provider runtime-load ladder + fail-fast boundaries"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "pure-first-acceptance-layer-flow-ssot.md"
role = "authority"
owner = "minimum-verifier-ssot.md"
precedence_parent = "minimum-verifier-ssot.md"
classification_basis = "C2-singleton-mz: acceptance contract: pure-first EXE failures must identify the failing compiler layer"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "pure-first-mir-artifact-and-diagnostics-ssot.md"
role = "authority"
owner = "mir-diagnostics-contract-ssot.md"
precedence_parent = "mir-diagnostics-contract-ssot.md"
classification_basis = "C2-singleton-mz: pure-first MIR artifact + diagnostics contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "random-capability-failfast-ssot.md"
role = "authority"
owner = "substrate-capability-ladder-ssot.md"
precedence_parent = "substrate-capability-ladder-ssot.md"
classification_basis = "C2-singleton-mz: `uses random` capability / fail-fast contract (RANDOM-CAP-001)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "random-capability-preflight-ssot.md"
role = "authority"
owner = "random-capability-failfast-ssot.md"
precedence_parent = "random-capability-failfast-ssot.md"
classification_basis = "C2-singleton-mz: unsupported-random route preflight (RANDOM-CAP-002)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "recipe-scope-effect-policy-ssot.md"
role = "authority"
owner = "recipe-tree-and-parts-ssot.md"
precedence_parent = "recipe-tree-and-parts-ssot.md"
classification_basis = "C2-singleton-mz: recipe/scope/effect/policy/leaf responsibility boundary for user-box optimization"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "reclaim-execution-preflight-ssot.md"
role = "authority"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: reclaim execution intent marker + unsupported preflight (MIMAP-052B)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "record-box-two-surface-one-substrate-ssot.md"
role = "authority"
owner = "record-and-packed-array-lowering-ssot.md"
precedence_parent = "record-and-packed-array-lowering-ssot.md"
classification_basis = "C2-singleton-mz: `record` vs `box` user-facing distinction + shared internal optimization substrate"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "record-construction-ergonomics-ssot.md"
role = "authority"
owner = "record-and-packed-array-lowering-ssot.md"
precedence_parent = "record-and-packed-array-lowering-ssot.md"
classification_basis = "C2-singleton-mz: accepted record construction ergonomics contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "record-construction-read-lowering-ssot.md"
role = "authority"
owner = "record-and-packed-array-lowering-ssot.md"
precedence_parent = "record-and-packed-array-lowering-ssot.md"
classification_basis = "C2-singleton-mz: REC-002 Stage1 record construction/read lowering"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "record-literal-parser-capsule-ssot.md"
role = "authority"
owner = "record-and-packed-array-lowering-ssot.md"
precedence_parent = "record-and-packed-array-lowering-ssot.md"
classification_basis = "C2-singleton-mz: REC-001 Stage0 record-literal parser capsule"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "record-local-scalarization-ssot.md"
role = "authority"
owner = "record-and-packed-array-lowering-ssot.md"
precedence_parent = "record-and-packed-array-lowering-ssot.md"
classification_basis = "C2-singleton-mz: compiler-owned record-local scalarization contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "record-with-update-lowering-ssot.md"
role = "authority"
owner = "record-and-packed-array-lowering-ssot.md"
precedence_parent = "record-and-packed-array-lowering-ssot.md"
classification_basis = "C2-singleton-mz: REC-003 record with-update lowering"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "representation-direct-lowering-ssot.md"
role = "authority"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-mz: (Active) C-like representation/direct-lowering authority contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "representation-direct-storage-substrate-ssot.md"
role = "authority"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-mz: (Active) storage substrate contract for NativeDirect representation"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "result-capsule-value-representation-ssot.md"
role = "authority"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-mz: (Active) result-capsule representation decisions before helper fusion"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "result-option-prelude-diagnostics-ssot.md"
role = "authority"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-mz: RESULT-001 Result/Option prelude enum + variant diagnostics"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "retained-boundary-and-birth-placement-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: (provisional) BoundaryKind vs retained-representation separation for string hot path"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "return-in-loop-minimal-ssot.md"
role = "authority"
owner = "generic-loop-v1-acceptance-by-recipe-ssot.md"
precedence_parent = "generic-loop-v1-acceptance-by-recipe-ssot.md"
classification_basis = "C2-singleton-mz: minimal early-return-in-loop acceptance vocabulary (stdlib `is_integer` shape)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "route-fixpoint-owner-ssot.md"
role = "authority"
owner = "compiler-pipeline-ssot.md"
precedence_parent = "compiler-pipeline-ssot.md"
classification_basis = "C2-singleton-mz: (Active) route metadata refresh is a compiler-owned convergence system"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "rune-and-stage2plus-final-shape-ssot.md"
role = "authority"
owner = "rune-profile-effect-capability-plan-ssot.md"
precedence_parent = "rune-profile-effect-capability-plan-ssot.md"
classification_basis = "C2-singleton-mz: (provisional) stage2-mainline daily shape + stage2+/Rune role split"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "rune-v0-contract-rollout-ssot.md"
role = "authority"
owner = "rune-profile-effect-capability-plan-ssot.md"
precedence_parent = "rune-profile-effect-capability-plan-ssot.md"
classification_basis = "C2-singleton-mz: (Provisional) Rune v0 syntax/parser/carrier/backend scope"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "rune-v1-metadata-unification-ssot.md"
role = "authority"
owner = "rune-profile-effect-capability-plan-ssot.md"
precedence_parent = "rune-profile-effect-capability-plan-ssot.md"
classification_basis = "C2-singleton-mz: (Provisional) canonical `@rune` metadata surface + legacy `@hint/@contract/@intrinsic_candidate` aliases"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "runtime-gc-policy-and-order-ssot.md"
role = "authority"
owner = "de-rust-post-g1-runtime-plan-ssot.md"
precedence_parent = "de-rust-post-g1-runtime-plan-ssot.md"
classification_basis = "C2-singleton-mz: GC semantic position + runtime implementation order"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "rust-lifecycle-projection-ssot.md"
role = "authority"
owner = "de-rust-compiler-thin-rust-roadmap-ssot.md"
precedence_parent = "de-rust-compiler-thin-rust-roadmap-ssot.md"
classification_basis = "C2-singleton-mz: projection of Rust semantic migration facts into Hako lifecycle plans"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "rustc-semir-internal-adapter-boundary.md"
role = "authority"
owner = "rust-lifecycle-projection-ssot.md"
precedence_parent = "rust-lifecycle-projection-ssot.md"
classification_basis = "C2-singleton-mz: (Design) rustc semantic fact-source boundary for lifecycle migration"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "selfhost-compiler-structure-ssot.md"
role = "authority"
owner = "selfhost-lift-boundary-and-task-order-ssot.md"
precedence_parent = "selfhost-lift-boundary-and-task-order-ssot.md"
classification_basis = "C2-singleton-mz: selfhost/MIR-direct/de-Rust compiler structure + ownership north-star"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "selfhost-coreplan-unblocking-policy.md"
role = "authority"
owner = "compiler-expressivity-first-policy.md"
precedence_parent = "compiler-expressivity-first-policy.md"
classification_basis = "C2-singleton-mz: policy: strengthen CorePlan instead of patching `.hako` on bringup stalls"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "selfhost-family-artifact-route-seam-ssot.md"
role = "authority"
owner = "selfhost-lift-boundary-and-task-order-ssot.md"
precedence_parent = "selfhost-lift-boundary-and-task-order-ssot.md"
classification_basis = "C2-singleton-mz: accepted minimal route seam for selecting compiler-family implementations"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "selfhost-g1-mir-compare-policy-ssot.md"
role = "authority"
owner = "selfhost-lift-boundary-and-task-order-ssot.md"
precedence_parent = "selfhost-lift-boundary-and-task-order-ssot.md"
classification_basis = "C2-singleton-mz: (Active) selfhost G1 MIR compare policy"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "selfhost-language-v1-freeze-ssot.md"
role = "authority"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-mz: Language v1 completion boundary (surface syntax + AST/JSON v0 + fail-fast tags)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "selfhost-mir-object-metadata-ssot.md"
role = "authority"
owner = "selfhost-parser-mirbuilder-migration-order-ssot.md"
precedence_parent = "selfhost-parser-mirbuilder-migration-order-ssot.md"
classification_basis = "C2-singleton-mz: minimal object-meaning metadata selfhost `.hako` MIRBuilder may emit"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "selfhost-program-json-boundary-vocabulary-ssot.md"
role = "authority"
owner = "selfhost-parser-mirbuilder-migration-order-ssot.md"
precedence_parent = "selfhost-parser-mirbuilder-migration-order-ssot.md"
classification_basis = "C2-singleton-mz: daily selfhost vocabulary around Program(JSON v0) boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "decoded-utf8-byte-length-contract-v0.md"
role = "authority"
owner = "selfhost-program-json-boundary-vocabulary-ssot.md"
precedence_parent = "selfhost-program-json-boundary-vocabulary-ssot.md"
classification_basis = "SnapshotV0 RHako/HHako decoded UTF-8 byte-count authority, capability task order, and adapter retirement boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "ProgramV0 adapters retire and no non-Snapshot consumer retains the generic byte-length contract"

[[documents]]
path = "selfhost-tools-loopless-subset-ssot.md"
role = "authority"
owner = "selfhost-coreplan-unblocking-policy.md"
precedence_parent = "selfhost-coreplan-unblocking-policy.md"
classification_basis = "C2-singleton-mz: selfhost tooling loopless-subset policy under `NYASH_DISABLE_PLUGINS=1`"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "semantic-optimization-authority-ssot.md"
role = "authority"
owner = "optimization-portability-classification-ssot.md"
precedence_parent = "optimization-portability-classification-ssot.md"
classification_basis = "C2-singleton-mz: optimization-authority chain `.hako owner -> MIR contract -> Rust executor -> LLVM`"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "short-circuit-joins-ssot.md"
role = "authority"
owner = "join-explicit-cfg-construction.md"
precedence_parent = "join-explicit-cfg-construction.md"
classification_basis = "C2-singleton-mz: accepted `&&`/`||` short-circuit vs 2-state `joins` model contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "smoke-taxonomy-and-discovery-ssot.md"
role = "authority"
owner = "current-docs-update-policy-ssot.md"
precedence_parent = "current-docs-update-policy-ssot.md"
classification_basis = "C2-singleton-mz: smoke profile taxonomy + discovery rules preserving runner contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "span-no-escape-ssot.md"
role = "authority"
owner = "raw-array-substrate-ssot.md"
precedence_parent = "raw-array-substrate-ssot.md"
classification_basis = "C2-singleton-mz: no-escape Span contract over DirectArrayI64"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "stage0-cleanup-catch-boundary-ssot.md"
role = "superseded"
owner = "language-result-propagation-and-exit-transaction-ssot.md"
precedence_parent = "language-result-propagation-and-exit-transaction-ssot.md"
classification_basis = "Historical Stage0 catch/postfix-cleanup bridge inventory; C′ target supersedes it"
sidecars = []
supersedes = []
superseded_by = "language-result-propagation-and-exit-transaction-ssot.md"
retire_when = "C′ R0 and DOC0 retire the legacy parser/MIR bridge and its guard vocabulary"

[[documents]]
path = "stage0-stage1-feature-responsibility-split-ssot.md"
role = "authority"
owner = "stage1-mir-authority-boundary-ssot.md"
precedence_parent = "stage1-mir-authority-boundary-ssot.md"
classification_basis = "C2-singleton-mz: Stage0/Stage1/Stage2-mainline feature responsibility split"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "standalone-exe-route-contract-ssot.md"
role = "authority"
owner = "hakoruneup-release-distribution-ssot.md"
precedence_parent = "hakoruneup-release-distribution-ssot.md"
classification_basis = "C2-singleton-mz: (Active) standalone EXE route contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "strict-nested-loop-guard-ssot.md"
role = "authority"
owner = "planner-entry-guards-ssot.md"
precedence_parent = "planner-entry-guards-ssot.md"
classification_basis = "C2-singleton-mz: JoinIR composer `strict_nested_loop_guard` contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "string-birth-placement-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: (provisional) placement decision before `freeze.str`"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "string-birth-sink-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: (provisional) `freeze.str` as the sole string birth sink"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "string-hot-corridor-runtime-carrier-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: (Provisional) runtime-private text carrier stack for exact/meso/whole fronts"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "string-semantic-value-and-publication-boundary-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: (Provisional) String immutable value model + publish/freeze boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "string-transient-lifecycle-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: string hot path 4-layer reading (authority/transient/birth/substrate)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "string-value-model-phased-rollout-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: (Provisional) String value-model north-star + phased rollout"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "substring-concat-len-closed-form-lowering-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: stable-length exact-route closed-form lowering decision"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "substring-view-materialize-boundary-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: (Provisional) substring StringView v0 + materialize boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "tool-entrypoint-lifecycle-ssot.md"
role = "authority"
owner = "code-retirement-history-policy-ssot.md"
precedence_parent = "code-retirement-history-policy-ssot.md"
classification_basis = "C2-singleton-mz: root tool entrypoint protection / archive / delete lifecycle policy"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "transient-text-pieces-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: (provisional) normalized small piece list (TextPlan/PiecesN) transient carrier"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "transition-metadata-capsule-ssot.md"
role = "authority"
owner = "selfhost-language-v1-freeze-ssot.md"
precedence_parent = "selfhost-language-v1-freeze-ssot.md"
classification_basis = "C2-singleton-mz: TRANS-001 Stage0 transition metadata-only syntax"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "type-abi-box-domain-ssot.md"
role = "authority"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-mz: Box Domain ownership for Type ABI views / plugin route / NewBox-DropBox / TypeBox slots"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "type-abi-view-and-plan-stamp-ssot.md"
role = "authority"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-mz: Type ABI read-only view boundary + TypeAbiPack + PlanStamp task order"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "type-alias-parser-capsule-ssot.md"
role = "authority"
owner = "selfhost-language-v1-freeze-ssot.md"
precedence_parent = "selfhost-language-v1-freeze-ssot.md"
classification_basis = "C2-singleton-mz: TYPE-001 Stage0 type-alias parser capsule"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "typed-array-method-contract-ssot.md"
role = "authority"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-mz: ARRAY-002A typed Array<T> method-surface contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "typed-object-exact-slot-abi-ssot.md"
role = "authority"
owner = "primitive-family-and-user-box-fast-path-ssot.md"
precedence_parent = "primitive-family-and-user-box-fast-path-ssot.md"
classification_basis = "C2-singleton-mz: typed-object exact-slot ABI split for C-speed user-box field access"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "unwind-cleanup-effect-integration-ssot.md"
role = "supporting"
owner = "exitkind-cleanup-effect-contract-ssot.md"
precedence_parent = "exitkind-cleanup-effect-contract-ssot.md"
classification_basis = "Reserved future-Unwind integration observations; not a C′ recoverable-failure authority"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "a separate terminal-Fault/host-unwind Decision accepts or rejects the reserved ExitKind"

[[documents]]
path = "userbox-nullable-object-return-ssot.md"
role = "authority"
owner = "primitive-family-and-user-box-fast-path-ssot.md"
precedence_parent = "primitive-family-and-user-box-fast-path-ssot.md"
classification_basis = "C2-singleton-mz: same-module user-box route metadata for nullable-object-returning methods"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "uses-metadata-capsule-ssot.md"
role = "authority"
owner = "selfhost-language-v1-freeze-ssot.md"
precedence_parent = "selfhost-language-v1-freeze-ssot.md"
classification_basis = "C2-singleton-mz: USES-001 Stage0 method-level `uses` capability metadata capsule"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "usize-semantic-foundation-ssot.md"
role = "authority"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-mz: exact `usize`/pointer-sized unsigned semantics before mimalloc migration"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "value-corridor-generic-optimization-contract.md"
role = "authority"
owner = "optimization-portability-classification-ssot.md"
precedence_parent = "optimization-portability-classification-ssot.md"
classification_basis = "C2-singleton-mz: generic value-corridor optimization contract vocabulary (string/bytes/scalar/array/map)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "valueflow-blockparams-ssot.md"
role = "authority"
owner = "phi-input-strategy-ssot.md"
precedence_parent = "phi-input-strategy-ssot.md"
classification_basis = "C2-singleton-mz: (design-only) ValueFlow BlockParams + edge_args SSA-merge representation"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "verified-recipe-port-sig-ssot.md"
role = "authority"
owner = "recipe-tree-and-parts-ssot.md"
precedence_parent = "recipe-tree-and-parts-ssot.md"
classification_basis = "C2-singleton-mz: (design-only) VerifiedRecipe PortSig + wiring contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "vm-active-lane-retirement-ssot.md"
role = "authority"
owner = "vm-fallback-lane-separation-ssot.md"
precedence_parent = "vm-fallback-lane-separation-ssot.md"
classification_basis = "C2-singleton-mz: Rust VM / `.hako` VM active-development boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "mimalloc-hakorune-brand-type-vocabulary-ssot.md"
role = "supporting"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-005A blueprint brand/type vocabulary model"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "mimalloc-hakorune-capability-surface-ssot.md"
role = "supporting"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-005D blueprint capability-surface model"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "mimalloc-hakorune-lifecycle-skeleton-ssot.md"
role = "supporting"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-005C non-executable enum/transition lifecycle skeleton"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "mimalloc-hakorune-record-vocabulary-ssot.md"
role = "supporting"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-005B blueprint record vocabulary model"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "mimalloc-lifecycle-integration-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-009 lifecycle integration pilot"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "mimalloc-object-lifecycle-queue-ssot.md"
role = "supporting"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-012 bounded object-backed lifecycle queue proof"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "mimalloc-page-free-list-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-008 page/free-list executable pilot (HakoAllocPageModel)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "mimalloc-size-class-bin-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-007 size-class/bin executable pilot (SizeClassBox)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "pattern-p5b-escape-design.md"
role = "supporting"
owner = "joinir-design-map.md"
precedence_parent = "joinir-design-map.md"
classification_basis = "C2-singleton-mz: Escape route P5b variable-step carrier design support"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "plugin-loadset-linking-ssot.md"
role = "supporting"
owner = "hakoruneup-release-distribution-ssot.md"
precedence_parent = "hakoruneup-release-distribution-ssot.md"
classification_basis = "C2-singleton-mz: (Active) plugin loadset linking design support (phase-295x)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "record-ergonomics-expansion-post-293x-ssot.md"
role = "supporting"
owner = "record-and-packed-array-lowering-ssot.md"
precedence_parent = "record-and-packed-array-lowering-ssot.md"
classification_basis = "C2-singleton-mz: (Active) record ergonomics expansion support plan"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "rustc-semir-adapter-tool-preflight-contract.md"
role = "supporting"
owner = "rustc-semir-internal-adapter-boundary.md"
precedence_parent = "rustc-semir-internal-adapter-boundary.md"
classification_basis = "C2-singleton-mz: (Design) rustc adapter tool-boundary preflight support"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "rustc-semir-binding-context-adapter-harness-design.md"
role = "supporting"
owner = "rustc-semir-internal-adapter-boundary.md"
precedence_parent = "rustc-semir-internal-adapter-boundary.md"
classification_basis = "C2-singleton-mz: (Design) first rustc semantic-adapter harness design support"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "scope-manager-condition-binding-adapter-wiring-design.md"
role = "supporting"
owner = "phi-lifecycle-ssot.md"
precedence_parent = "phi-lifecycle-ssot.md"
classification_basis = "C2-singleton-mz: (design) condition-binding identity adapter wiring/lookup support"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "selfhost-stageb-json-streaming-design.md"
role = "supporting"
owner = "selfhost-compiler-structure-ssot.md"
precedence_parent = "selfhost-compiler-structure-ssot.md"
classification_basis = "C2-singleton-mz: (Active) Stage-B emit program-json/mir-json memory-reduction design support"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "source-packed-array-autouse-pilot-ssot.md"
role = "supporting"
owner = "record-and-packed-array-lowering-ssot.md"
precedence_parent = "record-and-packed-array-lowering-ssot.md"
classification_basis = "C2-singleton-mz: PACKED-002 source PackedArray non-escaping auto-use pilot"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "mimalloc-first-executable-slice-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-006 first-executable slice selection"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mimalloc-hakorune-blueprint-task-breakdown-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: mimalloc upstream pin + blueprint port task slicing"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mimalloc-hakorune-joint-task-order-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: recommended joint mimalloc-port / core-dev task order"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mimalloc-migration-closeout-check-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: D208 closeout selecting the next safe inventory row after M214"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mimalloc-next-row-selection-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: D207 next single implementation-row selection"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mimalloc-osvm-release-capability-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-capability-taskboard-ssot.md"
precedence_parent = "mimalloc-capability-taskboard-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-048A OSVM release-capability inventory (OS release inactive)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mimalloc-page-queue-lifecycle-selection-ssot.md"
role = "status-ledger"
owner = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
precedence_parent = "mimalloc-lifecycle-rewrite-blueprint-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-010 lifecycle-aware page-selection owner selection"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mimalloc-post-huge-unreserve-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-035A facade huge-unreserve success/fail-fast closeout (base not in design root)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mimalloc-post-m215-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: D209 post-M215 thread-heap-owner inventory-wave closeout"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mimalloc-secure-entropy-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-capability-taskboard-ssot.md"
precedence_parent = "mimalloc-capability-taskboard-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-049A read-only secure-entropy/randomness inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mimalloc-substrate-representation-gap-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: MIMAP-004 substrate/representation gap ledger"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mir-builder-diet-flowplanner-boundary-ssot.md"
role = "status-ledger"
owner = "compiler-pipeline-thinning-ssot.md"
precedence_parent = "compiler-pipeline-thinning-ssot.md"
classification_basis = "C2-singleton-mz: temporary BoxShape cleanup boundary before MIMAP-021C"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mir-crate-split-prep-ssot.md"
role = "status-ledger"
owner = "compiler-pipeline-thinning-ssot.md"
precedence_parent = "compiler-pipeline-thinning-ssot.md"
classification_basis = "C2-singleton-mz: (provisional) `src/mir/` crate-split boundary inventory + entry-map tightening"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "mirbuilder-selfhost-checkpoint-roadmap-ssot.md"
role = "status-ledger"
owner = "selfhost-parser-mirbuilder-migration-order-ssot.md"
precedence_parent = "selfhost-parser-mirbuilder-migration-order-ssot.md"
classification_basis = "C2-singleton-mz: MirBuilder Rust-to-Hako selfhost checkpoint roadmap; provisional parent pending consultation on mirbuilder-rust-to-hako-converter-task-order-ssot.md"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "optimization-layer-roadmap-ssot.md"
role = "status-ledger"
owner = "optimization-task-card-os-ssot.md"
precedence_parent = "optimization-task-card-os-ssot.md"
classification_basis = "C2-singleton-mz: optimization layer roadmap"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "phi-carrier-join-id-vocabulary-decision.md"
role = "status-ledger"
owner = "phi-lifecycle-ssot.md"
precedence_parent = "phi-lifecycle-ssot.md"
classification_basis = "C2-singleton-mz: decision record: `CarrierVar.join_id` not a live lifecycle producer"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "phi-carrier-lifecycle-consumer-inventory.md"
role = "status-ledger"
owner = "phi-lifecycle-ssot.md"
precedence_parent = "phi-lifecycle-ssot.md"
classification_basis = "C2-singleton-mz: lifecycle-sensitive CarrierInfo PHI-carrier consumer inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "plan-dir-shallowing-ssot.md"
role = "status-ledger"
owner = "compiler-pipeline-thinning-ssot.md"
precedence_parent = "compiler-pipeline-thinning-ssot.md"
classification_basis = "C2-singleton-mz: `plan/` directory shallowing structure design (design-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "plan-mod-layout-ssot.md"
role = "status-ledger"
owner = "compiler-pipeline-thinning-ssot.md"
precedence_parent = "compiler-pipeline-thinning-ssot.md"
classification_basis = "C2-singleton-mz: `plan/mod.rs` declaration-section layering plan"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "planfrag-ssot-registry.md"
role = "status-ledger"
owner = "joinir-design-map.md"
precedence_parent = "joinir-design-map.md"
classification_basis = "C2-singleton-mz: (Draft) JoinIR plan/frag SSOT-location registry table"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "post-m213-next-lane-selection-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-singleton-mz: recommended next-lane selection after M192-M213 closeout"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "promoted-body-locals-lifecycle-inventory.md"
role = "status-ledger"
owner = "phi-lifecycle-ssot.md"
precedence_parent = "phi-lifecycle-ssot.md"
classification_basis = "C2-singleton-mz: `CarrierInfo.promoted_body_locals` ownership inventory"
sidecars = [
  "promoted-name-resolution-deny-closeout.md",
]
supersedes = []
superseded_by = ""
retire_when = "deny-closeout sidecar retires with this inventory when it is explicitly replaced"

[[documents]]
path = "recipe-file-naming-unification-ssot.md"
role = "status-ledger"
owner = "recipe-tree-and-parts-ssot.md"
precedence_parent = "recipe-tree-and-parts-ssot.md"
classification_basis = "C2-singleton-mz: historical Pattern-number→Recipe/Lego file-naming mapping ledger"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "repo-physical-structure-cleanup-ssot.md"
role = "status-ledger"
owner = "compiler-pipeline-thinning-ssot.md"
precedence_parent = "compiler-pipeline-thinning-ssot.md"
classification_basis = "C2-singleton-mz: (provisional) repo physical-structure BoxShape cleanup order"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "runtime-decl-manifest-v0.toml"
role = "status-ledger"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-mz: runtime decl symbol manifest (V0)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "runtime-decl-return-proof-fixture-v0.toml"
role = "status-ledger"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-mz: schema-fixture-only runtime decl return-proof fixture"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "static-data-manifest-v0.toml"
role = "status-ledger"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-mz: static data symbol manifest (V0)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "rust-lifecycle-context-facts-adapter-inventory.md"
role = "status-ledger"
owner = "rust-lifecycle-projection-ssot.md"
precedence_parent = "rust-lifecycle-projection-ssot.md"
classification_basis = "C2-singleton-mz: RustLifecycleFacts requirements inventory for MirBuilder context migration"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "selfhost-authority-facade-compat-inventory-ssot.md"
role = "status-ledger"
owner = "selfhost-lift-boundary-and-task-order-ssot.md"
precedence_parent = "selfhost-lift-boundary-and-task-order-ssot.md"
classification_basis = "C2-singleton-mz: selfhost file-level authority/adapter/facade/compat/shell responsibility inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "source-selfhost-family-manifest-split-ssot.md"
role = "status-ledger"
owner = "selfhost-lift-boundary-and-task-order-ssot.md"
precedence_parent = "selfhost-lift-boundary-and-task-order-ssot.md"
classification_basis = "C2-singleton-mz: Source Selfhost family guard active-index + history-ledger projections"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "source-selfhost-runner-and-route-task-breakdown-ssot.md"
role = "status-ledger"
owner = "selfhost-lift-boundary-and-task-order-ssot.md"
precedence_parent = "selfhost-lift-boundary-and-task-order-ssot.md"
classification_basis = "C2-singleton-mz: Source Selfhost design-stop recovery task breakdown + runner-role split"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "stage2-aot-fast-lane-crossing-inventory.md"
role = "status-ledger"
owner = "stage2-aot-native-thin-path-design-note.md"
precedence_parent = "stage2-aot-native-thin-path-design-note.md"
classification_basis = "C2-singleton-mz: stage2 AOT/native hot/cold crossing 3-bucket inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "stage2-collection-substrate-cleanup-ssot.md"
role = "status-ledger"
owner = "stage2-selfhost-and-hako-alloc-ssot.md"
precedence_parent = "stage2-selfhost-and-hako-alloc-ssot.md"
classification_basis = "C2-singleton-mz: (provisional) collection-substrate cleanup ordering before stage2+ perf"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "stage2-string-route-split-plan.md"
role = "status-ledger"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-mz: stage2 String hot-path next-wave split (search/slice vs concat) plan"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md"
role = "status-ledger"
owner = "stage2-selfhost-and-hako-alloc-ssot.md"
precedence_parent = "stage2-selfhost-and-hako-alloc-ssot.md"
classification_basis = "C2-singleton-mz: stage1→stage2-mainline entry gate + first optimization-wave task pack"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "task-lane-reconciliation-ssot.md"
role = "status-ledger"
owner = "compiler-task-map-ssot.md"
precedence_parent = "compiler-task-map-ssot.md"
classification_basis = "C2-singleton-mz: (paused) reconciliation separating three concurrently-discussed task lanes"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "trim-helper-carrier-lifecycle-inventory.md"
role = "status-ledger"
owner = "phi-lifecycle-ssot.md"
precedence_parent = "phi-lifecycle-ssot.md"
classification_basis = "C2-singleton-mz: `CarrierInfo.trim_helper` lifecycle ownership inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "typed-numeric-memory-substrate-task-order-ssot.md"
role = "status-ledger"
owner = "usize-semantic-foundation-ssot.md"
precedence_parent = "usize-semantic-foundation-ssot.md"
classification_basis = "C2-singleton-mz: task order for exact numeric types + memory substrate before mimalloc C-parity claims"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "variable-context-carrier-phi-lifecycle-inventory.md"
role = "status-ledger"
owner = "phi-lifecycle-ssot.md"
precedence_parent = "phi-lifecycle-ssot.md"
classification_basis = "C2-singleton-mz: carrier-sensitive `VariableContext.variable_map` consumer inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "variable-context-lifecycle-gap-inventory.md"
role = "status-ledger"
owner = "phi-lifecycle-ssot.md"
precedence_parent = "phi-lifecycle-ssot.md"
classification_basis = "C2-singleton-mz: MirBuilder `VariableContext` lifecycle migration gap inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "variable-context-returned-borrow-boundary-inventory.md"
role = "status-ledger"
owner = "phi-lifecycle-ssot.md"
precedence_parent = "phi-lifecycle-ssot.md"
classification_basis = "C2-singleton-mz: `VariableContext::variable_map(_mut)` returned-borrow lifecycle boundary inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "vm-known-limitations-ssot.md"
role = "status-ledger"
owner = "vm-fallback-lane-separation-ssot.md"
precedence_parent = "vm-fallback-lane-separation-ssot.md"
classification_basis = "C2-singleton-mz: known/bounded VM limitations ledger (must not silently affect LLVM/EXE acceptance)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "pyvm-retreat-ssot.md"
role = "superseded"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-mz: Historical; Moved to `design/archive/pyvm-retreat-ssot.md`"
sidecars = []
supersedes = []
superseded_by = "archive/pyvm-retreat-ssot.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "recipe-first-entry-contract-history.md"
role = "superseded"
owner = "recipe-first-entry-contract-ssot.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-mz: Historical; Moved to `design/archive/recipe-first-entry-contract-history.md`"
sidecars = []
supersedes = []
superseded_by = "archive/recipe-first-entry-contract-history.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "recipe-first-migration-phased-plan-proposal.md"
role = "superseded"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-mz: Historical; Moved to `design/archive/recipe-first-migration-phased-plan-proposal.md`"
sidecars = []
supersedes = []
superseded_by = "archive/recipe-first-migration-phased-plan-proposal.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "route-physical-path-legacy-lane-ssot.md"
role = "superseded"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-mz: Historical; Moved to `design/archive/route-physical-path-legacy-lane-ssot.md`"
sidecars = []
supersedes = []
superseded_by = "archive/route-physical-path-legacy-lane-ssot.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "stage2-aot-native-external-consultation-question.md"
role = "superseded"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-mz: Historical consultation, adopted-into-ssot; current truth is the thin-path design note"
sidecars = []
supersedes = []
superseded_by = "stage2-aot-native-thin-path-design-note.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "parser-extensions-param-implements-interface-generic-ssot.md"
role = "authority"
owner = "selfhost-language-v1-freeze-ssot.md"
precedence_parent = "selfhost-language-v1-freeze-ssot.md"
classification_basis = "C2-singleton-mz: (Provisional) minimal parser-extension acceptance set for `.hako` port (param type / implements / interface generic)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "provider-abi-shim-boundary-ssot.md"
role = "authority"
owner = "provider-abi-v1-ssot.md"
precedence_parent = "provider-abi-v1-ssot.md"
classification_basis = "C2-singleton-mz: (Active) provider ABI / LD_PRELOAD shim ownership boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "result-option-expected-type-diagnostics-ssot.md"
role = "authority"
owner = "result-option-prelude-diagnostics-ssot.md"
precedence_parent = "result-option-prelude-diagnostics-ssot.md"
classification_basis = "C2-singleton-mz: RESULT-002D generic-enum expected-type diagnostics"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "result-option-missing-arm-diagnostics-ssot.md"
role = "authority"
owner = "result-option-prelude-diagnostics-ssot.md"
precedence_parent = "result-option-prelude-diagnostics-ssot.md"
classification_basis = "C2-singleton-mz: RESULT-002A prelude enum missing-arm diagnostics"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "result-option-payload-diagnostics-ssot.md"
role = "authority"
owner = "result-option-prelude-diagnostics-ssot.md"
precedence_parent = "result-option-prelude-diagnostics-ssot.md"
classification_basis = "C2-singleton-mz: RESULT-002B prelude enum payload diagnostics"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "rust-lifecycle-facts-vocab-v0.md"
role = "authority"
owner = "rust-lifecycle-projection-ssot.md"
precedence_parent = "rust-lifecycle-projection-ssot.md"
classification_basis = "C2-singleton-mz: passive Rust-side lifecycle facts vocabulary for Rust-to-Hako migration"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "type-abi-catalog-planning-spine-ssot.md"
role = "authority"
owner = "type-abi-view-and-plan-stamp-ssot.md"
precedence_parent = "type-abi-view-and-plan-stamp-ssot.md"
classification_basis = "C2-singleton-mz: TypeAbiCatalog thin planning-query spine (not central truth)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "type-abi-naming-and-box-descriptor-ssot.md"
role = "authority"
owner = "type-abi-view-and-plan-stamp-ssot.md"
precedence_parent = "type-abi-view-and-plan-stamp-ssot.md"
classification_basis = "C2-singleton-mz: naming boundary between TypeBox ABI v2 / historical TypeAbi* / BoxDescriptor"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "type-abi-route-descriptor-plane-ssot.md"
role = "authority"
owner = "type-abi-view-and-plan-stamp-ssot.md"
precedence_parent = "type-abi-view-and-plan-stamp-ssot.md"
classification_basis = "C2-singleton-mz: (Active) allocator/provider route-descriptor application of the Type ABI view contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "typed-array-element-checks-ssot.md"
role = "authority"
owner = "typed-array-method-contract-ssot.md"
precedence_parent = "typed-array-method-contract-ssot.md"
classification_basis = "C2-singleton-mz: ARRAY-002B typed local Array<T> direct element checks"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "typed-array-inference-failfast-ssot.md"
role = "authority"
owner = "typed-array-method-contract-ssot.md"
precedence_parent = "typed-array-method-contract-ssot.md"
classification_basis = "C2-singleton-mz: ARRAY-002C unsupported Array-inference fail-fast diagnostics"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "typed-array-literal-context-ssot.md"
role = "authority"
owner = "typed-array-method-contract-ssot.md"
precedence_parent = "typed-array-method-contract-ssot.md"
classification_basis = "C2-singleton-mz: ARRAY-001 typed-context array-literal lowering"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "allocator-replacement-hook-boundary-ssot.md"
role = "authority"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: phase-293x allocator replacement hook boundary stop-line before any process allocator replacement"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "arc-retirement-and-ownership-substrate-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: Arc retirement / ownership substrate boundary contract (design side-lane, does not change active AOT lane)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "array-result-option-canonical-surface-ssot.md"
role = "authority"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: canonical language surface for Array<T>/PackedArray<T>/Result/Option/enum variants"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "backend-recipe-route-profile-ssot.md"
role = "authority"
owner = "llvm-line-ownership-and-boundary-ssot.md"
precedence_parent = "llvm-line-ownership-and-boundary-ssot.md"
classification_basis = "C2-singleton-al: backend-zero .hako policy owner vs transport-only C substrate responsibility split fixed as route-profile canonical object"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "birth-placement-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-al: Birth/Placement outcome as source of truth; .hako owner -> MIR canonical contract -> Rust birth backend responsibility fixed"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "block-expressions-and-condition-blocks-ssot.md"
role = "authority"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: language semantics for BlockExpr + compiler condition-entry normalization (Phase B1/B2 landed)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "box-callable-registry-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: final Box callable ownership model for builtin/plugin/user boxes, Type ABI projection, route plan generation"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "build-lane-separation-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: Build/Kernel daily-vs-maintenance lane separation boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "canonical-lowering-visibility-ssot.md"
role = "authority"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-al: .hako owner -> MIR canonical reading -> concrete lowering -> Rust executor visibility lock"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "compiler-object-final-shape-ssot.md"
role = "authority"
owner = "compiler-pipeline-ssot.md"
precedence_parent = "compiler-pipeline-ssot.md"
classification_basis = "C2-singleton-al: final compiler object-shape boundary before selfhost MIRBuilder growth"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "contract-region-v0-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: common contract-region envelope for fast memory and future fastpath profiles"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "coreplan-flowbox-interface-ssot.md"
role = "authority"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: minimal CorePlan FlowBox interface (ports / ExitMap / join payload)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "coreplan-unknown-loop-strategy-ssot.md"
role = "authority"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: CorePlan/JoinIR unknown-loop decompose-and-compose strategy law"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "de-rust-kernel-authority-cutover-ssot.md"
role = "authority"
owner = "kernel-replacement-axis-ssot.md"
precedence_parent = "kernel-replacement-axis-ssot.md"
classification_basis = "C2-singleton-al: kernel meaning/policy owner cutover to .hako (not immediate Rust delete) policy"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "de-rust-scope-decision-ssot.md"
role = "authority"
owner = "execution-lanes-and-axis-separation-ssot.md"
precedence_parent = "execution-lanes-and-axis-separation-ssot.md"
classification_basis = "C2-singleton-al: fixes de-rust 'done' declaration scope (non-plugin / plugin)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "de-rust-stage-and-owner-axis-ssot.md"
role = "authority"
owner = "execution-lanes-and-axis-separation-ssot.md"
precedence_parent = "execution-lanes-and-axis-separation-ssot.md"
classification_basis = "C2-singleton-al: separates stage0/1/2 execution axis, K0/K1/K2 build/runtime axis, owner/substrate axis to prevent stop-line mixing"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "de-rust-zero-buildability-contract-ssot.md"
role = "authority"
owner = "kernel-replacement-axis-ssot.md"
precedence_parent = "kernel-replacement-axis-ssot.md"
classification_basis = "C2-singleton-al: defines 0rust as Rust-meaning-owner-zero while keeping a Rust build/bootstrap route (contract)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "delegation-no-inheritance-ssot.md"
role = "authority"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: canonical behavior-reuse surface replacing inheritance with explicit delegation"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "effect-classification-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: MIR/PlanFrag effect classification and allowed-transform law (optimization/RC-insert/observation safe region)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "enum-sum-and-generic-surface-ssot.md"
role = "authority"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: first-class enum/sum declarations and generic surface syntax (provisional SSOT)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "exitkind-cleanup-effect-contract-ssot.md"
role = "authority"
owner = "effect-classification-ssot.md"
precedence_parent = "effect-classification-ssot.md"
classification_basis = "C2-singleton-al: ExitKind/cleanup/effect contract across JoinIR/PlanFrag/MIR (semantics preservation)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "fastpath-eligibility-resolver-ssot.md"
role = "authority"
owner = "current-optimization-mechanisms-ssot.md"
precedence_parent = "current-optimization-mechanisms-ssot.md"
classification_basis = "C2-singleton-al: demand-driven recursive fastpath eligibility law for exact-AOT lowering"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "feature-helper-boundary-ssot.md"
role = "authority"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: plan/features helper boundary (join/exit/phi/carrier single-location rule)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "generic-memory-dce-observer-owner-contract-ssot.md"
role = "authority"
owner = "current-optimization-mechanisms-ssot.md"
precedence_parent = "current-optimization-mechanisms-ssot.md"
classification_basis = "C2-singleton-al: first generic-memory DCE observer/owner contract (Load/Store ownership vs local-field lane vs observer lane)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hako-inspect-scope-dump-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: scope-wide MIR/LLVM-IR/asm dump is a tool query not source syntax (boundary decision)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hako-module-cache-build-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: module-level objectification + link + 3-layer cache design fixed"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "hako-thread-substrate-boundary-ssot.md"
role = "authority"
owner = "thread-and-tls-capability-ssot.md"
precedence_parent = "thread-and-tls-capability-ssot.md"
classification_basis = "C2-singleton-al: .hako source concurrency semantics, runtime OS-thread substrate, allocator threading evidence, benchmark-claim boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "handle-cache-metal-helper-contract-ssot.md"
role = "authority"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-al: phase-29ct metal helper contract lock: handle_cache.rs responsibilities/invariants/non-goals"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "joinir-observation-layer-ssot.md"
role = "authority"
owner = "joinir-design-map.md"
precedence_parent = "joinir-design-map.md"
classification_basis = "C2-singleton-al: JoinIR is an observation layer (foundational role fix)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "joinir-plan-frag-ssot.md"
role = "authority"
owner = "joinir-design-map.md"
precedence_parent = "joinir-design-map.md"
classification_basis = "C2-singleton-al: Plan/Frag responsibilities, prohibitions, freeze points"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "joinir-planner-required-gates-ssot.md"
role = "authority"
owner = "planner-entry-guards-ssot.md"
precedence_parent = "planner-entry-guards-ssot.md"
classification_basis = "C2-singleton-al: planner-required gate entry / TSV contract / allow_rc handling"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "lowering-plan-json-v0-ssot.md"
role = "authority"
owner = "hotline-core-method-contract-ssot.md"
precedence_parent = "hotline-core-method-contract-ssot.md"
classification_basis = "C2-singleton-al: backend-facing LoweringPlan JSON v0 contract for pure-first ny-llvmc"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review explicitly replaces this authority"

[[documents]]
path = "COREPLAN_GENERAL_LOOP_DECOMPOSITION_INQUIRY.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: external-review inquiry (questionnaire) on CorePlan unknown-loop decompose-vs-general-loop"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "ai-plan-review-checklist-ssot.md"
role = "supporting"
owner = "compiler-expressivity-first-policy.md"
precedence_parent = "compiler-expressivity-first-policy.md"
classification_basis = "C2-singleton-al: LLM plan-drift review checklist for JoinIR/planner-required recipe-first work"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "allocator-provider-boundary-v0-ssot.md"
role = "supporting"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-singleton-al: allocator provider boundary vocabulary before allocator replacement"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "allocator-provider-lightweight-doc-sync-policy-ssot.md"
role = "supporting"
owner = "current-docs-update-policy-ssot.md"
precedence_parent = "current-docs-update-policy-ssot.md"
classification_basis = "C2-singleton-al: allocator provider lane lightweight docs-sync policy after M86"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "allocator-provider-runtime-diagnostic-module-boundaries-ssot.md"
role = "supporting"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-singleton-al: M98B allocator provider runtime diagnostic module boundaries"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "array-repr-ssot.md"
role = "supporting"
owner = "raw-array-substrate-ssot.md"
precedence_parent = "raw-array-substrate-ssot.md"
classification_basis = "C2-singleton-al: ArrayRepr bridge between public ArrayBox facade and DirectArray family storage substrate"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "array-runtime-single-thread-store-backend-ssot.md"
role = "supporting"
owner = "raw-array-substrate-ssot.md"
precedence_parent = "raw-array-substrate-ssot.md"
classification_basis = "C2-singleton-al: provisional ArrayBox runtime single-thread store backend boundary for exact-EXE diagnostics"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "array-text-session-route-ssot.md"
role = "supporting"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-al: array-text string indexOf compat/export split + selected-route truth + session hot-path lowering"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "arraybox-json-v0-backend-guard-ssot.md"
role = "supporting"
owner = "raw-array-substrate-ssot.md"
precedence_parent = "raw-array-substrate-ssot.md"
classification_basis = "C2-singleton-al: ARRAY-002D ArrayBox JSON v0 backend guard"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "backend-legacy-preservation-and-archive-ssot.md"
role = "supporting"
owner = "code-retirement-history-policy-ssot.md"
precedence_parent = "code-retirement-history-policy-ssot.md"
classification_basis = "C2-singleton-al: preservation + external archive required before backend-zero deletion (application of retirement policy)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "balanced-depth-scan-analysis-view-ssot.md"
role = "supporting"
owner = "condition-observation-ssot.md"
precedence_parent = "condition-observation-ssot.md"
classification_basis = "C2-singleton-al: BalancedDepthScan analysis-only view contract (no AST rewrite) for find_balanced_*"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "block-expr-b3-sugar-decision.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: superseded B3 plain-BlockExpr sugar; branch-visible binding requires a distinct future scope owner"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "box-identity-view-allocation-design-note.md"
role = "supporting"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: BoxBase::new / StringViewBox::new optimization premise as in-repo contract note (draft)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "boxcount-new-box-addition-checklist-ssot.md"
role = "supporting"
owner = "compiler-expressivity-first-policy.md"
precedence_parent = "compiler-expressivity-first-policy.md"
classification_basis = "C2-singleton-al: JoinIR planner-required BoxCount new-box addition checklist"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "brand-constructor-unwrap-policy-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: BRAND-002 Stage1 brand constructor/unwrap using existing call syntax"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "brand-mismatch-checker-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: BRAND-003 Stage1 conservative brand mismatch checker"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "brand-parser-capsule-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: BRAND-001 Stage0 brand parser capsule (syntax + metadata transport only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "capsule-value-result-contract-ssot.md"
role = "supporting"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: hako_alloc result-capsule ValueAggregate contract before MIR/backend implementation"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "cfg-aware-typed-field-residence-ssot.md"
role = "supporting"
owner = "current-optimization-mechanisms-ssot.md"
precedence_parent = "current-optimization-mechanisms-ssot.md"
classification_basis = "C2-singleton-al: provisional CFG-aware MIR typed-field residence contract for hot scalar field access"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "cleanupwrap-cleanup-region-boundary-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: CleanupWrap + cleanup region boundary (CorePlan/Recipe-first feature)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "condblockview-desugar-consult.md"
role = "supporting"
owner = "condition-observation-ssot.md"
precedence_parent = "condition-observation-ssot.md"
classification_basis = "C2-singleton-al: design sanity-check consult for 'condition is always a block' internally"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "cond-block-view-prelude-ssot.md"
role = "supporting"
owner = "condition-observation-ssot.md"
precedence_parent = "condition-observation-ssot.md"
classification_basis = "C2-singleton-al: CondBlockView condition-prelude analysis-only view contract (no AST rewrite)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "condition-binding-promoted-identity-proof-probe.md"
role = "supporting"
owner = "condition-observation-ssot.md"
precedence_parent = "condition-observation-ssot.md"
classification_basis = "C2-singleton-al: read-only proof probe for promoted condition-binding identity"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "condition-binding-resolution-rewrite-design.md"
role = "supporting"
owner = "condition-observation-ssot.md"
precedence_parent = "condition-observation-ssot.md"
classification_basis = "C2-singleton-al: promoted condition-binding identity consumption design after proof probe"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "condprofile-ssot.md"
role = "supporting"
owner = "condition-observation-ssot.md"
precedence_parent = "condition-observation-ssot.md"
classification_basis = "C2-singleton-al: Condition Profile (CondProfile) skeleton types"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "contract-syntax-metadata-capsule-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: CONTRACT-002 Stage0 contract syntax metadata capsule"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "control-tree-nested-loop-depth-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: JoinIR strict-mode StepTree nested-loop depth guard (max_loop_depth)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "coreloop-composer-v0-v1-boundary-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: CoreLoopComposer v0/v1 responsibility boundary (value_join_needed selection)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "coreloop-exitmap-composition-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: ExitMap/Cleanup/ValueJoin composition rules for CoreLoop skeleton"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "coreloop-generic-loop-v0-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: CorePlan/FlowBox generic structured loop v0 (decompose & compose)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "coreloop-loopframe-v1-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: CorePlan LoopFrame v1 (Loop structural box with Break/Continue depth)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "coreloop-stepmode-inline-in-body-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: StepPlacement/StepMode InlineInBody (no-rewrite loop expressivity)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "coreplan-compat-normalizer-legoization-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: COREPLAN-FOUND-000/001 compat-normalizer lego-ization selection + first guard"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "declared-uses-capability-plan-mapping-ssot.md"
role = "supporting"
owner = "rune-profile-effect-capability-plan-ssot.md"
precedence_parent = "rune-profile-effect-capability-plan-ssot.md"
classification_basis = "C2-singleton-al: USES-002A declared `uses` metadata -> MIR CapabilityPlan mapping"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "delegation-exposes-lowering-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: DEL-003 Stage1 delegate-exposes lowering into forwarding methods"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "delegation-parser-capsule-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: DEL-002 Stage0 delegate syntax metadata capsule"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "exitkind-unwind-reservation-ssot.md"
role = "supporting"
owner = "coreplan-flowbox-interface-ssot.md"
precedence_parent = "coreplan-flowbox-interface-ssot.md"
classification_basis = "C2-singleton-al: reserve ExitKind::Unwind in CorePlan/FlowBox (docs-first reservation)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "executable-trim-route-lowering-implementation-design.md"
role = "supporting"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-al: first executable trim-route lowering seam design after identity proof"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "fastmem-layout-table-contract-v0-ssot.md"
role = "supporting"
owner = "contract-region-v0-ssot.md"
precedence_parent = "contract-region-v0-ssot.md"
classification_basis = "C2-singleton-al: memory-profile layout/table contract resolver for MIR-FMEM-008B"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "flowbox-fallback-observability-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: FlowBox fallback observability via flowbox/freeze codes (strict/dev only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "flowbox-observability-tags-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: observability tag vocabulary for CorePlan/FlowBox (strict/dev only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "frontend-owner-proof-index.md"
role = "supporting"
owner = "selfhost-lift-boundary-and-task-order-ssot.md"
precedence_parent = "selfhost-lift-boundary-and-task-order-ssot.md"
classification_basis = "C2-singleton-al: frontend/bootstrap owner buckets + smallest canonical proofs that reopen them (proof index)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "generic-arity-checker-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: GEN-002 Stage1 generic type-argument arity checking"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "generic-loop-v1-shape-ssot.md"
role = "supporting"
owner = "generic-loop-v1-acceptance-by-recipe-ssot.md"
precedence_parent = "generic-loop-v1-acceptance-by-recipe-ssot.md"
classification_basis = "C2-singleton-al: JoinIR generic_loop_v1 ShapeId detection (hint-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "generic-type-annotation-metadata-capsule-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: GEN-001 Stage0 generic type annotation metadata capsule"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "guard-let-pattern-sugar-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: GUARDLET-001 minimal guard-let enum variant sugar"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-alloc-atomic-bitmap-pilot-ssot.md"
role = "supporting"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: first bounded atomic-bitmap pilot fact after segment-map mutation"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-alloc-optional-process-allocator-replacement-proposal-ssot.md"
role = "supporting"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-singleton-al: MIMAP-425A optional process allocator replacement proposal (records boundary, not executed)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-alloc-osvm-page-source-pilot-ssot.md"
role = "supporting"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: bounded OSVM/page-source pilot fact after atomic-bitmap pilot"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-alloc-remote-free-retry-bound-ssot.md"
role = "supporting"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: MIMAP-039A hako_alloc remote-free retry bound cleanup"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-alloc-worker-tls-pilot-ssot.md"
role = "supporting"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: MIMAP-350A bounded worker/TLS pilot after OSVM/page-source pilot"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-app-front-boundary-template-ssot.md"
role = "supporting"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: structure/template for .hako app fronts validated through EXE/AOT"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-check-mir-observation-boundary-ssot.md"
role = "supporting"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: boundary between hako_check source perf-surface and MIR-level method-shape observation"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-lifecycle-emitter-probe-v0.md"
role = "supporting"
owner = "de-rust-compiler-thin-rust-roadmap-ssot.md"
precedence_parent = "de-rust-compiler-thin-rust-roadmap-ssot.md"
classification_basis = "C2-singleton-al: first bounded lifecycle-aware emission probe"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-lifecycle-plan-vocab-v0.md"
role = "supporting"
owner = "de-rust-compiler-thin-rust-roadmap-ssot.md"
precedence_parent = "de-rust-compiler-thin-rust-roadmap-ssot.md"
classification_basis = "C2-singleton-al: passive Hako-side lifecycle plan vocabulary for Rust-to-Hako migration"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-lifecycle-resolver-readonly-skeleton.md"
role = "supporting"
owner = "de-rust-compiler-thin-rust-roadmap-ssot.md"
precedence_parent = "de-rust-compiler-thin-rust-roadmap-ssot.md"
classification_basis = "C2-singleton-al: diagnostic-only lifecycle resolver read-only skeleton"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-lifecycle-verifier-result-vocab-v0.md"
role = "supporting"
owner = "de-rust-compiler-thin-rust-roadmap-ssot.md"
precedence_parent = "de-rust-compiler-thin-rust-roadmap-ssot.md"
classification_basis = "C2-singleton-al: passive verifier result vocabulary for Rust-to-Hako lifecycle migration"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-optimization-toolbox-usability-ssot.md"
role = "supporting"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: optimization toolbox entry-point reference (hako_check, MIR shape, exact-EXE measurement, row guards)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "hako-mirbuilder-migration-phase0-entry-contract-ssot.md"
role = "supporting"
owner = "selfhost-parser-mirbuilder-migration-order-ssot.md"
precedence_parent = "selfhost-parser-mirbuilder-migration-order-ssot.md"
classification_basis = "C2-singleton-al: .hako mirbuilder migration Phase-0 entrypoints/contracts/fail-fast tags (draft, staged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "kernel-authority-cutover-external-consultation-question.md"
role = "supporting"
owner = "de-rust-kernel-authority-cutover-ssot.md"
precedence_parent = "de-rust-kernel-authority-cutover-ssot.md"
classification_basis = "C2-singleton-al: external consultation prompt for remaining kernel-family migration to .hako"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "kernel-observability-and-two-stage-pilot-ssot.md"
role = "supporting"
owner = "kernel-implementation-phase-plan-ssot.md"
precedence_parent = "kernel-implementation-phase-plan-ssot.md"
classification_basis = "C2-singleton-al: kernel perf-observe vocabulary + .hako pilot comparison order"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "known-enum-underscore-exhaustiveness-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: RESULT-002C known-enum exhaustiveness underscore rules"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "local-type-annotation-metadata-capsule-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: LOCALTYPE-001 Stage0 local type annotation metadata transport"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "loop-cond-break-continue-ssot.md"
role = "supporting"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: strict/dev-only planner path for loop(cond) with multiple exit-if break/continue (no AST rewrite)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "loop-range-parser-capsule-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: LOOP-002 Stage0 LoopRange parser capsule"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "loop-range-stage1-route-ssot.md"
role = "supporting"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: LOOP-003A Stage1 LoopRange route decision"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "owner-family review replaces or supersedes this support note"

[[documents]]
path = "abi-export-inventory.md"
role = "status-ledger"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-al: phase-29ct V0 inventory of kernel/plugin ABI export surface (mainline/facade/compat/adapter split)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "abi-export-manifest-v0.toml"
role = "status-ledger"
owner = "value-repr-and-abi-manifest-ssot.md"
precedence_parent = "value-repr-and-abi-manifest-ssot.md"
classification_basis = "C2-singleton-al: V0 ABI export manifest data (adapter-default rows source consumed by AbiAdapterRegistryBox)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "allocator-provider-combined-dry-run-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-singleton-al: M70 combined hook/provider dry-run report"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "allocator-provider-current-task-breakdown-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-singleton-al: post-M101 allocator provider/replacement-hook task breakdown"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "allocator-provider-implementation-family-selection-future-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-singleton-al: long-lived future direction for explicit allocator-provider family selection"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "allocator-provider-post-m101-implementation-ladder-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-singleton-al: post-M101 allocator provider activation implementation ladder"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "allocator-provider-readiness-preflight-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-singleton-al: M69 allocator provider readiness preflight shape"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "arc-retirement-family-gate-and-first-family-ssot.md"
role = "status-ledger"
owner = "arc-retirement-and-ownership-substrate-ssot.md"
precedence_parent = "arc-retirement-and-ownership-substrate-ssot.md"
classification_basis = "C2-singleton-al: ARC-RETIRE-006..018 family gate through first family producer cutover order"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "array-lane-extension-roadmap-ssot.md"
role = "status-ledger"
owner = "raw-array-substrate-ssot.md"
precedence_parent = "raw-array-substrate-ssot.md"
classification_basis = "C2-singleton-al: task roadmap for extending Array residence lanes without making ArrayBox the perf substrate"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "array-map-owner-and-ring-cutover-ssot.md"
role = "status-ledger"
owner = "raw-array-substrate-ssot.md"
precedence_parent = "raw-array-substrate-ssot.md"
classification_basis = "C2-singleton-al: ArrayBox/MapBox current owner truth + ring/owner cutover order toward 0rust"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "ast-cleanup-before-localtype-ssot.md"
role = "status-ledger"
owner = "compiler-pipeline-thinning-ssot.md"
precedence_parent = "compiler-pipeline-thinning-ssot.md"
classification_basis = "C2-singleton-al: AST legacy-residue cleanup rows before LOCALTYPE-001"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "backend-owner-cutover-ssot.md"
role = "status-ledger"
owner = "llvm-line-ownership-and-boundary-ssot.md"
precedence_parent = "llvm-line-ownership-and-boundary-ssot.md"
classification_basis = "C2-singleton-al: provisional MIR->backend owner structure-first recut plan"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "box-object-model-replacement-map-ssot.md"
role = "status-ledger"
owner = "arc-retirement-and-ownership-substrate-ssot.md"
precedence_parent = "arc-retirement-and-ownership-substrate-ssot.md"
classification_basis = "C2-singleton-al: ARC-RETIRE-005A..005D Box object-model replacement map"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "build-crate-split-plan-ssot.md"
role = "status-ledger"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: build-time reduction crate-split planning"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "c-abi-shim-responsibility-cleanup-backlog-ssot.md"
role = "status-ledger"
owner = "hako-runtime-c-abi-cutover-order-ssot.md"
precedence_parent = "hako-runtime-c-abi-cutover-order-ssot.md"
classification_basis = "C2-singleton-al: C ABI shim responsibility cleanup backlog (BoxShape-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "collection-set-map-fst-task-breakdown-ssot.md"
role = "status-ledger"
owner = "raw-map-substrate-ssot.md"
precedence_parent = "raw-map-substrate-ssot.md"
classification_basis = "C2-singleton-al: Map/Set/HashMap naming + FST placement + ordering vs mimalloc port (task breakdown)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "compiler-cleanup-sidecar-task-breakdown-ssot.md"
role = "status-ledger"
owner = "compiler-cleanliness-campaign-ssot.md"
precedence_parent = "compiler-cleanliness-campaign-ssot.md"
classification_basis = "C2-singleton-al: BoxShape-only cleanup task breakdown discovered during mimalloc blueprint lane"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "concat3-array-store-placement-window-ssot.md"
role = "status-ledger"
owner = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
precedence_parent = "string-canonical-mir-corridor-and-placement-pass-ssot.md"
classification_basis = "C2-singleton-al: provisional concat3_hhh->array.set->trailing length() compiler-local placement window"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "concurrency-async-pre-selfhost-ssot.md"
role = "status-ledger"
owner = "thread-and-tls-capability-ssot.md"
precedence_parent = "thread-and-tls-capability-ssot.md"
classification_basis = "C2-singleton-al: execution ledger for pre-selfhost VM+LLVM concurrency stabilization"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "concurrency-boundary-migration-taskboard-ssot.md"
role = "status-ledger"
owner = "thread-and-tls-capability-ssot.md"
precedence_parent = "thread-and-tls-capability-ssot.md"
classification_basis = "C2-singleton-al: implementation-row taskboard for the concurrency Boundary model"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "condprofile-migration-plan-ssot.md"
role = "status-ledger"
owner = "condition-observation-ssot.md"
precedence_parent = "condition-observation-ssot.md"
classification_basis = "C2-singleton-al: CondProfile migration plan (ConditionShape shrink plan)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "coreplan-migration-done-criteria-ssot.md"
role = "status-ledger"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: done criteria for CorePlan migration"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "coreplan-migration-roadmap-ssot.md"
role = "status-ledger"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: JoinIR->PlanFrag->CorePlan migration roadmap"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "coreplan-purity-stage1-ssot.md"
role = "status-ledger"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: CorePlan purity Stage-1 (strict/dev fallback visibility target)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "coreplan-purity-stage2-ssot.md"
role = "status-ledger"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: CorePlan purity Stage-2 (release-default CorePlan sole structural SSOT target)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "de-rust-lane-map-ssot.md"
role = "status-ledger"
owner = "execution-lanes-and-axis-separation-ssot.md"
precedence_parent = "execution-lanes-and-axis-separation-ssot.md"
classification_basis = "C2-singleton-al: de-rust tasks fixed in lane A/B/C (lane map)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "de-rust-master-task-map-ssot.md"
role = "status-ledger"
owner = "execution-lanes-and-axis-separation-ssot.md"
precedence_parent = "execution-lanes-and-axis-separation-ssot.md"
classification_basis = "C2-singleton-al: overall de-rust completion order (lane A/B/C + 29cc orchestration) + done判定"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "de-rust-runtime-meaning-decision-red-inventory-ssot.md"
role = "status-ledger"
owner = "de-rust-post-g1-runtime-plan-ssot.md"
precedence_parent = "de-rust-post-g1-runtime-plan-ssot.md"
classification_basis = "C2-singleton-al: red inventory of Rust-side meaning-decision points in runtime lane"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "derived-to-native-hako-artifact-model-ssot.md"
role = "status-ledger"
owner = "de-rust-compiler-thin-rust-roadmap-ssot.md"
precedence_parent = "de-rust-compiler-thin-rust-roadmap-ssot.md"
classification_basis = "C2-singleton-al: two-stage Rust->generated-Hako->native-Hako artifact migration model"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "entry-name-map-ssot.md"
role = "status-ledger"
owner = "planner-entry-guards-ssot.md"
precedence_parent = "planner-entry-guards-ssot.md"
classification_basis = "C2-singleton-al: planner-first RuleId -> human-readable label name map"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "environment-variables-inventory-ssot.md"
role = "status-ledger"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: NYASH_* env-var inventory (unknown/unused/duplicate) for delete/deprecate decisions"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "fastmem-source-syntax-smoke-taxonomy-ssot.md"
role = "status-ledger"
owner = "contract-region-v0-ssot.md"
precedence_parent = "contract-region-v0-ssot.md"
classification_basis = "C2-singleton-al: fastmem source-syntax smoke row taxonomy + discovery rules"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "fastmem-verified-direct-default-retirement-ssot.md"
role = "status-ledger"
owner = "contract-region-v0-ssot.md"
precedence_parent = "contract-region-v0-ssot.md"
classification_basis = "C2-singleton-al: phased retirement plan for dedicated FastMemory AST lowering in MIRBuilder"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "flowbox-adopt-tag-migration-ssot.md"
role = "status-ledger"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: migrate adopt/fallback observability to FlowBox schema (strict/dev)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "flowbox-tag-coverage-map-ssot.md"
role = "status-ledger"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: FlowBox tag coverage map"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "guard-manifest-migration-ssot.md"
role = "status-ledger"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: manifest-backed guard/proof-app runner migration + no-growth policy"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "hako-alloc-backend-matcher-no-growth-closeout-ssot.md"
role = "status-ledger"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: MIMAP-354A backend-matcher no-growth closeout; owning base row absent from design root -> independent status-ledger per Q2 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "hako-alloc-execution-seam-summary-closeout-ssot.md"
role = "status-ledger"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: MIMAP-356A allocator execution-seam summary closeout; owning base row absent -> independent status-ledger per Q2 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "hako-alloc-hook-install-preflight-plan-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-singleton-al: MIMAP-423A hook-install preflight plan (planning-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "hako-alloc-options-inventory-ssot.md"
role = "status-ledger"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: M214 read-only allocator options/defaults inventory surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "hako-alloc-thread-heap-owner-inventory-ssot.md"
role = "status-ledger"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: M215 read-only thread-heap owner-token inventory surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "hako-alloc-wide-report-argument-cleanup-ssot.md"
role = "status-ledger"
owner = "hako-alloc-policy-state-contract-ssot.md"
precedence_parent = "hako-alloc-policy-state-contract-ssot.md"
classification_basis = "C2-singleton-al: ARG-DATA-001 BoxShape cleanup before MIMAP-454A to reduce wide-report/positional-argument pressure"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "hako-mimalloc-performance-parity-ssot.md"
role = "status-ledger"
owner = "mimalloc-capability-taskboard-ssot.md"
precedence_parent = "mimalloc-capability-taskboard-ssot.md"
classification_basis = "C2-singleton-al: task order for making .hako mimalloc port comparable with C mimalloc performance"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "hakorune-provider-package-abi-v1-future-ssot.md"
role = "status-ledger"
owner = "hako-runtime-c-abi-cutover-order-ssot.md"
precedence_parent = "hako-runtime-c-abi-cutover-order-ssot.md"
classification_basis = "C2-singleton-al: future Hakorune provider package / DLL shared-library ABI plan"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "hakorune-stage-term-existing-name-migration-inventory.md"
role = "status-ledger"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-singleton-al: STAGE-TERM-EXISTING-NAME-INVENTORY-001 classification-only inventory of stage names (natural parent hakorune-naming-and-rename-task-order-ssot.md is held for consultation; parented to INDEX.md provisionally)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "joinir-port-task-pack-ssot.md"
role = "status-ledger"
owner = "de-rust-compiler-thin-rust-roadmap-ssot.md"
precedence_parent = "de-rust-compiler-thin-rust-roadmap-ssot.md"
classification_basis = "C2-singleton-al: lane A JoinIR port fixed order (1 blocker=1 fixture=1 smoke=1 commit)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "json-v0-bridge-lowering-split-ssot.md"
role = "status-ledger"
owner = "selfhost-parser-mirbuilder-migration-order-ssot.md"
precedence_parent = "selfhost-parser-mirbuilder-migration-order-ssot.md"
classification_basis = "C2-singleton-al: json_v0_bridge/lowering.rs structure-cleanup responsibility map + split state (behavior-neutral)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "json-v0-route-map-ssot.md"
role = "status-ledger"
owner = "selfhost-lift-boundary-and-task-order-ssot.md"
precedence_parent = "selfhost-lift-boundary-and-task-order-ssot.md"
classification_basis = "C2-singleton-al: Program(JSON v0) compat routes + MIR(JSON) mainline routes artifact route map"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "kilo-meso-benchmark-ladder-ssot.md"
role = "status-ledger"
owner = "runtime-hot-lane-optimization-patterns-ssot.md"
precedence_parent = "runtime-hot-lane-optimization-patterns-ssot.md"
classification_basis = "C2-singleton-al: string/edit workload meso benchmark ladder filling the kilo_micro/kilo_kernel observation gap"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "language-feature-implementation-order-ssot.md"
role = "status-ledger"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: durable implementation order for low-level Hakorune language features"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "language-minimal-lane-switch-after-m215-ssot.md"
role = "status-ledger"
owner = "type-system-policy-ssot.md"
precedence_parent = "type-system-policy-ssot.md"
classification_basis = "C2-singleton-al: D210 lane switch from mimalloc inventory closeout to minimal language surface rows"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "loop-cleanup-before-packedarray-ssot.md"
role = "status-ledger"
owner = "loop-canonicalizer.md"
precedence_parent = "loop-canonicalizer.md"
classification_basis = "C2-singleton-al: loop-surface cleanup lane before resuming PACKED-001"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory is replaced by an explicit durable owner"

[[documents]]
path = "coreloop-composer-single-entry-ssot.md"
role = "superseded"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: Status Retired; single-entry composer replaced by v0/v1 boundary"
sidecars = []
supersedes = []
superseded_by = "coreloop-composer-v0-v1-boundary-ssot.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "coreplan-shadow-adopt-tag-coverage-ssot.md"
role = "superseded"
owner = "coreplan-skeleton-feature-model.md"
precedence_parent = "coreplan-skeleton-feature-model.md"
classification_basis = "C2-singleton-al: Status Deprecated; replaced by FlowBox schema tags"
sidecars = []
supersedes = []
superseded_by = "flowbox-tag-coverage-map-ssot.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "domainplan-residue-ssot.md"
role = "superseded"
owner = "recipe-tree-and-parts-ssot.md"
precedence_parent = "recipe-tree-and-parts-ssot.md"
classification_basis = "C2-singleton-al: Status Historical; moved to design/archive/"
sidecars = []
supersedes = []
superseded_by = "archive/domainplan-residue-ssot.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "domainplan-thinning-ssot.md"
role = "superseded"
owner = "recipe-first-entry-contract-ssot.md"
precedence_parent = "recipe-first-entry-contract-ssot.md"
classification_basis = "C2-singleton-al: Status Historical; moved to design/archive/"
sidecars = []
supersedes = []
superseded_by = "archive/domainplan-thinning-ssot.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "if-then-condition-block-consult.md"
role = "superseded"
owner = "condition-observation-ssot.md"
precedence_parent = "condition-observation-ssot.md"
classification_basis = "C2-singleton-al: Status Superseded (2026-01)"
sidecars = []
supersedes = []
superseded_by = "block-expressions-and-condition-blocks-ssot.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "loop-route-detection-physical-path-retirement-ssot.md"
role = "superseded"
owner = "execution-lanes-and-axis-separation-ssot.md"
precedence_parent = "execution-lanes-and-axis-separation-ssot.md"
classification_basis = "C2-singleton-al: Status Historical; moved to design/archive/"
sidecars = []
supersedes = []
superseded_by = "archive/loop-route-detection-physical-path-retirement-ssot.md"
retire_when = "C3 supersession review retires this stub after reachable-reference closure"

[[documents]]
path = "mimalloc-hako-port-purpose-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-MF:mimalloc-hako-port; current normative port purpose and allocator-provider stop-line owner (CLAUDE.md-cited)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "mimalloc port purpose and allocator-provider stop line are replaced explicitly"

[[documents]]
path = "language-minimal-surface-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "docs/reference/language/charter.md"
classification_basis = "C2-MF:language-minimal-surface; normative one-canonical-spelling language surface law"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "language minimal-surface law is replaced explicitly"

[[documents]]
path = "language-result-propagation-and-exit-transaction-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "docs/reference/language/failure-outcome-relations.md"
classification_basis = "README:Result-only propagation and verified exit transaction; accepted target with production activation 0"
sidecars = []
supersedes = ["exception-cleanup-async.md", "stage0-cleanup-catch-boundary-ssot.md"]
superseded_by = ""
retire_when = "recoverable failure, propagation, and exit transaction adopt an explicitly accepted replacement authority"

[[documents]]
path = "ring1-core-provider-scope-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-MF:ring1-core-provider; normative ring1 provider responsibility scope and decision matrix"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "ring1 provider scope is replaced explicitly"

[[documents]]
path = "selfhost-bootstrap-route-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "INDEX.md"
classification_basis = "C2-MF:selfhost-bootstrap-route; normative minimal selfhost bootstrap route contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "selfhost bootstrap route contract is replaced explicitly"

[[documents]]
path = "selfhost-bootstrap-route-evidence-and-legacy-lanes.md"
role = "superseded"
owner = "selfhost-bootstrap-route-ssot.md"
precedence_parent = "selfhost-bootstrap-route-ssot.md"
classification_basis = "C2-MF:selfhost-bootstrap-route; historical moved-to-archive tombstone (Status: Historical)"
sidecars = []
supersedes = []
superseded_by = "archive/selfhost-bootstrap-route-evidence-and-legacy-lanes.md"
retire_when = "tombstone removed after references to the old path are closed"

[[documents]]
path = "ring1-core-provider-promotion-template-ssot.md"
role = "supporting"
owner = "ring1-core-provider-scope-ssot.md"
precedence_parent = "ring1-core-provider-scope-ssot.md"
classification_basis = "C2-MF:ring1-core-provider; provisional-to-accepted promotion procedure template under the scope authority"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "mimalloc-hako-port-capability-gap-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:mimalloc-hako-port; capability gap decision-surface inventory, not execution authority"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-baseline-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-427A comparison baseline inventory; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-baseline-diagnostics-ssot.md",
  "hako-alloc-allocator-comparison-baseline-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-workload-matrix-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-430A comparison workload matrix inventory; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-workload-matrix-diagnostics-ssot.md",
  "hako-alloc-allocator-comparison-workload-matrix-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-measurement-plan-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-433A comparison measurement plan inventory; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-measurement-plan-diagnostics-ssot.md",
  "hako-alloc-allocator-comparison-measurement-plan-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-benchmark-execution-preflight-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-436A benchmark execution preflight inventory; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-benchmark-execution-preflight-diagnostics-ssot.md",
  "hako-alloc-allocator-comparison-benchmark-execution-preflight-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-controlled-benchmark-execution-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-440A controlled benchmark execution inventory; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-controlled-benchmark-execution-diagnostics-ssot.md",
  "hako-alloc-allocator-comparison-controlled-benchmark-execution-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-execution-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-448A C mimalloc execution inventory; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-c-mimalloc-execution-diagnostics-ssot.md",
  "hako-alloc-allocator-comparison-c-mimalloc-execution-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-454A C-vs-Hako scalar result ledger; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-summary-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-457A result summary inventory; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics-ssot.md",
  "hako-alloc-allocator-comparison-c-mimalloc-result-summary-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-reporting-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-460A result reporting inventory; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-c-mimalloc-result-reporting-diagnostics-ssot.md",
  "hako-alloc-allocator-comparison-c-mimalloc-result-reporting-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-controlled-benchmark-execution-plan-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-439A controlled benchmark execution plan (planning-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-execution-plan-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-447A C mimalloc execution plan (planning-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-preflight-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-464A guarded first performance/memory conclusion preflight (conclusion closed)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-451A explicit C mimalloc runner execution pilot; evidence diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics-ssot.md",
  "hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-representative-benchmark-execution-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-444A representative benchmark execution pilot; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-allocator-comparison-representative-benchmark-execution-diagnostics-ssot.md",
  "hako-alloc-allocator-comparison-representative-benchmark-execution-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-468A first provisional conclusion pilot (model-space only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-explicit-runner-planning-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-566A terminal explicit-runner planning pilot (execution closed)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-conclusion-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-474A presentation-only conclusion pilot"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-follow-on-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-480A presentation follow-on pilot"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-560A presentation-only extension pilot"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-552A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-546A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-540A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-534A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-528A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-522A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-516A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-510A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-504A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-498A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-492A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-allocator; MIMAP-486A presentation extension chain pilot (reshape-only, provisional outcome unchanged)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-inactive-boundary-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-352A provider/host inactive boundary inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-facing-ladder-closed-plan-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-358A provider-facing ladder closed plan (planning-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-boundary-diagnostic-vocabulary-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-360A provider boundary diagnostic reason-vocabulary inventory (base row, not a diagnostics sidecar)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-readiness-preflight-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-362A provider readiness preflight (activation closed)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-selection-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-364A provider selection candidate inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-activation-first-pattern-plan-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-368A provider activation first-pattern plan (planning-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-activation-explicit-input-contract-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-374A provider activation explicit-input contract (planning/contract, no runtime behavior)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-activation-input-bundle-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-376A provider activation input bundle inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-activation-unsupported-outcome-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-370A provider activation unsupported-outcome ledger; its closeout is an owned sidecar"
sidecars = [
  "hako-alloc-provider-activation-unsupported-outcome-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-call-execution-capability-preflight-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-388A provider-call execution capability preflight (execution closed)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-call-real-api-execution-preflight-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-392A real provider API execution preflight readiness (execution closed)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-call-external-api-adapter-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-400A external provider API adapter boundary inventory (member of the multi-base 404A closeout)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-call-external-api-adapter-preflight-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-402A external provider API adapter preflight readiness (member of the multi-base 404A closeout)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-activation-dry-run-unsupported-behavior-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-378A dry-run unsupported activation behavior proof (activation closed)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-activation-modeled-open-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-380A modeled provider-activation-open pilot (model space)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-call-modeled-open-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-386A modeled provider-call-open pilot"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-call-noop-execution-seam-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-390A provider-call no-op execution seam pilot"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-call-real-api-stub-execution-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-396A real-API stub execution pilot; its closeout is an owned sidecar"
sidecars = [
  "hako-alloc-provider-call-real-api-stub-execution-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-call-external-api-call-stub-execution-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-406A external-API call stub execution pilot; its closeout is an owned sidecar"
sidecars = [
  "hako-alloc-provider-call-external-api-call-stub-execution-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-facing-ladder-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-366A multi-base closeout of the provider-facing ladder (358A/360A/362A/364A); independent row per Q1 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-provider-call-external-api-adapter-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-provider; MIMAP-404A multi-base closeout of the adapter inventory/preflight pack (400A/402A); independent row per Q1 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-real-external-provider-api-call-first-pattern-plan-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-real; MIMAP-414A real external provider API call first-pattern plan (planning-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-real-external-provider-api-adapter-execution-preflight-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-real; MIMAP-410A real external adapter execution preflight; its closeout is an owned sidecar"
sidecars = [
  "hako-alloc-real-external-provider-api-adapter-execution-preflight-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-real-external-provider-api-call-first-pattern-pilot-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-real; MIMAP-415A first real external provider API call pilot; its closeout is an owned sidecar"
sidecars = [
  "hako-alloc-real-external-provider-api-call-first-pattern-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-host-replacement-optional-ladder-plan-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-host; MIMAP-419A host replacement optional ladder plan (replacement closed)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-host-replacement-explicit-preflight-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-host; MIMAP-420A host replacement explicit preflight inventory; diagnostics/closeout are its owned pack"
sidecars = [
  "hako-alloc-host-replacement-blocked-state-diagnostics-ssot.md",
  "hako-alloc-host-replacement-preflight-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-host-replacement-backend-matcher-no-growth-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-host; MIMAP-424A standalone backend-matcher no-growth reconfirm guard (no single base row); independent row per Q2 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-owner-transfer-contract-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-051A read-only owner-transfer precondition contract model"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-atomic-claim-contract-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-054A no-execution owner-token claim contract model"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-owner-transfer-execution-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-055A first guarded owner-transfer execution route (executor-local model)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-remote-free-drain-contract-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-056A no-execution remote-free drain contract model"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-remote-free-drain-execution-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-057A first modeled remote-free drain execution route"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-post-drain-owner-transfer-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-058A modeled post-drain owner-transfer integration route"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-completion-marker-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-060A scalar reclaim completion marker route"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-scheduler-request-marker-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-064A scalar scheduler request marker contract route"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-scheduler-request-ledger-consume-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-071A scalar scheduler-request-ledger consume route; its closeout is an owned sidecar"
sidecars = [
  "hako-alloc-reclaim-scheduler-request-ledger-consume-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-scheduler-request-ledger-roundtrip-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-074A scalar scheduler-request-ledger roundtrip route; its closeout is an owned sidecar"
sidecars = [
  "hako-alloc-reclaim-scheduler-request-ledger-roundtrip-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-scheduler-boundary-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-063A reclaim scheduler boundary inventory"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-scheduler-request-ledger-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-068A scalar scheduler request ledger; its closeout is an owned sidecar"
sidecars = [
  "hako-alloc-reclaim-scheduler-request-ledger-closeout-ssot.md",
]
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-scalar-lane-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-061A multi-base closeout of the scalar reclaim lane (054A-060A); independent row per Q1 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-scheduler-marker-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-065A multi-base closeout of scheduler boundary/request-marker (063A/064A); independent row per Q1 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "hako-alloc-reclaim-scheduler-scalar-lane-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:hako-alloc-reclaim; MIMAP-077A multi-base closeout of the scheduler scalar lane (063A/064A/068A/071A/074A); independent row per Q1 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-entry-contract-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M76 activation entry contract (docs/fixture/guard)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-implementation-entry-contract-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M92 activation implementation entry contract (docs/fixture/guard)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-safety-gate-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M81 activation safety gate diagnostic shape"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-safety-diagnostic-owner-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M82 activation-safety diagnostic owner/guard hygiene row"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-decision-diagnostic-owner-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M88 activation-decision diagnostic owner/guard hygiene row"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-decision-surface-proposal-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M86 activation-decision surface proposal (proposal-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-decision-diagnostic-report-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M89 activation-decision diagnostic report surface (parses caller TOML only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-decision-cli-surface-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M90 activation-decision diagnostic CLI surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-safety-diagnostic-report-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M83 activation-safety diagnostic report surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-safety-cli-surface-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M84 activation-safety diagnostic CLI surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-safety-closeout-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M85 multi-base closeout inventory of the activation-safety ladder M81-M84; independent row per Q1 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-decision-closeout-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M91 multi-base closeout inventory of the activation-decision ladder M86-M90; independent row per Q1 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-activation-diagnostic-closeout-inventory-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-activation; M95 multi-base closeout inventory of the post-decision diagnostic ladder M92-M94/M93B; independent row per Q1 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-proof-bundle-consumption-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-proof; M79 reserved proof-bundle consumption diagnostic shape (base)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-proof-bundle-consumption-entry-contract-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-proof; M100 proof-bundle consumption implementation entry contract"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-proof-bundle-consumption-diagnostic-report-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-proof; M98 proof-bundle consumption diagnostic report surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-proof-bundle-consumption-cli-surface-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-proof; M99 proof-bundle consumption CLI surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-proof-consumption-failfast-entry-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-proof; M101 first fail-fast runtime proof-consumption entry behavior"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-registry-boundary-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-registry; M71 provider registry boundary docs (docs-only)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-registry-snapshot-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-registry; M77 registry snapshot diagnostic shape (base)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-registry-snapshot-diagnostic-report-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-registry; M93 registry-snapshot diagnostic report surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-registry-snapshot-cli-surface-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-registry; M94 registry-snapshot diagnostic CLI surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-selection-decision-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-selection; M78 selection decision diagnostic shape (base)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-selection-decision-diagnostic-report-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-selection; M96 selection-decision diagnostic report surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-selection-decision-cli-surface-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-selection; M97 selection-decision diagnostic CLI surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-manifest-v0-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-manifest; M65 reserved manifest vocabulary fixture (base)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-manifest-diagnostic-parser-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-manifest; M67 manifest diagnostic parser surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-manifest-cli-surface-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-manifest; M68 manifest diagnostic CLI surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-native-mimalloc-proof-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-native; M75 native_mimalloc reserved proof boundary fixture"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-native-system-proof-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-native; M74 native_system_malloc reserved proof boundary fixture"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-selected-provider-precondition-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-selected; M102 selected-provider precondition (no selection implemented)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-selected-provider-proof-validation-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-selected; M103 selected-provider proof-validation runtime row"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-diagnostic-helper-cleanup-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-diagnostic; M97B provider diagnostic TOML-helper single-owner cleanup guard"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-diagnostic-inactive-actions-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-diagnostic; M93B provider diagnostic inactive-action code-side guard"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-rollback-preflight-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-rollback; M80 rollback preflight diagnostic shape"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-hako-model-proof-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-hako; M72 hako_model_allocator reserved proof fixture"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-provider-debug-guarded-proof-ssot.md"
role = "supporting"
owner = "mimalloc-hako-port-purpose-ssot.md"
precedence_parent = "mimalloc-hako-port-purpose-ssot.md"
classification_basis = "C2-MF:allocator-provider-debug; M73 debug_guarded_allocator reserved proof fixture"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-hook-plan-v0-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-MF:allocator-hook-plan; M53 reserved HookPlan v0 vocabulary lock"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-hook-activation-proof-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-MF:allocator-hook-activation; M55 reserved hook activation-proof vocabulary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-hook-activation-preflight-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-MF:allocator-hook-activation; M-preflight hook activation preflight boundary (proof handoff naming, inactive)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-hook-activation-preflight-shape-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-MF:allocator-hook-activation; M-shape diagnostic-only hook activation preflight data shape"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-hook-activation-proof-validator-ssot.md"
role = "supporting"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-MF:allocator-hook-activation; M-validator hook activation-proof TOML validator (diagnostic-only runtime fact)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-hook-runtime-dry-run-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-MF:allocator-hook-runtime; M54 hook runtime dry-run boundary/guard row (activation absent)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-hook-runtime-owner-ssot.md"
role = "status-ledger"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-MF:allocator-hook-runtime; M56 hook runtime owner/guard row (implementation absent)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "allocator-hook-dry-run-cli-surface-ssot.md"
role = "supporting"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-MF:allocator-hook-dry; M-cli hook dry-run diagnostic CLI surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-hook-dry-run-manifest-callsite-ssot.md"
role = "supporting"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-MF:allocator-hook-dry; M58 hook dry-run manifest callsite integration (diagnostic)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "allocator-hook-dry-run-test-surface-ssot.md"
role = "supporting"
owner = "allocator-replacement-hook-boundary-ssot.md"
precedence_parent = "allocator-replacement-hook-boundary-ssot.md"
classification_basis = "C2-MF:allocator-hook-dry; M-test hook dry-run test-only observation surface"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this proof/pilot row is explicitly replaced; owned sidecars retire with it"

[[documents]]
path = "mimalloc-osvm-fast-path-route-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:mimalloc-osvm-fast; multi-base closeout of MIMAP-042A/043A OSVM fast-path route rows (base rows in phase-293x, absent from design root); independent row per Q2 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "mimalloc-osvm-fast-path-unreserve-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:mimalloc-osvm-fast; multi-base closeout of MIMAP-045A/046A OSVM fast-path unreserve rows (base rows absent from design root); independent row per Q2 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "purge-lifecycle-ladder-map-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:purge-lifecycle-ladder; navigation map of the M192-M213 purge/lifecycle/reclaim ladder (prevents seam bypass)"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "purge-lifecycle-ladder-closeout-ssot.md"
role = "status-ledger"
owner = "mimalloc-hako-port-implementation-plan-ssot.md"
precedence_parent = "mimalloc-hako-port-implementation-plan-ssot.md"
classification_basis = "C2-MF:purge-lifecycle-ladder; multi-base closeout of the M192-M213 ladder (base rows absent from design root); independent row per Q2 precedent"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "this ledger/inventory row is replaced by an explicit durable owner; owned sidecars retire with it"

[[documents]]
path = "binding-ssa-first-control-lowering-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "compiler-pipeline-ssot.md"
classification_basis = "README:現役の設計図（入口）; canonical resolved-source local-value/control lowering authority"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "canonical control lowering adopts an explicitly accepted replacement authority"

[[documents]]
path = "box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md"
role = "superseded"
owner = "box-lifecycle-cprime-terminal-home-finalization-ssot.md"
precedence_parent = "box-lifecycle-cprime-terminal-home-finalization-ssot.md"
classification_basis = "README:B′ Box lifecycle historical constitution; superseded by accepted C′ terminal-Home finalization"
sidecars = []
supersedes = []
superseded_by = "box-lifecycle-cprime-terminal-home-finalization-ssot.md"
retire_when = "C′ reference and implementation closeout no longer needs the historical B′ comparison"

[[documents]]
path = "box-lifecycle-cprime-terminal-home-finalization-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "docs/reference/language/lifecycle.md"
classification_basis = "README:C′ terminal Home finalization; accepted target with production activation 0"
sidecars = []
supersedes = ["box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md"]
superseded_by = ""
retire_when = "Box lifecycle and terminal Home finalization adopt an explicitly accepted replacement constitution"

[[documents]]
path = "box-member-field-method-surface-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "docs/reference/language/EBNF.md"
classification_basis = "README:現役の設計図（入口）; accepted field-or-method Box surface and computed/once/birth_once Property retirement order"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "the Box member surface and Property retirement order are replaced by an explicitly accepted language Decision"

[[documents]]
path = "joinir-if-recipe-contract-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "joinir-loop-selfhost-recipe-pipeline-ssot.md"
classification_basis = "CURRENT_STATE:portable If Recipe/JoinSig/physical-adoption boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "the portable If recipe contract is replaced by an explicitly accepted control-recipe authority"

[[documents]]
path = "joinir-loop-scoped-nongeneric-cutover-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "joinir-loop-selfhost-recipe-pipeline-ssot.md"
classification_basis = "CURRENT_STATE:scoped non-Generic bridge and final atomic Loop cutover boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "the scoped Loop bridge closes or the final atomic cutover replaces it"

[[documents]]
path = "ownership-home-model-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "docs/reference/language/ownership.md"
classification_basis = "DOCS_LAYOUT:durable cross-layer Home/place/value/callable-ABI authority map"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "the source Home model and cross-layer authority split are replaced explicitly"

[[documents]]
path = "ring2-provider-link-abi-lifecycle-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "docs/architecture/RINGS.md"
classification_basis = "DOCS_LAYOUT:ring2 provider link ABI and provider-image lifecycle authority"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "ring2 provider transport, residency, and lifecycle adopt an explicitly accepted replacement"

[[documents]]
path = "design-registry-v1-sharded-manifest-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "current-docs-archive-policy-ssot.md"
classification_basis = "README:現役の設計図（入口）; deterministic sharded Design Registry V1 storage, parity, cutover, and V0 retirement boundary"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "design registry storage adopts an explicitly accepted replacement schema"
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
