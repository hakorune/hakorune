---
Status: Active
Date: 2026-04-25
Scope: CoreBox surface catalog を ArrayBox から StringBox / MapBox へ広げる phase front。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-290x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-94-map-std-prelude-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
  - docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md
  - docs/development/current/main/phases/phase-291x/291x-98-mapbox-content-enumeration-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-99-mapbox-write-return-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-100-mapbox-bad-key-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-102-mapbox-keys-values-element-publication-card.md
  - docs/development/current/main/phases/phase-291x/291x-103-stringbox-lastindexof-start-card.md
  - docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-105-mapbox-clear-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-106-arraybox-element-result-publication-card.md
  - docs/development/current/main/phases/phase-291x/291x-107-string-semantic-owner-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-108-alias-ssot-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-109-map-compat-source-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-110-mapbox-get-existing-key-typing-card.md
  - docs/development/current/main/phases/phase-291x/291x-111-stringbox-case-conversion-card.md
  - docs/development/current/main/phases/phase-291x/291x-112-arraybox-clear-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-113-arraybox-contains-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-114-arraybox-indexof-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-115-arraybox-join-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-116-arraybox-reverse-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-117-arraybox-sort-router-card.md
  - docs/development/current/main/phases/phase-291x/291x-118-arraybox-slice-result-receiver-card.md
  - docs/development/current/main/phases/phase-291x/291x-119-docs-status-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-120-mapbox-taskboard-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-121-doc-update-simplification-contract.md
  - docs/development/current/main/phases/phase-291x/291x-131-hotline-core-method-contract-task-plan.md
  - docs/development/current/main/phases/phase-291x/291x-132-core-method-contract-seed-card.md
  - docs/development/current/main/phases/phase-291x/291x-133-core-method-contract-manifest-guard-card.md
  - docs/development/current/main/phases/phase-291x/291x-134-core-method-contract-inc-no-growth-guard-card.md
  - docs/development/current/main/phases/phase-291x/291x-135-core-method-op-carrier-card.md
  - docs/development/current/main/phases/phase-291x/291x-136-core-method-has-inc-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-137-lowering-tier-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-138-hcm7-maphas-preflight-evidence-card.md
  - docs/development/current/main/phases/phase-291x/291x-139-receiver-origin-proof-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-140-key-route-value-demand-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-141-maphas-i64-route-card.md
  - docs/development/current/main/phases/phase-291x/291x-142-mapget-return-shape-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-143-mapget-scalar-return-shape-proof-card.md
  - docs/development/current/main/phases/phase-291x/291x-144-mapget-preheader-scalar-proof-card.md
  - docs/development/current/main/phases/phase-291x/291x-145-mapget-scalar-lowering-probe-card.md
  - docs/development/current/main/phases/phase-291x/291x-146-mapget-owner-seam-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-147-mapget-maphas-fusion-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-148-mapget-maphas-fusion-has-const-probe-card.md
  - docs/development/current/main/phases/phase-291x/291x-149-maplookup-get-const-fold-card.md
  - docs/development/current/main/phases/phase-291x/291x-150-maplookup-fusion-const-fold-guard-card.md
  - docs/development/current/main/phases/phase-291x/291x-151-core-method-get-inc-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-152-stageb-trace-adapter-thinning-card.md
  - docs/development/current/main/phases/phase-291x/291x-153-stageb-args-source-resolver-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-154-stageb-main-detection-helper-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-155-stageb-same-source-defs-scan-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-156-stageb-json-fragment-injection-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-157-stageb-keyword-expr-strip-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-158-stageb-dead-comment-strip-helper-removal-card.md
  - docs/development/current/main/phases/phase-291x/291x-159-stageb-dead-helper-box-removal-card.md
  - docs/development/current/main/phases/phase-291x/291x-160-stageb-driver-guard-helper-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-161-core-method-route-policy-mirror-preflight-card.md
  - docs/development/current/main/phases/phase-291x/291x-162-core-method-maphas-emit-kind-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-163-maphas-emit-kind-mirror-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-164-metadata-absent-has-fallback-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-165-core-method-mapget-emit-kind-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-166-metadata-absent-get-fallback-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-167-core-method-len-route-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-168-core-method-len-emit-kind-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-169-metadata-absent-len-fallback-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-170-core-method-substring-route-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-171-core-method-substring-emit-kind-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-172-metadata-absent-substring-fallback-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-173-push-mutating-carrier-preflight-card.md
  - docs/development/current/main/phases/phase-291x/291x-174-core-method-push-route-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-175-core-method-push-emit-kind-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-176-push-emit-kind-mirror-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-177-set-mutating-carrier-preflight-card.md
  - docs/development/current/main/phases/phase-291x/291x-178-core-method-set-route-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-179-core-method-set-emit-kind-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-180-set-emit-kind-mirror-prune-card.md
