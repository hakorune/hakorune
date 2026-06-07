---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-119.
Related:
  - docs/development/current/main/phases/phase-296x/296x-617-MIM-PORT-FMEM-118-REFILL-THEN-FREE-HEAD-ALLOC-BODY-SOURCE-SYNTAX-MANIFEST-PROMOTION.md
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
---

# 296x-618 MIM-PORT-FMEM-119 Remaining Source-Syntax Smoke Retirement Task Order

## Purpose

Re-inventory the remaining `.hako` sources still referenced directly by
`tools/hako_check/fastmem_source_syntax_smoke.sh` after 296x-615..617 moved the
last straight-line allocation bodies into the manifest runner.

This is a task-ordering row. It does not add fixtures, edit `.hako` sources,
or change FastMemory behavior.

## Current Manifest Ownership

The source-syntax manifest now owns 13 fixtures:

```text
SAME_REMOTE_FREE_PUBLISH_BODY
SAME_OWNER_FREE_BODY
LOCAL_FREE_ALLOC_BODY
LOCAL_FREE_TO_FREE_REFILL_BODY
LOCAL_FREE_TO_FREE_REFILL_COUNTER_BODY
FREE_HEAD_ALLOC_BODY
REFILL_THEN_FREE_HEAD_ALLOC_BODY
FREE_HEAD
PILOT
OWNER
OWNER_EQ
LOCAL_FREE_HEAD
LOCAL_FREE_MEMOP
```

## Remaining Shell-Owned Source References

The shell smoke still references:

```text
page_meta_fastmem_branch_return_scope_box.hako
page_meta_atomic_remote_head_push_vocabulary_box.hako
page_meta_atomic_remote_head_drain_vocabulary_box.hako
page_meta_drain_remote_list_to_local_vocabulary_box.hako
page_meta_remote_owner_branch_routing_lowering_box.hako
page_meta_fastmem_branch_cfg_lowering_box.hako
page_meta_local_free_push_precondition_box.hako
page_meta_local_free_pop_precondition_box.hako
page_meta_free_head_pop_vocabulary_box.hako
page_meta_free_head_pop_precondition_box.hako
page_meta_free_head_push_vocabulary_box.hako
page_meta_free_head_push_precondition_box.hako
page_meta_refill_then_free_head_alloc_body_box.hako
page_meta_page_local_alloc_route_cfg_preflight_box.hako
```

`page_meta_refill_then_free_head_alloc_body_box.hako` is already
manifest-backed as `REFILL_THEN_FREE_HEAD_ALLOC_BODY`; the shell keeps only
AST/MIR generation because later terminal ladder checks reuse that MIR input.
It is not a remaining source-body assertion owner.

## Classification

### Good Manifest Promotion Rows

These should move one-at-a-time into `fastmem_source_syntax_smoke.toml`:

```text
page_meta_page_local_alloc_route_cfg_preflight_box.hako
page_meta_local_free_push_precondition_box.hako
page_meta_local_free_pop_precondition_box.hako
page_meta_free_head_push_precondition_box.hako
page_meta_free_head_pop_precondition_box.hako
```

They have direct AST/MIR inventory and producer/check expectations that match
the manifest runner model.

### Vocabulary / Failure Fixture Rows

These can move to manifest, but should preserve expected failure behavior or
non-lowerable producer evidence exactly:

```text
page_meta_free_head_push_vocabulary_box.hako
page_meta_free_head_pop_vocabulary_box.hako
page_meta_atomic_remote_head_push_vocabulary_box.hako
page_meta_atomic_remote_head_drain_vocabulary_box.hako
page_meta_drain_remote_list_to_local_vocabulary_box.hako
```

Each row should explicitly decide whether the producer expectation is success
or `expect_failure` with a stable stderr substring.

### Branch / Routing Rows

These are higher risk and should stay late:

```text
page_meta_remote_owner_branch_routing_lowering_box.hako
page_meta_fastmem_branch_cfg_lowering_box.hako
page_meta_fastmem_branch_return_scope_box.hako
```

They exercise branch CFG, route-body, or MIRBuilder lexical return-scope
behavior. Move them after the precondition/vocabulary rows, or split them into
a route-CFG manifest if source-syntax manifest starts mixing too many route
profiles.

### Shared Input, Not Assertion Owner

```text
page_meta_refill_then_free_head_alloc_body_box.hako
```

Keep AST/MIR generation in the shell until terminal ladder report/check rows no
longer need the shared MIR input. Do not duplicate its source-body assertions in
the shell.

## Proposed Next Rows

### 296x-619: Page-Local Alloc Route CFG Preflight Manifest Promotion

