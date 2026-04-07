/*!
 * VM user-factory helper.
 *
 * Purpose:
 * - Keep VM lanes from owning inline user-box factory plumbing directly.
 * - Keep static user-box registration and fallback factory setup under one thin owner.
 */

use crate::backend::MirInterpreter;
use crate::core::model::BoxDeclaration as CoreBoxDecl;
use nyash_rust::ast::ASTNode;
use std::collections::HashMap;

pub(crate) struct VmUserFactoryState {
    static_box_decls: HashMap<String, CoreBoxDecl>,
}

impl VmUserFactoryState {
    pub(crate) fn static_box_count(&self) -> usize {
        self.static_box_decls.len()
    }

    pub(crate) fn register_static_box_decls(&self, vm: &mut MirInterpreter) {
        for (name, decl) in &self.static_box_decls {
            vm.register_static_box_decl(name.clone(), decl.clone());
        }
    }
}

pub(crate) fn prepare_vm_user_factory(
    ast: &ASTNode,
    include_static_box_decls: bool,
    alias_static_instances: bool,
) -> VmUserFactoryState {
    let static_box_decls =
        crate::runner::modes::common_util::user_box_factory::install_inline_user_box_factory(
            ast,
            include_static_box_decls,
            alias_static_instances,
        );
    VmUserFactoryState { static_box_decls }
}
