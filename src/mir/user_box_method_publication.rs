/*!
 * Report-only publication classifier for user-box method fastpath candidates.
 *
 * This module does not create `LocalFastPathFact`. It only classifies whether a
 * user-box direct method route has enough local publication evidence for a
 * later producer row to consider it.
 */

use std::collections::BTreeSet;

use crate::mir::value_origin::{build_value_def_map, resolve_value_origin};
use crate::mir::{Callee, MirFunction, MirInstruction, ValueId};
use crate::object_storage_plan::PublicationState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBoxMethodPublicationClassification {
    block: u32,
    instruction_index: usize,
    receiver_value: ValueId,
    route_plan_label: &'static str,
    origin_kind: &'static str,
    publication_state: PublicationState,
    proof: &'static str,
}

impl UserBoxMethodPublicationClassification {
    pub fn block(&self) -> u32 {
        self.block
    }

    pub fn instruction_index(&self) -> usize {
        self.instruction_index
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn route_plan_label(&self) -> &'static str {
        self.route_plan_label
    }

    pub fn origin_kind(&self) -> &'static str {
        self.origin_kind
    }

    pub fn publication_state(&self) -> PublicationState {
        self.publication_state
    }

    pub fn publication_state_tag(&self) -> &'static str {
        match self.publication_state {
            PublicationState::Unpublished => "unpublished",
            PublicationState::Published => "published",
            PublicationState::MaybePublished => "maybe_published",
        }
    }

    pub fn proof(&self) -> &'static str {
        self.proof
    }

    pub fn fact_allowed(&self) -> bool {
        self.publication_state == PublicationState::Unpublished
    }
}

pub fn refresh_function_user_box_method_publication_classifications(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut rows = Vec::new();

    for route in &function.metadata.user_box_method_routes {
        if route.reason().is_some() {
            continue;
        }
        let origin = resolve_value_origin(function, &def_map, route.receiver_value());
        let (origin_kind, publication_state, proof) = classify_receiver_at_site(
            function,
            &def_map,
            origin,
            route.receiver_value(),
            route.block().as_u32(),
            route.instruction_index(),
        );
        rows.push(UserBoxMethodPublicationClassification {
            block: route.block().as_u32(),
            instruction_index: route.instruction_index(),
            receiver_value: route.receiver_value(),
            route_plan_label: route.route_id(),
            origin_kind,
            publication_state,
            proof,
        });
    }

    rows.sort_by_key(|row| {
        (
            row.block,
            row.instruction_index,
            row.receiver_value.as_u32(),
        )
    });
    function
        .metadata
        .user_box_method_publication_classifications = rows;
}

fn classify_receiver_at_site(
    function: &MirFunction,
    def_map: &crate::mir::value_origin::ValueDefMap,
    origin: ValueId,
    receiver: ValueId,
    route_block: u32,
    route_instruction_index: usize,
) -> (&'static str, PublicationState, &'static str) {
    if function.params.contains(&origin) {
        return (
            "param",
            PublicationState::MaybePublished,
            "param_origin_requires_interprocedural_publication_proof",
        );
    }

    let Some((origin_block, origin_index)) = def_map.get(&origin).copied() else {
        return (
            "unknown",
            PublicationState::MaybePublished,
            "origin_definition_unknown",
        );
    };
    let Some(origin_inst) = function
        .blocks
        .get(&origin_block)
        .and_then(|block| block.instructions.get(origin_index))
    else {
        return (
            "unknown",
            PublicationState::MaybePublished,
            "origin_instruction_missing",
        );
    };

    match origin_inst {
        MirInstruction::NewBox { .. } if origin_block.as_u32() == route_block => {
            if origin_index > route_instruction_index {
                return (
                    "local_newbox_same_block",
                    PublicationState::MaybePublished,
                    "origin_after_route_site",
                );
            }
            if same_block_alias_published_before_site(
                function,
                def_map,
                origin,
                receiver,
                origin_block.as_u32(),
                origin_index,
                route_instruction_index,
            ) {
                (
                    "local_newbox_same_block",
                    PublicationState::MaybePublished,
                    "same_block_alias_maybe_published_before_site",
                )
            } else {
                (
                    "local_newbox_same_block",
                    PublicationState::Unpublished,
                    "same_block_newbox_no_alias_publication_before_site",
                )
            }
        }
        MirInstruction::NewBox { .. } => (
            "local_newbox_other_block",
            PublicationState::MaybePublished,
            "cross_block_publication_requires_dominance_proof",
        ),
        MirInstruction::Call { .. } => (
            "call_result",
            PublicationState::MaybePublished,
            "call_result_requires_callee_publication_summary",
        ),
        MirInstruction::Phi { .. } => (
            "phi",
            PublicationState::MaybePublished,
            "phi_merge_publication_not_proven",
        ),
        MirInstruction::FieldGet { .. } => (
            "field_get",
            PublicationState::MaybePublished,
            "field_get_origin_is_published_object_state",
        ),
        MirInstruction::Copy { .. } => (
            "copy",
            PublicationState::MaybePublished,
            "copy_origin_unresolved",
        ),
        _ => (
            "other",
            PublicationState::MaybePublished,
            "origin_kind_not_accepted_by_v0",
        ),
    }
}