---

# Phase 291x: CoreBox surface catalog

- Status: Active reference lane
- Date: 2026-04-24
- Purpose: phase-290x の `ArrayBox` catalog/invoke seam を、CoreBox surface の横断ルールへ上げる。
- Landed implementation targets:
  - `StringBox`
  - `MapBox` first current-vtable slice
- Latest landed cleanup target: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- Next implementation target: set storage-route metadata preflight
- Sibling guardrail:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - phase-137x remains observe-only unless app work produces a real blocker

## Decision

ArrayBox で固定した読み方を CoreBox 全体へ広げる。

```text
surface contract
  -> canonical name / aliases / arity / slot / effect / return

execution dispatch
  -> one invoke seam per Box family

exposure state
  -> runtime / VM / std sugar / docs / smoke pinned state
```

ただし、最初の code slice で `StringBox` と `MapBox` を同時に触らない。
phase-291x の初回実装は `StringBox` だけに閉じる。

## Reading Order

1. `docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md`
2. `docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md`
3. `docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md`
4. `docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md`
5. `docs/development/current/main/phases/phase-291x/291x-94-map-std-prelude-cleanup-card.md`
6. `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`
7. `docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md`
8. `docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md`
9. `docs/development/current/main/phases/phase-291x/291x-98-mapbox-content-enumeration-contract-card.md`
10. `docs/development/current/main/phases/phase-291x/291x-99-mapbox-write-return-contract-card.md`
11. `docs/development/current/main/phases/phase-291x/291x-100-mapbox-bad-key-contract-card.md`
12. `docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md`
13. `docs/development/current/main/phases/phase-291x/291x-102-mapbox-keys-values-element-publication-card.md`
14. `docs/development/current/main/phases/phase-291x/291x-103-stringbox-lastindexof-start-card.md`
15. `docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md`
16. `docs/development/current/main/phases/phase-291x/291x-105-mapbox-clear-router-card.md`
17. `docs/development/current/main/phases/phase-291x/291x-106-arraybox-element-result-publication-card.md`
18. `docs/development/current/main/phases/phase-291x/291x-107-string-semantic-owner-cleanup-card.md`
19. `docs/development/current/main/phases/phase-291x/291x-108-alias-ssot-cleanup-card.md`
20. `docs/development/current/main/phases/phase-291x/291x-109-map-compat-source-cleanup-card.md`
21. `docs/development/current/main/phases/phase-291x/291x-110-mapbox-get-existing-key-typing-card.md`
22. `docs/development/current/main/phases/phase-291x/291x-111-stringbox-case-conversion-card.md`
23. `docs/development/current/main/phases/phase-291x/291x-112-arraybox-clear-router-card.md`
24. `docs/development/current/main/phases/phase-291x/291x-113-arraybox-contains-router-card.md`
25. `docs/development/current/main/phases/phase-291x/291x-114-arraybox-indexof-router-card.md`
26. `docs/development/current/main/phases/phase-291x/291x-115-arraybox-join-router-card.md`
27. `docs/development/current/main/phases/phase-291x/291x-116-arraybox-reverse-router-card.md`
28. `docs/development/current/main/phases/phase-291x/291x-117-arraybox-sort-router-card.md`
29. `docs/development/current/main/phases/phase-291x/291x-118-arraybox-slice-result-receiver-card.md`
30. `docs/development/current/main/phases/phase-291x/291x-119-docs-status-closeout-card.md`
31. `docs/development/current/main/phases/phase-291x/291x-120-mapbox-taskboard-closeout-card.md`
32. `docs/development/current/main/phases/phase-291x/291x-121-doc-update-simplification-contract.md`
33. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
34. `docs/development/current/main/phases/phase-291x/291x-131-hotline-core-method-contract-task-plan.md`
35. `docs/development/current/main/phases/phase-291x/291x-132-core-method-contract-seed-card.md`
36. `docs/development/current/main/phases/phase-291x/291x-133-core-method-contract-manifest-guard-card.md`
37. `docs/development/current/main/phases/phase-291x/291x-134-core-method-contract-inc-no-growth-guard-card.md`
38. `docs/development/current/main/phases/phase-291x/291x-135-core-method-op-carrier-card.md`
39. `docs/development/current/main/phases/phase-291x/291x-136-core-method-has-inc-consumer-card.md`
40. `docs/development/current/main/phases/phase-291x/291x-137-lowering-tier-metadata-card.md`
41. `docs/development/current/main/phases/phase-291x/291x-138-hcm7-maphas-preflight-evidence-card.md`
42. `docs/development/current/main/phases/phase-291x/291x-139-receiver-origin-proof-metadata-card.md`
43. `docs/development/current/main/phases/phase-291x/291x-140-key-route-value-demand-metadata-card.md`
44. `docs/development/current/main/phases/phase-291x/291x-141-maphas-i64-route-card.md`
45. `docs/development/current/main/phases/phase-291x/291x-142-mapget-return-shape-metadata-card.md`
46. `docs/development/current/main/phases/phase-291x/291x-143-mapget-scalar-return-shape-proof-card.md`
47. `docs/development/current/main/phases/phase-291x/291x-144-mapget-preheader-scalar-proof-card.md`
48. `docs/development/current/main/phases/phase-291x/291x-145-mapget-scalar-lowering-probe-card.md`
49. `docs/development/current/main/phases/phase-291x/291x-146-mapget-owner-seam-selection-card.md`
50. `docs/development/current/main/phases/phase-291x/291x-147-mapget-maphas-fusion-metadata-card.md`
51. `docs/development/current/main/phases/phase-291x/291x-148-mapget-maphas-fusion-has-const-probe-card.md`
52. `docs/development/current/main/phases/phase-291x/291x-149-maplookup-get-const-fold-card.md`
53. `docs/development/current/main/phases/phase-291x/291x-150-maplookup-fusion-const-fold-guard-card.md`
54. `docs/development/current/main/phases/phase-291x/291x-151-core-method-get-inc-consumer-card.md`
55. `docs/development/current/main/phases/phase-291x/291x-152-stageb-trace-adapter-thinning-card.md`
56. `docs/development/current/main/phases/phase-291x/291x-153-stageb-args-source-resolver-split-card.md`
57. `docs/development/current/main/phases/phase-291x/291x-154-stageb-main-detection-helper-split-card.md`
58. `docs/development/current/main/phases/phase-291x/291x-155-stageb-same-source-defs-scan-split-card.md`
59. `docs/development/current/main/phases/phase-291x/291x-156-stageb-json-fragment-injection-split-card.md`
60. `docs/development/current/main/phases/phase-291x/291x-157-stageb-keyword-expr-strip-split-card.md`
61. `docs/development/current/main/phases/phase-291x/291x-158-stageb-dead-comment-strip-helper-removal-card.md`
62. `docs/development/current/main/phases/phase-291x/291x-159-stageb-dead-helper-box-removal-card.md`
63. `docs/development/current/main/phases/phase-291x/291x-160-stageb-driver-guard-helper-split-card.md`
64. `docs/development/current/main/phases/phase-291x/291x-161-core-method-route-policy-mirror-preflight-card.md`
65. `docs/development/current/main/phases/phase-291x/291x-162-core-method-maphas-emit-kind-metadata-card.md`
66. `docs/development/current/main/phases/phase-291x/291x-163-maphas-emit-kind-mirror-prune-card.md`
67. `docs/development/current/main/phases/phase-291x/291x-164-metadata-absent-has-fallback-contract-card.md`
68. `docs/development/current/main/phases/phase-291x/291x-165-core-method-mapget-emit-kind-metadata-card.md`
69. `docs/development/current/main/phases/phase-291x/291x-166-metadata-absent-get-fallback-contract-card.md`
70. `docs/development/current/main/phases/phase-291x/291x-167-core-method-len-route-metadata-card.md`
71. `docs/development/current/main/phases/phase-291x/291x-168-core-method-len-emit-kind-metadata-card.md`
72. `docs/development/current/main/phases/phase-291x/291x-169-metadata-absent-len-fallback-contract-card.md`
73. `docs/development/current/main/phases/phase-291x/291x-170-core-method-substring-route-metadata-card.md`
74. `docs/development/current/main/phases/phase-291x/291x-171-core-method-substring-emit-kind-metadata-card.md`
75. `docs/development/current/main/phases/phase-291x/291x-172-metadata-absent-substring-fallback-contract-card.md`
76. `docs/development/current/main/phases/phase-291x/291x-173-push-mutating-carrier-preflight-card.md`
77. `docs/development/current/main/phases/phase-291x/291x-174-core-method-push-route-metadata-card.md`
78. `docs/development/current/main/phases/phase-291x/291x-175-core-method-push-emit-kind-metadata-card.md`
79. `docs/development/current/main/phases/phase-291x/291x-176-push-emit-kind-mirror-prune-card.md`
80. `docs/development/current/main/phases/phase-291x/291x-177-set-mutating-carrier-preflight-card.md`
81. `docs/development/current/main/phases/phase-291x/291x-178-core-method-set-route-metadata-card.md`
82. `docs/development/current/main/phases/phase-291x/291x-179-core-method-set-emit-kind-metadata-card.md`
83. `docs/development/current/main/phases/phase-291x/291x-180-set-emit-kind-mirror-prune-card.md`

