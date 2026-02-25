//! Entry-like policy (SSOT)
//!
//! Contract:
//! - "entry-like" は MAIN のみを対象にする（by-name 以外の推測は禁止）

use crate::mir::join_ir::lowering::canonical_names;

pub(in crate::mir::builder::control_flow::joinir::merge) fn is_entry_like_source(
    func_name: &str,
) -> bool {
    func_name == canonical_names::MAIN
}