fn same_block_alias_published_before_site(
    function: &MirFunction,
    def_map: &crate::mir::value_origin::ValueDefMap,
    origin: ValueId,
    receiver: ValueId,
    block_id: u32,
    origin_index: usize,
    route_instruction_index: usize,
) -> bool {
    let Some(block) = function
        .blocks
        .get(&crate::mir::BasicBlockId::new(block_id))
    else {
        return true;
    };
    let mut aliases = BTreeSet::new();
    aliases.insert(origin);
    aliases.insert(receiver);
    for index in origin_index..route_instruction_index {
        let Some(inst) = block.instructions.get(index) else {
            return true;
        };
        if let Some(dst) = inst.dst_value() {
            if resolve_value_origin(function, def_map, dst) == origin {
                aliases.insert(dst);
            }
        }
        if instruction_publishes_any_alias(inst, &aliases) {
            return true;
        }
    }
    false
}

fn instruction_publishes_any_alias(inst: &MirInstruction, aliases: &BTreeSet<ValueId>) -> bool {
    match inst {
        MirInstruction::FieldSet { base, value, .. } => {
            aliases.contains(base) || aliases.contains(value)
        }
        MirInstruction::Store { value, ptr } => aliases.contains(value) || aliases.contains(ptr),
        MirInstruction::Return { value } => value.is_some_and(|value| aliases.contains(&value)),
        MirInstruction::Call {
            callee, args, func, ..
        } => {
            aliases.contains(func)
                || method_receiver_is_alias(callee, aliases)
                || args.iter().any(|arg| aliases.contains(arg))
        }
        _ => false,
    }
}

fn method_receiver_is_alias(callee: &Option<Callee>, aliases: &BTreeSet<ValueId>) -> bool {
    matches!(
        callee,
        Some(Callee::Method {
            receiver: Some(receiver),
            ..
        }) if aliases.contains(receiver)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::function::TypedObjectPlan;
    use crate::mir::{
        BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirModule, MirType,
    };

    fn add_pair_sum(module: &mut MirModule) {
        module
            .metadata
            .user_box_decls
            .insert("Pair".to_string(), vec!["left".to_string()]);
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Pair".to_string(),
            type_id: 7,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut sum = MirFunction::new(
            FunctionSignature {
                name: "Pair.sum/0".to_string(),
                params: vec![MirType::Box("Pair".to_string())],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        sum.params = vec![ValueId::new(0)];
        let mut sum_block = BasicBlock::new(BasicBlockId::new(0));
        sum_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(30),
        });
        sum_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        sum.add_block(sum_block);
        module.add_function(sum);
    }

    fn pair_sum_call(receiver: ValueId) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Pair".to_string(),
                method: "sum".to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        }
    }

    #[test]
    fn classifier_allows_same_block_newbox_before_method_call() {
        let mut module = MirModule::new("user_box_method_publication_test".to_string());
        add_pair_sum(&mut module);

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "Pair".to_string(),
            args: vec![],
        });
        block.add_instruction(pair_sum_call(ValueId::new(1)));
        main.add_block(block);
        module.add_function(main);

        crate::mir::user_box_method_route_plan::refresh_module_user_box_method_routes(&mut module);
        let main = module.get_function_mut("main").expect("main");
        refresh_function_user_box_method_publication_classifications(main);

        let row = &main.metadata.user_box_method_publication_classifications[0];
        assert_eq!(row.origin_kind(), "local_newbox_same_block");
        assert_eq!(row.publication_state(), PublicationState::Unpublished);
        assert!(row.fact_allowed());
        assert_eq!(
            row.proof(),
            "same_block_newbox_no_alias_publication_before_site"
        );
    }

    #[test]
    fn classifier_rejects_param_receiver_without_interprocedural_proof() {
        let mut module = MirModule::new("user_box_method_publication_param_test".to_string());
        add_pair_sum(&mut module);

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![MirType::Box("Pair".to_string())],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        main.params = vec![ValueId::new(0)];
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(pair_sum_call(ValueId::new(0)));
        main.add_block(block);
        module.add_function(main);

        crate::mir::user_box_method_route_plan::refresh_module_user_box_method_routes(&mut module);
        let main = module.get_function_mut("main").expect("main");
        refresh_function_user_box_method_publication_classifications(main);

        let row = &main.metadata.user_box_method_publication_classifications[0];
        assert_eq!(row.origin_kind(), "param");
        assert_eq!(row.publication_state(), PublicationState::MaybePublished);
        assert!(!row.fact_allowed());
        assert_eq!(
            row.proof(),
            "param_origin_requires_interprocedural_publication_proof"
        );
    }
}