## Current Rule

- docs-first before code
- current docs update policy is
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`;
  do not append landed history to current mirrors for every card
- zero-cost hot boundary and CoreMethodContract migration policy lives in
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
- CoreMethodContract work must stay split into contract seed, generated
  metadata, drift guard, MIR carrier, `.inc` consumer, and later
  evidence-backed hot lowering
- existing guarded `.inc` mirrors may remain during migration, but new
  method-name classifier growth needs a contract row, deletion condition, and
  focused guard
- `291x-132` landed `CoreMethodContractBox` as the inert Array/String/Map
  seed owner under `lang/src/runtime/meta/`; it does not change `.inc`
  lowering or hot inline behavior
- `291x-133` landed the generated CoreMethodContract manifest and drift guard;
  the manifest is derived and `.inc` consumers still have not moved
- `291x-134` landed the `.inc` no-growth guard for the transitional generic
  method policy mirror; `.inc` consumers still have not moved
- `291x-135` landed the first MIR-side CoreMethodOp carrier for `MapBox.has`;
  `.inc` consumers still use the compatibility route metadata
- `291x-136` moved the generic-method `has` metadata consumer to prefer
  `core_method.op=MapHas`, with route_kind fallback unchanged
- `291x-137` made `lowering_tier` the explicit CoreMethodContract metadata
  field and added shared MIR/`.inc` tier readers without adding hot inline
  lowering
- `291x-138` recorded HCM-7 preflight evidence: the measured Map get/has front
  still routes through `RuntimeDataBox` with `core_method=null`, so hot lowering
  is blocked until receiver-origin/CoreMethod route proof lands
- `291x-139` emits `receiver_origin_box=MapBox` for the measured RuntimeData
  facade route while keeping `core_method=null`; direct MapHas promotion was
  rejected because it regressed and remained key-conversion dominated
- `291x-140` emits `key_route=i64_const` and route-owned `value_demand=read_ref`
  for the measured RuntimeData facade route while keeping lowering unchanged
- `291x-141` promotes only the proven `RuntimeDataBox.has` route with
  `receiver_origin_box=MapBox` and `key_route=i64_const` to CoreMethod MapHas
  via `nyash.map.probe_hi`; `get` routes remain unchanged pending value-demand
  / return-shape proof
- `291x-142` emits metadata-only `RuntimeDataBox.get` MapBox-origin routes as
  CoreMethod MapGet with `return_shape=mixed_runtime_i64_or_handle`,
  `publication_policy=runtime_data_facade`, and `lowering_tier=cold_fallback`;
  codegen remains on `nyash.runtime_data.get_hh`
- `291x-143` adds a same-block scalar MapGet proof that emits
  `return_shape=scalar_i64_or_missing_zero`, `value_demand=scalar_i64`, and
  `publication_policy=no_publication` only after a same-receiver/same-i64-key
  scalar store with no same-receiver mutation/escape; codegen remains on
  `nyash.runtime_data.get_hh`
- `291x-144` extends the scalar MapGet proof to a conservative
  preheader/dominating store shape for the measured `kilo_leaf_map_getset_has`
  front; codegen and `.inc` lowering remain unchanged
- `291x-145` rejected scalar MapGet warm-helper lowering to
  `nyash.map.scalar_load_hi`: it removed `nyash.runtime_data.get_hh` from the
  loop and lowered instruction count, but cycles/IPC regressed and
  `spec_to_string` / `hash_one` remained the owner family
- `291x-146` selects same-key MapGet/MapHas fusion metadata as the next seam;
  native i64-key MapBox storage is deferred because it touches string-key
  compatibility and should not be the immediate card
- `291x-147` lands `MapLookupSameKey` metadata for same-block scalar
  MapGet/MapHas pairs; codegen and `.inc` remain unchanged
- `291x-148` consumes `MapLookupSameKey` metadata to fold the proven `has`
  result to true; `nyash.map.has_h` leaves the hot loop and cycles/IPC improve
- `291x-149` consumes the same `MapLookupSameKey` metadata to fold the proven
  constant `get` result; `nyash.runtime_data.get_hh` leaves the hot loop
- `291x-150` pins the MapLookup fusion shared reader boundary and const-fold
  IR smoke in quick gate coverage
- `291x-151` moves generic-method `get` policy to prefer MIR MapGet
  `core_method` metadata before legacy `bname` fallback while keeping the cold
  RuntimeData facade
- `291x-152` starts HCM-8 Stage-B thinning by removing the duplicate inline
  Stage-B trace helper from `compiler_stageb.hako`
- `291x-153` moves Stage-B args/source resolution into
  `stageb_args_box.hako` while preserving source precedence and `--stage3`
  behavior
- `291x-154` moves Stage-B main/body pattern detection into
  `stageb_main_detection_box.hako` while preserving Stage-A fallback and
  Stage-B same-source scan call sites
- `291x-155` moves Stage-B same-source defs scanning into
  `stageb_same_source_defs_box.hako` and reuses `StageBJsonBuilderBox` for
  defs JSON emission
- `291x-156` moves Stage-B JSON fragment insertion to
  `StageBJsonBuilderBox.inject_json_fragment(...)` and removes the inline
  adapter helper
- `291x-157` moves Stage-B exact keyword expression cleanup into
  `stageb_keyword_expr_strip_box.hako`
- `291x-158` removes the dead inline Stage-B `_strip_comments(...)` duplicate;
  comment stripping remains owned by `CommentStripperBox`
- `291x-159` removes the unreferenced `StageBHelperBox.test_loop(...)`
  scaffold from the Stage-B adapter
- `291x-160` moves Stage-B driver entry trace/depth guard mechanics into
  `stageb_driver_guard_box.hako`
- `291x-161` extends the CoreMethodContract `.inc` no-growth guard to cover
  the mir-call route surface mirror as well as the generic-method policy mirror;
  the guard now pins 27 existing classifier rows before any pruning
- `291x-162` makes generic-method `has` emit-kind selection prefer valid MIR
  `core_method.op=MapHas` metadata before legacy method-name fallback; helper
  selection and lowering remain unchanged
- `291x-163` rejected pruning the generic `mname == "has"` emit-kind mirror
  row because metadata-absent RuntimeData boundary MIR JSON still relies on the
  legacy fallback; the no-growth baseline remains 27 rows
- `291x-164` tightens the `has` allowlist deletion condition so future prune
  attempts must cover metadata-absent `RuntimeDataBox.has` boundary fixtures
- `291x-165` makes generic-method `get` emit-kind selection prefer valid MIR
  `core_method.op=MapGet` metadata before legacy method-name fallback; helper
  selection and lowering remain unchanged
- `291x-166` rejects pruning the generic `mname == "get"` emit-kind mirror
  row for now and tightens its deletion condition to require metadata-absent
  `RuntimeDataBox.get` boundary coverage
- `291x-167` adds arity-0 MIR `generic_method.len` carriers for
  `ArrayLen`/`MapLen`/`StringLen`; `.inc` still uses the legacy length alias
  fallback until the consumer card lands
- `291x-168` makes generic-method `len`/`length`/`size` emit-kind selection
  prefer valid MIR `generic_method.len` CoreMethod metadata before legacy alias
  fallback; helper selection and lowering remain unchanged
- `291x-169` rejects pruning the generic length alias emit-kind mirror rows
  for now and tightens their deletion condition to require metadata-absent
  length boundary coverage
- `291x-170` adds MIR `generic_method.substring` carriers for
  `StringSubstring`; `.inc` still uses the legacy substring fallback until the
  consumer card lands
- `291x-171` makes generic-method `substring` emit-kind selection prefer valid
  MIR `generic_method.substring` CoreMethod metadata before legacy fallback;
  string corridor/window lowering remains unchanged
- `291x-172` rejects pruning the generic `substring` emit-kind mirror row for
  now and tightens its deletion condition to require metadata-absent substring
  boundary coverage
- `291x-173` pins the mutating `push` carrier boundary before implementation:
  `ArrayPush` metadata may be added next, but legacy push mirror rows require
  metadata-absent mutating boundary coverage before any prune attempt
- `291x-174` adds direct `ArrayBox.push(value)` MIR route carriers as
  `generic_method.push` + `core_method.op=ArrayPush`; `RuntimeDataBox.push`
  remains metadata-absent fallback and `.inc` still uses the legacy classifier
- `291x-175` makes generic-method `push` emit-kind selection prefer valid
  MIR `generic_method.push` CoreMethod metadata before legacy fallback; helper
  selection and lowering remain unchanged
- `291x-176` rejects pruning the generic `push` emit-kind mirror row for now:
  metadata-absent direct ArrayBox and RuntimeData push boundary fixtures still
  require the legacy fallback row
- `291x-177` pins the wider mutating `set` carrier boundary before
  implementation: ArraySet/MapSet metadata may be added next, but emit-kind and
  storage-route mirror rows require metadata-absent mutating boundary coverage
  before any prune attempt
- `291x-178` adds direct `ArrayBox.set(index, value)` and
  `MapBox.set(key, value)` MIR route carriers as `generic_method.set` +
  `core_method.op=ArraySet/MapSet`; `RuntimeDataBox.set` remains
  metadata-absent fallback and `.inc` still uses the legacy classifier
- `291x-179` makes generic-method `set` emit-kind selection prefer valid
  MIR `generic_method.set` CoreMethod metadata before legacy fallback; storage
  route selection and lowering remain unchanged
- `291x-180` rejects pruning the generic `set` emit-kind mirror row for now:
  metadata-absent RuntimeData set boundary fixtures still require the legacy
  fallback row
- `StringBox.length()` is canonical; `len()` and `size()` are compatibility aliases
- `StringBox.indexOf(needle, start)` is stable; `find` is compatibility alias
- `StringBox.lastIndexOf(needle, start_pos)` is landed as a StringBox-only catalog row
- `apps/std/string.hako` is sugar, not the semantic owner
- `apps.std.string` is the exact manifest alias that pins the current public
  sugar smoke; this is not a broader `std.string` packaging decision
- alias ownership is split on purpose:
  - manifest alias / module lookup lives in `hako.toml`
  - imported static-box alias binding lives in the runner text-merge strip path
  - static receiver/type-name lowering lives in the MIR builder only
- `using apps.std.string as S` resolves `apps.std.string` as a manifest alias,
  then binds `S` to the exported `StdStringNy` static box for `S.method(...)`
  calls after merge
- imported static-box aliases are not namespace roots; they do not imply
  `new Alias.BoxName()` or `new apps.std.string.BoxName()`
- `apps/lib/boxes/string_std.hako` is an internal selfhost helper, not a
  public std owner
- `apps/std/string_std.hako` is dead scaffold and is removed by `291x-107`
- legacy `apps/std/string2.hako` diagnostic residue was deleted by an explicit cleanup card
- `MapBox` first slice cataloged current Rust vtable rows only
- do not add `length` as a Rust vtable alias in the first MapBox commit
- do not collapse `size` and `len` slots in the first MapBox commit
- do not normalize `set` / `delete` / `clear` return contracts in the first MapBox commit
- `MapBox.length` is now a separate contract-first alias slice; it must not
  promote `keys` / `values` / `delete` / `remove` / `clear`
- `MapBox` source-level write rows now have a contract decision: `set`,
  `delete` / `remove`, and `clear` return Rust-vtable-compatible receipt
  strings; bad-key normalization remains separate
- `MapBox` source-visible bad-key rows now have a contract decision:
  non-string `set/get/has/delete/remove` keys publish
  `[map/bad-key] key must be string`; field rows keep the field-name variant
- `MapBox.get(missing-key)` keeps the stable tagged read-miss text
  `[map/missing] Key not found: <key>`
- `291x-110` landed the conservative successful-read rule for
  `MapBox.get(existing-key)`: publish `V` only for receiver-local homogeneous
  Map facts with tracked literal keys; mixed, untyped, and missing-key reads
  stay `Unknown`
- `291x-111` landed StringBox case conversion as stable surface rows:
  `toUpper` / `toLower` live in the catalog and keep
  `toUpperCase` / `toLowerCase` as compatibility aliases
- `291x-112` landed `ArrayBox.clear()` as a catalog-backed receiver-only
  write-`Void` row on the Unified value path
- `291x-113` landed `ArrayBox.contains(value)` as a catalog-backed
  receiver-plus-value read-`Bool` row on the Unified value path
- `291x-114` landed `ArrayBox.indexOf(value)` as a catalog-backed
  receiver-plus-value read-`Integer` row on the Unified value path
- `291x-115` landed `ArrayBox.join(delimiter)` as a catalog-backed
  receiver-plus-delimiter read-`String` row on the Unified value path
- `291x-116` landed `ArrayBox.reverse()` as a catalog-backed receiver-only
  write-`String` receipt row on the Unified value path
- `291x-117` landed `ArrayBox.sort()` as a catalog-backed receiver-only
  write-`String` receipt row on the Unified value path
- `291x-118` landed the `ArrayBox.slice()` result-receiver pin: direct source
  `slice().length()` stays on the `ArrayBox` receiver path and does not degrade
  to `RuntimeDataBox.length`
- `291x-119` closed stale status/deferred wording as docs-only BoxShape
  cleanup; no CoreBox behavior changed
- `291x-120` closed stale MapBox taskboard follow-up wording as docs-only
  BoxShape cleanup; future-risk rows remain explicitly deferred
- `291x-121` simplified current docs update policy: current mirrors stay thin,
  and latest-card history lives in `CURRENT_STATE.toml` plus the active card
- `MapBox.keys()/values()` element publication is landed through the S0 state
  owner; `keys().get(i)` and `values().get(i)` are pinned in sorted-key order
- `MapBox.delete(key)` and `MapBox.remove(key)` use the catalog-backed Unified
  receiver-plus-key value path
- `MapBox.clear()` now uses the catalog-backed Unified receiver-only value path
- `ArrayBox.get/pop/remove` element-result publication is landed:
  publish `T` only when the receiver has a known `MirType::Array(T)`; keep
  `Unknown` for mixed or untyped receivers.
- alias SSOT cleanup is landed in `291x-108`
- Map compat/source cleanup is landed in `291x-109`: keep `OpsCalls.map_has(...)`
  as the only remaining selfhost-runtime `pref == "ny"` Map wrapper, and keep
  `crates/nyash_kernel/src/plugin/map_compat.rs` as compat-only legacy ABI
  quarantine
- next cleanup starts from the `latest_card_path` recorded in
  `CURRENT_STATE.toml`; do not reopen the landed ArrayBox.clear / contains /
  indexOf / join / reverse / sort rows or the older existing-key typing rule
  without an owner-path change

## Implementation State

Landed first implementation card:

```text
String surface catalog
  -> StringMethodId
  -> StringBox::invoke_surface(...)
  -> thin registry / method-resolution / dispatch consumers
  -> stable String surface smoke
