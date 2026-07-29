//! One source-only batch for ordinary methods of a non-Main static Box.
//!
//! Program-root and raw-static lifecycles keep their distinct outer state
//! transactions. This owner is the sole projection from the shared method map
//! into ordered, owned inputs for the existing Box-method child port.

use std::collections::HashMap;

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};

use super::recursive_child_lowering::RawBoxMethodChildPortV1;
use super::MirBuilder;

pub(super) struct PreparedNonMainStaticBoxMethodBatchV1 {
    owner: String,
    methods: Box<[PreparedNonMainStaticBoxMethodV1]>,
}

struct PreparedNonMainStaticBoxMethodV1 {
    function_name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
}

impl PreparedNonMainStaticBoxMethodBatchV1 {
    pub(super) fn prepare(owner: String, methods: HashMap<String, ASTNode>) -> Self {
        let mut entries: Vec<(String, ASTNode)> = methods.into_iter().collect();
        entries.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
        let methods = entries
            .into_iter()
            .filter_map(|(method_name, method)| {
                let ASTNode::FunctionDeclaration {
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                    ..
                } = method
                else {
                    return None;
                };
                let function_name = format!("{}.{}/{}", owner, method_name, params.len());
                Some(PreparedNonMainStaticBoxMethodV1 {
                    function_name,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                })
            })
            .collect();
        Self { owner, methods }
    }

    pub(super) fn owner(&self) -> &str {
        &self.owner
    }

    pub(super) fn lower_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<(), String>
    where
        Port: RawBoxMethodChildPortV1,
    {
        for method in self.methods {
            port.lower_static_box_method(
                builder,
                method.function_name,
                method.params,
                method.param_decls,
                method.return_type_name,
                method.body,
                method.uses,
                method.attrs,
            )?;
        }
        Ok(())
    }
}
