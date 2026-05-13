# 293x-251 M207 Page Lifecycle Invariant Freeze

Status: Complete

## Scope

M207 freezes the page-local lifecycle vocabulary opened by M205/M206. It adds a
read-only observer and proof app; it does not add allocator behavior.

## Transition Table

| From | Operation | To | Required facts |
| --- | --- | --- | --- |
| active | `releaseLocal` empties page | retired | `used == 0`, `retired == 1` |
| active | decommit attempt | active | rejected before source execution |
| active | recommit attempt | active | rejected before source execution |
| retired | state-aware decommit | decommitted | marker generation advances |
| decommitted | duplicate decommit | decommitted | rejected before source execution |
| decommitted | `acquire` | decommitted | rejected while marker is active |
| decommitted | recommit integration | recommitted-active | marker generation balances, page reactivates |
| recommitted-active | `acquire` | active | page-local free-list is usable again |

## State Codes

| Code | Meaning |
| --- | --- |
| `0` | missing page/backing |
| `1` | active |
| `2` | retired |
| `3` | decommitted |
| `4` | recommitted-active |

## Owner

```text
lang/src/hako_alloc/memory/page_lifecycle_invariant_box.hako
```

The owner may read heap page/backing state and marker generation counts. It
must not call allocation, release, decommit, recommit, reactivation, page-source,
unreserve, or OS release APIs.

## Acceptance

- Initial heap page is active and acquire-allowed.
- Active page decommit and recommit attempts are rejected before source calls.
- Empty released page is retired and decommit-candidate.
- Decommitted page rejects duplicate decommit and direct acquire.
- Recommit integration moves the page to recommitted-active.
- A second decommit/recommit generation preserves balanced marker generations.
- Pure-first EXE proof output matches the lifecycle matrix.