```

Landed smoke:

- `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh`

Landed StringBox cleanup smoke:

- `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_lastindexof_start_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_startswith_vm.sh`

CoreBox catalog cleanup now proceeds by one-family cards. Use
`CURRENT_STATE.toml` for the latest landed card pointer.

Landed MapBox card:

```text
Map surface catalog
  -> MapMethodId
  -> MapBox::invoke_surface(...)
  -> thin registry / method-resolution / effect-analysis / VM dispatch consumers
  -> stable MapBox surface smoke for Rust catalog + hako-visible VM subset
```

Landed smoke:

- `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh`

Landed MapBox follow-up:

- source-level vm-hako non-empty `MapBox.values().size()` state-owner shape is landed and
  pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_values_vm.sh`.
- source-level vm-hako non-empty `MapBox.keys().size()` state-owner shape is landed and
  pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_keys_vm.sh`.
- source-level vm-hako `MapBox.remove(key)` delete-owner alias is landed and
  pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_remove_vm.sh`.
- source-level vm-hako `MapBox.clear()` state reset is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_clear_vm.sh`.
- source-level vm-hako `MapBox.set(...)` duplicate receiver stripping is landed
  and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_set_multiarg_vm.sh`.
- `keys()/values()` element publication is landed in source-level vm-hako and
  pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_keys_values_elements_vm.sh`.
  Rust audit/fix (2026-04-23): `keys()` sorts deterministically and
  `values()` now follows the same sorted-key order for the promoted contract.
- `MapBox.set/delete/remove/clear` source-level write-return receipt contract
  is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh`.
