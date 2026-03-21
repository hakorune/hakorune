# Integration phase29cc_wsm family

This family is the next live semantic split after `vm_hako_caps`.

## Active Split

- `g3_canvas/`
  - Canvas contract smokes (`arc`, `beginpath`, `clear`, `drawline`, `fill`, `fillcircle`, `setfillstyle`, `setlinewidth`, `setstrokestyle`, `stroke`, `strokerect`)
  - helper: `g3_canvas/lib/phase29cc_wsm_g3_canvas_contract_common_vm.sh`

## Migration Note

- The remaining `phase29cc_wsm_*` scripts still live under `tools/smokes/v2/profiles/integration/apps/`.
- Keep new `phase29cc_wsm` work under this family tree; do not add more `phase29cc_wsm_*` files to `apps/`.
- The next subfamily to inspect is `g2_browser/`.
