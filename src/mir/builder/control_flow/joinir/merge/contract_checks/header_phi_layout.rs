use crate::mir::builder::control_flow::joinir::merge::boundary_carrier_layout::BoundaryCarrierLayout;
use crate::mir::builder::control_flow::joinir::merge::loop_header_phi_info::LoopHeaderPhiInfo;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;

/// Phase 29af P4: Verify boundary carrier order matches header PHI order.
pub(in crate::mir::builder::control_flow::joinir::merge) fn verify_header_phi_layout(
    boundary: &JoinInlineBoundary,
    loop_header_phi_info: &LoopHeaderPhiInfo,
) -> Result<(), String> {
    use crate::mir::join_ir::lowering::error_tags;

    let strict =
        crate::config::env::joinir_strict_enabled() || crate::config::env::joinir_dev_enabled();
    if !strict {
        return Ok(());
    }

    let boundary_layout = BoundaryCarrierLayout::from_boundary(boundary);
    let boundary_names = boundary_layout.ordered_names();

    let header_names: Vec<&str> = loop_header_phi_info
        .carrier_order
        .iter()
        .map(|name| name.as_str())
        .collect();

    if boundary_names != header_names {
        return Err(error_tags::freeze_with_hint(
            "phase29af/layout/header_phi_order",
            &format!(
                "boundary carrier order {:?} does not match header PHI order {:?}",
                boundary_names, header_names
            ),
            "ensure boundary carrier layout matches header PHI construction order",
        ));
    }

    Ok(())
}
