---
Status: Active
Date: 2026-04-28
Scope: CoreBox surface catalog / CoreMethodContract cleanup phase front.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-smoke-index.md
  - docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md
  - docs/development/current/main/phases/phase-291x/291x-315-joinir-casea-context-label-helper-card.md
  - docs/development/current/main/phases/phase-291x/291x-316-current-pointer-thinning-card.md
  - docs/development/current/main/phases/phase-291x/291x-317-joinir-simple-while-main-gate-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-318-joinir-simple-while-main-gate-helper-card.md
  - docs/development/current/main/phases/phase-291x/291x-319-joinir-casea-update-summary-name-only-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-320-joinir-casea-update-summary-name-only-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-321-joinir-casea-carrier-count-heuristic-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-322-joinir-casea-carrier-count-heuristic-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-323-joinir-loop-update-index-name-heuristic-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-324-joinir-loop-update-rhs-first-classification-card.md
  - docs/development/current/main/phases/phase-291x/291x-325-joinir-loop-update-nested-scope-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-326-joinir-loop-update-nested-scope-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-327-joinir-loop-update-multi-assignment-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-328-joinir-loop-update-all-rhs-classification-card.md
  - docs/development/current/main/phases/phase-291x/291x-329-joinir-loop-update-assignment-value-traversal-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-330-joinir-loop-update-assignment-value-traversal-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-331-joinir-loop-update-summary-helper-split-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-332-joinir-loop-update-summary-helper-split-card.md
  - docs/development/current/main/phases/phase-291x/291x-333-joinir-loop-update-stale-docs-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-334-joinir-loop-update-doc-comment-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-335-joinir-loop-update-caller-surface-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-336-joinir-loop-update-reserved-field-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-337-joinir-loopfeatures-update-summary-surface-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-338-joinir-loopfeatures-update-summary-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-339-joinir-loopfeatures-if-phi-stub-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-340-joinir-loopfeatures-if-phi-stub-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-341-joinir-loopfeatures-nesting-stub-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-342-joinir-loopfeatures-nesting-stub-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-343-joinir-loopfeatures-helper-surface-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-344-joinir-loopfeatures-helper-surface-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-345-joinir-loopfeatures-if-phi-flag-redundancy-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-346-joinir-loopfeatures-if-phi-flag-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-347-joinir-loopfeatures-count-fields-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-348-joinir-loopfeatures-break-continue-count-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-349-joinir-loopfeatures-loopform-constants-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-350-joinir-loopfeatures-loopform-constants-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-351-joinir-loopfeatures-route-surface-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-352-joinir-loopfeatures-route-surface-comment-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-353-joinir-route-detector-legacy-export-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-354-joinir-route-detector-legacy-wildcard-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-355-joinir-route-detector-direct-type-export-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-356-joinir-route-detector-direct-type-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-357-joinir-route-detector-unused-module-export-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-358-joinir-route-detector-legacy-internal-import-owner-path-card.md
  - docs/development/current/main/phases/phase-291x/291x-359-joinir-route-detector-unused-module-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-360-joinir-route-detector-legacy-route-function-export-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-361-joinir-route-detector-legacy-route-function-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-362-joinir-route-detector-legacy-route-function-definition-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-363-joinir-route-detector-legacy-route-function-definition-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-364-joinir-route-detector-legacy-convenience-reexport-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-365-joinir-route-detector-legacy-convenience-reexport-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-366-joinir-route-detector-export-surface-closeout-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-367-joinir-route-detector-legacy-module-visibility-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-368-joinir-route-detector-legacy-module-visibility-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-369-joinir-route-detector-parent-module-doc-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-370-joinir-route-detector-compatibility-module-ownership-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-371-joinir-route-detector-support-facade-add-card.md
  - docs/development/current/main/phases/phase-291x/291x-372-joinir-route-detector-small-support-family-migration-card.md
  - docs/development/current/main/phases/phase-291x/291x-373-joinir-route-detector-small-compatibility-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-374-joinir-route-detector-trim-support-family-migration-card.md
  - docs/development/current/main/phases/phase-291x/291x-375-joinir-route-detector-trim-compatibility-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-376-joinir-route-detector-function-scope-support-family-migration-card.md
  - docs/development/current/main/phases/phase-291x/291x-377-joinir-route-detector-function-scope-compatibility-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-378-joinir-route-detector-condition-scope-support-family-migration-card.md
  - docs/development/current/main/phases/phase-291x/291x-379-joinir-route-detector-condition-scope-compatibility-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-380-joinir-route-detector-body-local-support-family-migration-card.md
  - docs/development/current/main/phases/phase-291x/291x-381-joinir-route-detector-body-local-compatibility-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-382-joinir-route-detector-compatibility-module-ownership-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-383-joinir-route-detector-support-facade-physical-owner-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-384-joinir-route-detector-legacy-support-readme-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-385-joinir-route-detector-legacy-surface-guard-card.md
  - docs/development/current/main/phases/phase-291x/291x-386-joinir-route-detector-locals-physical-owner-move-card.md
  - docs/development/current/main/phases/phase-291x/291x-387-joinir-route-detector-break-condition-physical-owner-move-card.md
  - docs/development/current/main/phases/phase-291x/291x-388-joinir-route-detector-trim-physical-owner-move-card.md
  - docs/development/current/main/phases/phase-291x/291x-389-joinir-route-detector-function-scope-physical-owner-move-card.md
  - docs/development/current/main/phases/phase-291x/291x-390-joinir-route-detector-condition-scope-physical-owner-move-card.md
  - docs/development/current/main/phases/phase-291x/291x-391-joinir-route-detector-body-local-physical-owner-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-392-joinir-route-detector-physical-owner-closeout-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-393-next-compiler-cleanliness-seam-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-394-joinir-carrier-update-legacy-emitter-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-395-normalized-shadow-legacy-lowerer-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-396-normalized-shadow-shared-expr-facade-card.md
  - docs/development/current/main/phases/phase-291x/291x-397-normalized-shadow-shared-expr-implementation-extract-card.md
  - docs/development/current/main/phases/phase-291x/291x-398-normalized-shadow-legacy-entry-surface-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-399-normalized-shadow-legacy-helper-privacy-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-400-normalized-shadow-legacy-entry-facade-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-401-normalized-shadow-if-only-entry-facade-card.md
  - docs/development/current/main/phases/phase-291x/291x-402-normalized-shadow-legacy-physical-storage-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-403-normalized-shadow-legacy-physical-storage-move-card.md
  - docs/development/current/main/phases/phase-291x/291x-404-normalized-shadow-stale-legacy-wording-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-405-normalized-shadow-stale-legacy-wording-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-406-next-compiler-cleanliness-seam-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-407-normalized-shadow-support-contract-wording-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-408-normalized-shadow-k-exit-naming-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-409-normalized-shadow-k-exit-naming-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-410-normalized-shadow-fixed-function-name-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-411-normalized-shadow-fixed-function-name-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-412-normalized-shadow-generic-function-name-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-413-normalized-shadow-generic-function-name-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-414-normalized-shadow-route-local-function-name-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-415-normalized-shadow-route-local-function-name-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-416-normalized-shadow-post-if-fallback-wording-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-417-normalized-shadow-post-if-fallback-wording-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-418-normalized-shadow-exit-reconnector-deprecated-stub-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-419-normalized-shadow-exit-reconnector-deprecated-stub-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-420-normalized-shadow-known-intrinsic-comment-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-421-normalized-shadow-known-intrinsic-comment-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-422-normalized-shadow-anf-status-wording-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-423-normalized-shadow-anf-status-wording-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-424-normalized-shadow-public-surface-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-425-normalized-shadow-public-surface-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-426-normalized-shadow-suffix-router-owner-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-427-normalized-shadow-suffix-router-owner-move-card.md
  - docs/development/current/main/phases/phase-291x/291x-428-normalized-shadow-if-only-fossil-boundary-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-429-normalized-shadow-if-only-fossil-boundary-note-card.md
  - docs/development/current/main/phases/phase-291x/291x-430-cleanup-closeout-granularity-card.md
  - docs/development/current/main/phases/phase-291x/291x-431-normalized-shadow-loop-if-exit-wording-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-432-normalization-decline-wording-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-433-normalization-default-path-wording-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-434-loop-if-break-continue-placeholder-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-435-loop-if-break-continue-scope-wording-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-436-cleanup-burst-closeout-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-437-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-438-stageb-adapter-thinning-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-439-stageb-output-boundary-helper-card.md
  - docs/development/current/main/phases/phase-291x/291x-440-stageb-disabled-funcscan-harness-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-441-stageb-adapter-thinning-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-442-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-443-generic-method-route-metadata-string-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-444-generic-method-route-kind-metadata-helper-card.md
  - docs/development/current/main/phases/phase-291x/291x-445-generic-method-route-metadata-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-446-rustfmt-drift-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-447-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-448-generic-method-route-surface-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-449-generic-method-route-surface-record-card.md
  - docs/development/current/main/phases/phase-291x/291x-450-generic-method-route-surface-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-451-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-452-generic-method-route-decision-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-453-generic-method-route-decision-record-card.md
  - docs/development/current/main/phases/phase-291x/291x-454-generic-method-route-decision-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-455-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-456-generic-method-route-evidence-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-457-generic-method-route-evidence-record-card.md
  - docs/development/current/main/phases/phase-291x/291x-458-generic-method-route-evidence-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-459-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-460-generic-method-route-site-operands-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-461-generic-method-route-site-operands-record-card.md
  - docs/development/current/main/phases/phase-291x/291x-462-generic-method-route-site-operands-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-463-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-464-generic-method-route-constructor-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-465-generic-method-route-constructor-card.md
  - docs/development/current/main/phases/phase-291x/291x-466-generic-method-route-constructor-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-467-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-468-generic-method-route-component-field-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-469-generic-method-route-component-field-privacy-card.md
  - docs/development/current/main/phases/phase-291x/291x-470-generic-method-route-component-field-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-471-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-472-generic-method-route-root-export-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-473-generic-method-route-root-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-474-generic-method-route-root-export-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-475-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-476-generic-method-route-visibility-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-477-generic-method-route-visibility-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-478-generic-method-route-visibility-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-479-phase-readme-checkpoint-refresh-card.md
  - docs/development/current/main/phases/phase-291x/291x-480-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-481-generic-method-route-fixture-construction-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-482-generic-method-route-fixture-construction-isolation-card.md
  - docs/development/current/main/phases/phase-291x/291x-483-generic-method-route-fixture-construction-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-484-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-485-map-lookup-fusion-route-field-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-486-map-lookup-fusion-route-field-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-487-map-lookup-fusion-route-field-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-488-current-task-order-baseline-refresh-card.md
  - docs/development/current/main/phases/phase-291x/291x-635-current-task-order-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-655-unified-member-property-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-656-parser-member-syntax-ssot-card.md
  - docs/development/current/main/phases/phase-291x/291x-657-planner-reject-detail-diagnostics-card.md
  - docs/development/current/main/phases/phase-291x/291x-658-lower-planner-compat-test-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-659-generic-loop-canon-reverse-export-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-660-body-local-route-facade-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-661-digitpos-reject-message-test-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-662-loop-cond-feature-pipeline-reexport-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-663-if-phi-join-facts-alias-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-664-loop-continue-only-recipe-reexport-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-665-if-phi-join-recipe-reexport-prune-card.md
