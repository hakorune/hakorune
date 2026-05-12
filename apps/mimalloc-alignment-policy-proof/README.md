# mimalloc-alignment-policy-proof

M177 proof app. It freezes the standalone alignment policy row before aligned
allocation execution exists.

The proof keeps four boundaries explicit:

1. sub-word alignments normalize up to the current minimum alignment;
2. non-power-of-two and non-positive alignments reject;
3. padded request size is computed without allocating or touching page-map state;
4. huge/unsupported padded requests reject through policy only.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_alignment_policy_guard.sh
```
