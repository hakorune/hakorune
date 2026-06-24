---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-082.
Related:
  - docs/development/current/main/phases/phase-296x/296x-580-MIM-PORT-FMEM-081-POST-ROUTE-JOIN-TERMINAL-LADDER-REENTRY-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-579-MIM-PORT-FMEM-080-PAGE-LOCAL-ALLOC-FREE-ROUTE-BODY-JOIN-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-556-MIM-PORT-FMEM-058-TLS-BACKING-TRANSFER-PREFLIGHT.md
---

# 296x-581 MIM-PORT-FMEM-082 Terminal Ladder Refresh Preflight

## Purpose

Add a refreshed terminal-ladder preflight after the page-local alloc/free route
body join producer pilot. This row makes the terminal ladder consume both the
allocation route CFG and the free route CFG through the joined route body
boundary before TLS, owner lifecycle, activation, hook, global allocator, or
winner rows reopen.

## Required Boundaries

```text
terminal ladder refresh behavior remains closed
TLS backing transfer refresh remains closed
owner slot reuse refresh remains closed
abandoned reclaim refresh remains closed
product activation remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
no new MemOp kind
no diagnostic Python-template C bridge retirement
```

## Acceptance Sketch

```text
replacement_front_selected_route=terminal_ladder_refresh_preflight
replacement_front_selected_memop_family=terminal_ladder_refresh
replacement_front_selected_memop_kinds=TerminalLadderRefresh
replacement_front_next_producer_slice=tls_backing_transfer_preflight_refresh

page_local_route_body_join_selected=1
page_local_route_body_join_open=1
page_local_alloc_route_cfg_selected=1
page_local_alloc_route_cfg_lowering_enabled=1
page_local_free_route_cfg_selected=1
page_local_free_route_cfg_lowering_enabled=1

terminal_ladder_refresh_selected=1
terminal_ladder_refresh_open=0
tls_backing_transfer_enabled=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
opening TLS transfer behavior
reopening owner lifecycle behavior
reopening product activation / hook / global allocator / winner claims
adding a new source syntax surface
retiring the diagnostic Python-template C bridge
```

## Implementation Notes

This row is a preflight. It should add report/check evidence that the terminal
ladder now starts from `page_local_route_body_join_open=1`, but it should keep
`terminal_ladder_refresh_open=0`. The producer row can open the refreshed
terminal entry after this preflight is pinned.

## Landed Evidence

```text
replacement_front_selected_route=terminal_ladder_refresh_preflight
replacement_front_selected_memop_family=terminal_ladder_refresh
replacement_front_selected_memop_kinds=TerminalLadderRefresh
replacement_front_next_producer_slice=tls_backing_transfer_preflight_refresh
fastmem_terminal_ladder_refresh_preflight=1
terminal_ladder_refresh_selected=1
terminal_ladder_refresh_open=0
page_local_route_body_join_selected=1
page_local_route_body_join_open=1
page_local_alloc_route_cfg_lowering_enabled=1
page_local_free_route_cfg_lowering_enabled=1
tls_backing_transfer_enabled=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_route_profiles.py tools/hako_check/fastmem_check_profile_functions.py tools/hako_check/fastmem_check_route_rules.py tools/hako_check/fastmem_check_terminal_rules.py tools/hako_check/fastmem_mir_to_llvm_producer_report_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_route_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_body.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
296x-582 MIM-PORT-FMEM-083 TLS backing transfer preflight refresh.
```