```text
source:
  page_meta_page_local_alloc_route_cfg_preflight_box.hako

open:
  manifest fixture for page-local alloc route CFG preflight
  page-local-alloc-route-cfg-preflight report/check evidence

closed:
  route CFG lowering
  route-body join
  terminal ladder refresh
  TLS / owner reuse / reclaim
  branch claim
  product/hook/global/winner claims
```

### 296x-620: Page-Local Alloc Route CFG Producer Profile Promotion

```text
source:
  page_meta_page_local_alloc_route_cfg_preflight_box.hako

open:
  page-local-alloc-route-cfg producer profile expectations

closed:
  route-body join
  terminal ladder refresh
  TLS / owner reuse / reclaim
  product/hook/global/winner claims
```

### 296x-621: Page-Local Route Body Join Preflight Promotion

```text
source:
  page_meta_page_local_alloc_route_cfg_preflight_box.hako

open:
  page-local-route-body-join preflight profile

closed:
  route-body join producer
  terminal ladder refresh
  TLS / owner reuse / reclaim
  product/hook/global/winner claims
```

### 296x-622: Page-Local Route Body Join Producer Profile Promotion

```text
source:
  page_meta_page_local_alloc_route_cfg_preflight_box.hako

open:
  page-local-route-body-join producer profile

closed:
  terminal ladder refresh
  TLS / owner reuse / reclaim
  product/hook/global/winner claims
```

### 296x-623: Terminal Ladder Shared-Input Split

```text
open:
  dedicated terminal ladder smoke/manifest using existing shared MIR inputs

move out of source-syntax smoke:
  terminal_ladder_refresh
  tls_backing_transfer
  owner_slot_reuse
  abandoned_reclaim
  product_activation
  hook_install
  global_allocator_claim
  winner_claim

closed:
  new product behavior
  source-body assertion changes
```

### 296x-624: Local-Free Precondition Manifest Promotion

```text
sources:
  page_meta_local_free_push_precondition_box.hako
  page_meta_local_free_pop_precondition_box.hako

open:
  LocalFreePush / LocalFreePop precondition fixtures
  same-owner and block-next proof evidence

closed:
  allocation route CFG
  remote routing
  product/hook/global/winner claims
```

### 296x-625: Free-Head Precondition Manifest Promotion

```text
sources:
  page_meta_free_head_push_precondition_box.hako
  page_meta_free_head_pop_precondition_box.hako

open:
  FreeHeadPush / FreeHeadPop precondition fixtures
  same-owner, block-next, and non-empty proof evidence

closed:
  derived route composition
  product/hook/global/winner claims
```

### 296x-626: Free-Head Vocabulary Failure Fixture Promotion

```text
sources:
  page_meta_free_head_push_vocabulary_box.hako
  page_meta_free_head_pop_vocabulary_box.hako

open:
  vocabulary-only fixture rows
  lowerer fail-closed / non-lowerable expectations

closed:
  precondition proofs
  route execution
```

### 296x-627: Atomic Remote / Drain Vocabulary Fixture Promotion

```text
sources:
  page_meta_atomic_remote_head_push_vocabulary_box.hako
  page_meta_atomic_remote_head_drain_vocabulary_box.hako
  page_meta_drain_remote_list_to_local_vocabulary_box.hako

open:
  atomic remote head and drain vocabulary manifest fixtures
  retry/drain failure or producer expectations as currently pinned

closed:
  remote-owner branch routing
  TLS transfer
  product/hook/global/winner claims
```

### 296x-628: Branch / Remote Routing Source-Syntax Fixture Split

```text
sources:
  page_meta_remote_owner_branch_routing_lowering_box.hako
  page_meta_fastmem_branch_cfg_lowering_box.hako
  page_meta_fastmem_branch_return_scope_box.hako

open:
  route/branch source-syntax ownership split
  either manifest promotion or a dedicated route-CFG smoke manifest

closed:
  new branch semantics
  product/hook/global/winner claims
```

## Deferred Original Ordering

The local/free-head precondition rows remain valid, but they should follow the
route/terminal split because the current shell smoke still mixes the page-local
route source with terminal ladder report/check profiles.

```text
after terminal ladder split:
  local/free-head preconditions
  free-head vocabulary failure fixtures
  atomic remote / drain vocabulary fixtures
  branch / remote routing split
```

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
Remaining source-syntax shell-owned FastMemory targets are classified into
manifest-ready precondition rows, vocabulary/failure fixture rows, branch/routing
rows, and shared-input-only MIR generation. The next implementation row is
296x-619 page-local alloc route CFG preflight manifest promotion.
```

## Closeout

```text
next: 296x-619 page-local alloc route CFG preflight manifest promotion
```