- `MapBox` bad-key normalization is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_bad_key_vm.sh`
  and `tools/smokes/v2/profiles/quick/core/map/map_bad_key_has_vm.sh`.
- `MapBox.get(missing-key)` is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_get_missing_vm.sh`
  and `tools/smokes/v2/profiles/quick/core/map/map_missing_key_tag_vm.sh`.
- `MapBox.get(existing-key)` typing is landed and pinned by focused MIR tests in
  `src/tests/mir_corebox_router_unified.rs`; publish `V` only for
  receiver-local homogeneous Map facts with tracked literal keys.
- `StringBox` case conversion is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh`
  plus focused MIR tests; `toUpperCase` / `toLowerCase` remain compatibility
  aliases on the same stable rows.
- `StringBox.startsWith(prefix)` is landed and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_startswith_vm.sh`
  plus the stable surface smoke; it is a read-only Bool row on the Unified
  value path.
- legacy `apps/std/map_std.hako` JIT-only placeholder was deleted; it was not an active module-registry/prelude route.
- unused `lang/src/vm/hakorune-vm/map_keys_values_bridge.hako` prototype was deleted; it was not an active VM route.
- `apps/lib/boxes/map_std.hako` prelude/module-registry dependency was deleted by the phase-291x cleanup card.
- landed alias card:
  `docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md`
