use super::NormalizedExprLowererBox;
use super::super::expr_lowering_contract::{ImpurePolicy, KnownIntrinsic};
use super::super::known_intrinsics::KnownIntrinsicRegistryBox;
use crate::ast::ASTNode;
use crate::mir::join_ir::JoinInst;
use crate::mir::types::MirType;
use crate::mir::ValueId;
use std::collections::BTreeMap;

impl NormalizedExprLowererBox {
    /// Phase 141 P1.5: Refactored to use KnownIntrinsicRegistryBox
    pub(super) fn match_known_intrinsic_method_call(
        policy: ImpurePolicy,
        object: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
        env: &BTreeMap<String, ValueId>,
    ) -> Option<KnownIntrinsic> {
        match policy {
            ImpurePolicy::KnownIntrinsicOnly => {
                let receiver_ok =
                    matches!(object, ASTNode::Variable { name, .. } if env.contains_key(name));
                if !receiver_ok {
                    return None;
                }
                KnownIntrinsicRegistryBox::lookup(method, arguments.len())
            }
        }
    }

    /// Phase 141 P1.5: Refactored to use KnownIntrinsicRegistryBox
    pub(super) fn lower_known_intrinsic_method_call(
        intrinsic: KnownIntrinsic,
        object: &ASTNode,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
        env: &BTreeMap<String, ValueId>,
    ) -> Result<Option<ValueId>, String> {
        let receiver = match object {
            ASTNode::Variable { name, .. } => match env.get(name).copied() {
                Some(v) => v,
                None => return Ok(None),
            },
            _ => return Ok(None),
        };

        let spec = KnownIntrinsicRegistryBox::get_spec(intrinsic);

        let dst = Self::alloc_value_id(next_value_id);
        body.push(JoinInst::MethodCall {
            dst,
            receiver,
            method: spec.method_name.to_string(),
            args: vec![],
            type_hint: Some(MirType::Integer),
        });
        Ok(Some(dst))
    }
}
