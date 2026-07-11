# P381GJ Current Mirror Thinning

Date: 2026-05-06
Scope: doc compaction / mirror thinning after T6 closeout.

## Decision

Current mirrors should show the restart path and next action only. Landed T5/T6
detail belongs in the individual P381 cards, not in `CURRENT_TASK.md`,
`05-Restart-Quick-Resume.md`, `10-Now.md`, or the near-term inventory cards.

## Change

- Thinned `CURRENT_TASK.md` to point at targeted helper dedup only when a local
  owner seam is clear.
- Thinned `05-Restart-Quick-Resume.md` and `10-Now.md` to avoid replaying the
  T6 sequence.
- Compressed `P381FI-STAGE0-CLEANUP-REMAINING-INVENTORY.md` into category-level
  status plus optional polish.
- Compressed `P381FN-CONCRETE-BLOCKER-ORDER.md` into current order and pointers
  rather than listing every landed P381 card.

## Result

The restart path now points at:

1. `CURRENT_STATE.toml`
2. latest card path from `CURRENT_STATE.toml`
3. P381FI/P381FN for targeted helper-dedup order

The detailed T6 trail remains in:

- `P381GC-SMOKE-ARCHIVE-INVENTORY-LOCK.md`
- `P381GD-SMOKE-INVENTORY-REPORT-CLASS-COLUMN.md`
- `P381GE-SMOKE-ARCHIVE-FIRST-CANDIDATE-LIST.md`
- `P381GF-SMOKE-ARCHIVE-FIRST-DELETE-WAVE.md`
- `P381GG-LEGACY-ROOT-SMOKE-LIFECYCLE.md`
- `P381GH-LEGACY-ROOT-SMOKE-DELETE.md`
- `P381GI-SMOKE-REFERENCED-HOLDS-CLOSEOUT.md`

## Next

Targeted helper dedup is allowed only when a local owner seam is clear. If a
real Stage0 expressivity blocker appears, switch to BoxCount mode instead:

```text
1 accepted shape = fixture + gate = 1 commit
```

## Validation

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