- landed extended-route card:
  `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`

Landed CoreBox router first slice:

- `src/mir/builder/router/policy.rs` now routes only the catalog-backed
  `StringBox.length` / `len` / `size` and `StringBox.substring` / `substr`
  and `StringBox.concat`, `StringBox.trim`, `StringBox.contains`,
  `StringBox.startsWith`, `StringBox.lastIndexOf` one-arg and two-arg,
  `StringBox.replace`, and
  `StringBox.indexOf` /
  `find`, plus `ArrayBox.length` / `size` / `len`, `ArrayBox.push`,
  `ArrayBox.slice`, `ArrayBox.get`, `ArrayBox.pop`, `ArrayBox.set`,
  `ArrayBox.clear`, `ArrayBox.contains`, `ArrayBox.indexOf`, `ArrayBox.join`,
  `ArrayBox.reverse`, `ArrayBox.sort`,
  `ArrayBox.remove`, `ArrayBox.insert`, `MapBox.size`, `MapBox.length`, `MapBox.len`,
  `MapBox.has`, `MapBox.get`, `MapBox.set`, `MapBox.keys`, and
  `MapBox.values`, `MapBox.delete`, `MapBox.remove`, and `MapBox.clear` rows
  through `Route::Unified`.
- `src/mir/builder/utils/boxcall_emit.rs` still bridges `MirType::String` to
  `StringBox` before route selection; uncovered methods remain on the BoxCall
  fallback.
