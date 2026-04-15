use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;

pub(in crate::mir::builder) type NestedLoopRecipe =
    crate::mir::builder::control_flow::recipes::scan_loop_segments::NestedLoopRecipe;

pub(in crate::mir::builder) type LoopScanPhiSegment =
    crate::mir::builder::control_flow::recipes::scan_loop_segments::LoopScanSegment<NoExitBlockRecipe>;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanPhiVarsV0Segments {
    pub segments: Vec<LoopScanPhiSegment>,
}

/// Recipe for loop_scan_phi_vars_v0 pattern.
/// Stores extracted AST nodes from the outer loop body.
///
/// Accepted body structures:
/// - Classic (7 statements):
///   0. local var_name = "" + common_vars.get(i)
///   1. local j = 0
///   2. local m = break_list.length()
///   3. local found = 0
///   4. loop(j < m) { if cond { found = 1; break } j = j + 1 }
///   5. if found == 1 { local k = 0; loop(k < m) { ... } }
///   6. i = i + 1
/// - EXT-SHAPE-01 (4 statements):
///   0. local j = 0
///   1. local m = ...
///   2. loop(j < m) { ... no exit ... }
///   3. i = i + <non-const expr>
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopScanPhiVarsV0Recipe {
    /// local var_name = "" + common_vars.get(i) (classic shape only)
    pub local_var_name_stmt: Option<ASTNode>,
    /// local j = 0
    pub local_j_stmt: ASTNode,
    /// local m = break_list.length()
    pub local_m_stmt: ASTNode,
    /// local found = 0 (classic shape only)
    pub local_found_stmt: Option<ASTNode>,
    /// Inner loop 1: loop(j < m) { if cond { found = 1; break } j = j + 1 }
    pub inner_loop_search: ASTNode,
    /// if found == 1 { ... inner loop 2 ... } (classic shape only)
    pub found_if_stmt: Option<ASTNode>,
    /// i = i + 1
    pub step_inc_stmt: ASTNode,
}
