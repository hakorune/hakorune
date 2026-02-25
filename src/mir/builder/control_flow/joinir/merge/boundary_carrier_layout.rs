use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;

/// Phase 29af P3: Boundary carrier layout SSOT (order only).
///
/// Order rule:
/// - loop_var (if any) first
/// - then carriers (from carrier_info if present, otherwise exit_bindings order)
#[derive(Debug, Clone)]
pub struct BoundaryCarrierLayout {
    ordered_names: Vec<String>,
}

impl BoundaryCarrierLayout {
    pub fn from_boundary(boundary: &JoinInlineBoundary) -> Self {
        let mut ordered_names = Vec::new();

        if let Some(loop_var) = boundary.loop_var_name.as_deref() {
            ordered_names.push(loop_var.to_string());
        }

        if let Some(ref carrier_info) = boundary.carrier_info {
            for carrier in carrier_info.carriers.iter() {
                ordered_names.push(carrier.name.clone());
            }
        } else {
            for binding in boundary.exit_bindings.iter() {
                if boundary.loop_var_name.as_deref() == Some(binding.carrier_name.as_str()) {
                    continue;
                }
                ordered_names.push(binding.carrier_name.clone());
            }
        }

        Self { ordered_names }
    }

    pub fn ordered_names(&self) -> Vec<&str> {
        self.ordered_names.iter().map(|name| name.as_str()).collect()
    }

    pub fn ordered_arg_index(&self, name: &str) -> Option<usize> {
        self.ordered_names.iter().position(|n| n == name)
    }

}