---

# Phase 291x: CoreBox Surface Contract Cleanup

- Status: active reference lane
- Latest landed cleanup target: read `latest_card_path` in
  `docs/development/current/main/CURRENT_STATE.toml`
- Next implementation target: choose the next compiler-cleanliness lane
- Canonical smoke index:
  `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
- Sibling guardrail: phase-137x remains observe-only unless app work produces
  a real blocker

## Navigation

Read these first:

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md`
3. `docs/development/current/main/phases/phase-291x/291x-665-if-phi-join-recipe-reexport-prune-card.md`
4. `docs/development/current/main/phases/phase-291x/291x-488-current-task-order-baseline-refresh-card.md`
5. `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
6. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
7. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
8. `docs/development/current/main/design/mir-root-facade-contract-ssot.md`

Detailed card history is intentionally not duplicated here. Use the numbered
`291x-*` card files as the ledger.

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

CoreMethodContract cleanup continues as small cards. Do not mix Stage-B adapter
thinning, CoreMethod carrier migration, `.inc` mirror pruning, and hot lowering
in one card.

## Current Rule

- docs-first before code
- current docs update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- zero-cost hot boundary and CoreMethodContract migration policy:
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
- existing guarded `.inc` mirrors may remain during migration
- new method-name classifier growth needs a contract row, deletion condition,
  and focused guard
- new card work should update `CURRENT_STATE.toml` and the active card first,
  not the restart/current mirrors

## Current Checkpoint

- latest known cleanup checkpoint: `291x-665`
- no-growth baseline: `classifiers=0 rows=0`
- detailed landed history lives in phase card files and the current
  `latest_card_path` in `CURRENT_STATE.toml`
- closed cleanup bursts:
  - `291x-575` ordered plan-side compat residue queue through `291x-634`
  - unified-member property cleanup through `291x-655`
  - parser member syntax SSOT cleanup through `291x-656`
  - planner reject-detail diagnostics cleanup through `291x-657`
  - lower planner compat test-only export pruning through `291x-658`
  - generic-loop canon reverse export pruning through `291x-659`
  - BodyLocalRoute facade pruning through `291x-660`
  - DigitPos reject-message test cleanup through `291x-661`
  - loop-cond feature pipeline re-export pruning through `291x-662`
  - IfPhiJoinFacts alias pruning through `291x-663`
  - LoopContinueOnly recipe re-export pruning through `291x-664`
  - IfPhiJoin recipe re-export pruning through `291x-665`
- next cleanup: choose the next compiler-cleanliness lane; broad
  facts/planner and lower planner compat ownership work must reopen as focused
  BoxShape lanes
- normalized-shadow / normalization cleanup burst is closed; larger findings
  must move to a new lane
- Stage-B adapter thinning stays BoxShape-only; do not mix it with
  CoreMethodContract/CoreOp or MapGet proof/lowering work
- has fallback series: closed and inventoried
- no `.inc` method/box string classifier rows are allowlisted
- metadata-absent direct `MapBox.has` is no longer a supported boundary

## Task Families

| Family | Source |
| --- | --- |
| CoreBox surface catalog design | `291x-90-corebox-surface-catalog-design-brief.md` |
| Surface inventory | `291x-92-corebox-surface-inventory-ledger.md` |
| Current task-order closeout | `291x-635-current-task-order-closeout-card.md` |
| Unified member property closeout | `291x-655-unified-member-property-closeout-card.md` |
| Parser member syntax SSOT | `291x-656-parser-member-syntax-ssot-card.md` |
| Planner reject-detail diagnostics | `291x-657-planner-reject-detail-diagnostics-card.md` |
| Lower planner compat test export prune | `291x-658-lower-planner-compat-test-export-prune-card.md` |
| Generic-loop canon reverse export prune | `291x-659-generic-loop-canon-reverse-export-prune-card.md` |
| BodyLocalRoute facade prune | `291x-660-body-local-route-facade-prune-card.md` |
| DigitPos reject-message test contract | `291x-661-digitpos-reject-message-test-contract-card.md` |
| Loop-cond feature pipeline re-export prune | `291x-662-loop-cond-feature-pipeline-reexport-prune-card.md` |
| IfPhiJoinFacts alias prune | `291x-663-if-phi-join-facts-alias-prune-card.md` |
| LoopContinueOnly recipe re-export prune | `291x-664-loop-continue-only-recipe-reexport-prune-card.md` |
| IfPhiJoin recipe re-export prune | `291x-665-if-phi-join-recipe-reexport-prune-card.md` |
| Current task-order baseline | `291x-488-current-task-order-baseline-refresh-card.md` |
| Historical post-birth row-prune order | `291x-255-post-birth-cleanup-task-order-card.md` |
| CoreMethodContract / Hotline policy | `docs/development/current/main/design/hotline-core-method-contract-ssot.md` |
| Smoke selection | `291x-smoke-index.md` |

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
tools/checks/dev_gate.sh quick
```

Run `cargo check -q` when Rust code changed. Keep heavy perf ladders for
explicit perf cards or milestone checks.
