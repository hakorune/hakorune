# Integration phase29cc_wsm family

This family is the next live semantic split after `vm_hako_caps`, and it currently hosts the `g3_canvas`, `g2_browser`, `g4`, `p10`, `p5`, `p6`, and `p7` subfamilies.

## Active Split

- `g3_canvas/`
  - Canvas contract smokes (`arc`, `beginpath`, `clear`, `drawline`, `fill`, `fillcircle`, `setfillstyle`, `setlinewidth`, `setstrokestyle`, `stroke`, `strokerect`)
  - helper: `g3_canvas/lib/phase29cc_wsm_g3_canvas_contract_common_vm.sh`
- `g2_browser/`
  - Browser bridge baseline for WSM-G2
  - browser / build bridge cases only
- `g4/`
  - Playground/browser progression for WSM-G4
  - browser / wasm fixture parity cases only
- `p10/`
  - loop/extern native promotion progression
  - shared helper: `p10/phase29cc_wsm_p10_common.sh`
- `p5/`
  - route-trace / default-lane progression for WSM-P5
  - shared helper: `p5/phase29cc_wsm_p5_route_trace_common.sh`
- `p6/`
  - route-policy freeze pin for WSM-P6
  - shared helper: `p6/phase29cc_wsm_p6_common.sh`
- `p7/`
  - default hako-only guard and compat retention progression for WSM-P7
  - shared helper: `p7/phase29cc_wsm_p7_common.sh`

## Migration Note

- The remaining `phase29cc_wsm_*` scripts still live under `tools/smokes/v2/profiles/integration/apps/`.
- Keep new `phase29cc_wsm` work under this family tree; do not add more `phase29cc_wsm_*` files to `apps/`.
- The next subfamily to inspect is `p8/`.
