# 225x-90 Generic Placement-Effect Transform Owner Seam SSOT

Status: Landed

Goal
- make the first MIR-side generic placement/effect transform pass the top-level owner instead of calling one family-specific transform directly from the optimizer

Landing
- optimizer pre/post-DCE placement/effect hooks now call one owner seam
- the owner seam delegates to the landed string corridor sink in this cut
- behavior does not change; this is a BoxShape landing for future widening

Exit
- the next transform widening can happen under the generic owner seam without reopening optimizer-level family-specific wiring
