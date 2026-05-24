# Hako Alloc Mimalloc Comparison Huge-Ish EXE Proof

Row: `295x-25`

This app mirrors the selected `representative-huge-ish-v0` request sequence in
`.hako` model space. It uses the huge page model plus one small page request,
but does not claim OSVM/page-source equivalence with C mimalloc.

Run:

```bash
bash tools/checks/k2_wide_phase295x_huge_ish_contract_refresh_guard.sh
```
