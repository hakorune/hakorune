# phase29x vm_hako smoke family

vm-hako lane contract smokes for phase-29x.

This family was split out of `tools/smokes/v2/profiles/integration/apps/` so the
phase-29x S6/newclosure runtime lane can be navigated by meaning instead of by
flat prefix.

## Coverage

- S6 vocabulary inventory guard
- S6 parity gate
- NewClosure fail-fast contract
- NewClosure lane decision refresh

## Suite

- `tools/smokes/v2/suites/integration/phase29x-vm-hako.txt`
