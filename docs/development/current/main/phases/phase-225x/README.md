# Phase 225x: generic placement-effect transform owner seam

Status: Landed

Purpose
- add the first top-level MIR transform owner seam for the `generic placement / effect` lane

Scope
- stop calling the landed string corridor sink directly from the optimizer pipeline
- route pre/post-DCE placement/effect rewrites through one top-level owner seam
- keep behavior unchanged in this cut

Acceptance
- optimizer pre/post-DCE placement/effect steps flow through one owner module
- landed string corridor sink behavior stays intact
- quick gate stays green

Follow-on
- continue widening the top-level `generic placement / effect` transform lane from this owner seam