- `ArrayBox.get` / `pop` / `remove` intentionally stayed `MirType::Unknown` in
  the route-only slice; `291x-106` landed the dedicated element-result
  publication card that narrows them only when the receiver has a known
  `Array<T>` MIR fact.
- `ArrayBox.set` follows the write-`Void` contract already used by
  `ArrayBox.push`.
- `ArrayBox.clear` follows the same receiver-only write-`Void` contract already
  used by `ArrayBox.push` / `set` / `insert`.
- `ArrayBox.contains` follows the read-only Bool-return contract already proven
  by `StringBox.contains`, with a receiver-plus-value Unified shape.
- `ArrayBox.indexOf` follows the read-only Integer-return contract already
  proven by StringBox search rows, with a receiver-plus-value Unified shape.
- `ArrayBox.join` follows the read-only String-return contract already proven
  by StringBox read rows, with a receiver-plus-delimiter Unified shape.
- `ArrayBox.reverse` follows the mutating String-receipt contract, with a
  receiver-only Unified shape.
- `ArrayBox.sort` follows the same mutating String-receipt contract, with a
  receiver-only Unified shape.
- `ArrayBox.slice()` result follow-up calls are pinned by `291x-118`; direct
  source `slice().length()` stays on `ArrayBox.length` and must not lower as
  `RuntimeDataBox.length`.
- `ArrayBox.insert` follows the same write-`Void` contract already used by
  `ArrayBox.push` / `ArrayBox.set`.
- `MapBox.get` intentionally stays `MirType::Unknown` because stored map values
  are data-dependent.
- `MapBox.set`, `MapBox.delete` / `remove`, and `MapBox.clear` write-return
  rows have a landed receipt-string contract in `291x-99`; source-level
  vm-hako publication and matching type hints are synced.
- `MapBox.delete` / `remove` router promotion is landed in `291x-104`;
  `MapBox.clear` is landed in `291x-105`.
- `MapBox.get(missing-key)` keeps its landed tagged read-miss contract in
  `291x-101`; successful `get(existing-key)` typing remains data-dependent.
- two-arg `lastIndexOf` is landed in `291x-103`; MapBox keys/values element
  publication is landed in `291x-102`.
- task cards:
  - `docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-105-mapbox-clear-router-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-116-arraybox-reverse-router-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-117-arraybox-sort-router-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-118-arraybox-slice-result-receiver-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-119-docs-status-closeout-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-120-mapbox-taskboard-closeout-card.md`
  - `docs/development/current/main/phases/phase-291x/291x-121-doc-update-simplification-contract.md`
