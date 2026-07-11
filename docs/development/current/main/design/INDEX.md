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
unregistered_baseline = 844

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
path = "current-docs-update-policy-ssot.md"
role = "authority"
owner = "INDEX.md"
precedence_parent = "CURRENT_STATE.toml"
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "current documentation update policy is replaced explicitly"
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

