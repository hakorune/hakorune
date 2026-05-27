//! RecordHelperArgumentScalarizationBox.
//!
//! C205b record values are builder-local carriers, not runtime objects. This
//! owner handles the narrow helper form where a same-module helper receives a
//! local record argument and field-reads it immediately.
//!
//! It also owns the narrow same-module helper-setter inline seam used by the
//! mimalloc parity lane. That seam is intentionally tiny: only selected
//! same-module setter helpers may inline, and wrapper helpers stay as calls.

use crate::ast::{ASTNode, ParamDecl};
use crate::mir::builder::calls::function_lowering;
use crate::mir::builder::MirBuilder;
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin};
use crate::mir::MirInstruction;
use crate::mir::{MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

const INLINEABLE_SAME_MODULE_HELPER_SETTERS: &[&str] = &[
    "HakoAllocObjectLifecycleAllocResult.recordAttempt/0",
    "HakoAllocObjectLifecycleAllocResult.recordSelectedPage/1",
    "HakoAllocObjectLifecycleAllocResult.recordBlock/1",
    "HakoAllocObjectLifecycleFacade.recordLastAllocPage/3",
];

#[derive(Clone)]
struct HelperArgBinding {
    param_name: String,
    value: ValueId,
}

impl MirBuilder {
    pub(in crate::mir::builder) fn try_inline_record_helper_call(
        &mut self,
        func_name: &str,
        args: &[ASTNode],
        receiver: Option<ValueId>,
    ) -> Result<Option<ValueId>, String> {
        let Some(helper) = self.comp_ctx.lowered_method_ast(func_name).cloned() else {
            return Ok(None);
        };
        if helper.params.len() != args.len() {
            return Ok(None);
        }

        let record_args = self.collect_record_helper_arg_indices(args);
        if record_args.is_empty() {
            return Ok(None);
        }

        let bindings = self.build_record_helper_arg_bindings(
            func_name,
            args,
            &helper.params,
            &helper.param_decls,
            &record_args,
        )?;

        self.inline_record_helper_body(func_name, receiver, bindings, &helper.body)
            .map(Some)
    }

    pub(in crate::mir::builder) fn try_inline_same_module_helper_setter_call(
        &mut self,
        func_name: &str,
        args: &[ASTNode],
        receiver: Option<ValueId>,
    ) -> Result<Option<ValueId>, String> {
        if !INLINEABLE_SAME_MODULE_HELPER_SETTERS.contains(&func_name) {
            if crate::config::env::builder_static_call_trace() {
                crate::runtime::get_global_ring0().log.info(&format!(
                    "[same-module-helper-inline] reject allowlist func={}",
                    func_name
                ));
            }
            return Ok(None);
        }

        let Some(helper) = self.comp_ctx.lowered_method_ast(func_name).cloned() else {
            if crate::config::env::builder_static_call_trace() {
                crate::runtime::get_global_ring0().log.info(&format!(
                    "[same-module-helper-inline] reject missing-ast func={}",
                    func_name
                ));
            }
            return Ok(None);
        };
        if helper.params.len() != args.len() {
            if crate::config::env::builder_static_call_trace() {
                crate::runtime::get_global_ring0().log.info(&format!(
                    "[same-module-helper-inline] reject arity-mismatch func={} helper_params={} args={}",
                    func_name,
                    helper.params.len(),
                    args.len()
                ));
            }
            return Ok(None);
        }
        if !is_inlineable_same_module_helper_body(func_name, &helper.body) {
            if crate::config::env::builder_static_call_trace() {
                crate::runtime::get_global_ring0().log.info(&format!(
                    "[same-module-helper-inline] reject body-shape func={}",
                    func_name
                ));
            }
            return Ok(None);
        }

        let mut bindings = Vec::with_capacity(args.len());
        for (param_name, arg) in helper.params.iter().zip(args.iter()) {
            let value = self.build_expression(arg.clone())?;
            bindings.push(HelperArgBinding {
                param_name: param_name.clone(),
                value,
            });
        }

        if crate::config::env::builder_static_call_trace() {
            crate::runtime::get_global_ring0().log.info(&format!(
                "[same-module-helper-inline] accept func={} receiver={:?} args={}",
                func_name,
                receiver.map(|v| v.0),
                args.len()
            ));
        }
        self.inline_record_helper_body(func_name, receiver, bindings, &helper.body)
            .map(Some)
    }

    pub(in crate::mir::builder) fn try_inline_same_module_helper_setter_call_from_receiver(
        &mut self,
        object_value: ValueId,
        method: &str,
        args: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        let Some(box_name) = self.infer_same_module_helper_receiver_box_name(object_value) else {
            if crate::config::env::builder_static_call_trace() {
                crate::runtime::get_global_ring0().log.info(&format!(
                    "[same-module-helper-inline] reject receiver-box-miss receiver=%{} method={}",
                    object_value.0, method
                ));
            }
            return Ok(None);
        };
        let func_name =
            function_lowering::generate_method_function_name(&box_name, method, args.len());
        if crate::config::env::builder_static_call_trace() {
            crate::runtime::get_global_ring0().log.info(&format!(
                "[same-module-helper-inline] candidate receiver=%{} box={} method={} func={}",
                object_value.0, box_name, method, func_name
            ));
        }
        self.try_inline_same_module_helper_setter_call(&func_name, args, Some(object_value))
    }

    fn collect_record_helper_arg_indices(&self, args: &[ASTNode]) -> Vec<usize> {
        args.iter()
            .enumerate()
            .filter_map(|(idx, arg)| {
                let ASTNode::Variable { name, .. } = arg else {
                    return None;
                };
                let value = self.variable_ctx.variable_map.get(name).copied()?;
                self.comp_ctx.record_local_value(value)?;
                Some(idx)
            })
            .collect()
    }

    fn build_record_helper_arg_bindings(
        &mut self,
        func_name: &str,
        args: &[ASTNode],
        params: &[String],
        param_decls: &[ParamDecl],
        record_arg_indices: &[usize],
    ) -> Result<Vec<HelperArgBinding>, String> {
        let mut record_arg_set = BTreeMap::new();
        for idx in record_arg_indices {
            record_arg_set.insert(*idx, ());
        }

        let declared_params = ParamDecl::with_name_fallback(param_decls, params);
        let mut bindings = Vec::with_capacity(args.len());
        for (idx, (param_name, arg)) in params.iter().zip(args.iter()).enumerate() {
            if record_arg_set.contains_key(&idx) {
                let ASTNode::Variable { name, .. } = arg else {
                    return Err(format!(
                        "[record-helper-arg/internal] func={} arg_index={} expected=variable",
                        func_name, idx
                    ));
                };
                let value = self
                    .variable_ctx
                    .variable_map
                    .get(name)
                    .copied()
                    .ok_or_else(|| {
                        format!(
                        "[record-helper-arg/internal] func={} name={} expected=bound-record-local",
                        func_name, name
                    )
                    })?;
                let record = self.comp_ctx.record_local_value(value).ok_or_else(|| {
                    format!(
                        "[record-helper-arg/internal] func={} name={} expected=record-local",
                        func_name, name
                    )
                })?;
                let declared_type = declared_params
                    .get(idx)
                    .and_then(|decl| decl.declared_type_name.as_deref());
                if declared_type != Some(record.record_name.as_str()) {
                    return Err(format!(
                        "[record-helper-arg/type-mismatch] func={} param={} declared_type={:?} record={}",
                        func_name, param_name, declared_type, record.record_name
                    ));
                }
                bindings.push(HelperArgBinding {
                    param_name: param_name.clone(),
                    value,
                });
            } else {
                let value = self.build_expression(arg.clone())?;
                bindings.push(HelperArgBinding {
                    param_name: param_name.clone(),
                    value,
                });
            }
        }

        Ok(bindings)
    }

    fn infer_same_module_helper_receiver_box_name(&self, object_value: ValueId) -> Option<String> {
        let mut visiting = BTreeSet::new();
        if let Some(box_name) = self.infer_same_module_helper_box_name_from_current_function_value(
            object_value,
            &mut visiting,
        ) {
            return Some(box_name);
        }

        if crate::config::env::builder_use_type_registry() {
            let inferred = self.comp_ctx.type_registry.infer_class(object_value, None);
            if inferred != "UnknownBox" {
                return Some(inferred);
            }
        }

        if let Some(box_name) = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned()
        {
            return Some(box_name);
        }

        match self.type_ctx.value_types.get(&object_value) {
            Some(MirType::Box(box_name)) => Some(box_name.clone()),
            _ => {
                if let Some(function) = self.scope_ctx.current_function.as_ref() {
                    if let Some(MirType::Box(box_name)) =
                        function.metadata.value_types.get(&object_value)
                    {
                        return Some(box_name.clone());
                    }
                }
                None
            }
        }
    }

    fn infer_same_module_helper_box_name_from_current_function_value(
        &self,
        value: ValueId,
        visiting: &mut BTreeSet<ValueId>,
    ) -> Option<String> {
        let origin_value = self.resolve_same_module_helper_receiver_origin_value(value);

        if let Some(box_name) =
            self.infer_same_module_helper_box_name_from_known_value(origin_value)
        {
            return Some(box_name);
        }

        let function = self.scope_ctx.current_function.as_ref()?;
        let def_map = build_value_def_map(function);
        let (block_id, inst_idx) = def_map.get(&origin_value).copied()?;
        let block = function.blocks.get(&block_id)?;
        let inst = block.instructions.get(inst_idx)?;

        match inst {
            MirInstruction::Phi {
                type_hint: Some(MirType::Box(box_name)),
                ..
            } => Some(box_name.clone()),
            MirInstruction::Phi { inputs, .. } => {
                if !visiting.insert(origin_value) {
                    return None;
                }
                let result =
                    self.infer_same_module_helper_box_name_from_phi_inputs(inputs, visiting);
                visiting.remove(&origin_value);
                result
            }
            MirInstruction::FieldGet {
                declared_type: Some(MirType::Box(box_name)),
                ..
            } => Some(box_name.clone()),
            MirInstruction::NewBox { box_type, .. } => Some(box_type.clone()),
            _ => None,
        }
    }

    fn infer_same_module_helper_box_name_from_phi_inputs(
        &self,
        inputs: &[(crate::mir::BasicBlockId, ValueId)],
        visiting: &mut BTreeSet<ValueId>,
    ) -> Option<String> {
        let mut inferred: Option<String> = None;
        for (_, input_value) in inputs {
            let Some(box_name) = self
                .infer_same_module_helper_box_name_from_current_function_value(
                    *input_value,
                    visiting,
                )
            else {
                return None;
            };
            match &inferred {
                None => inferred = Some(box_name),
                Some(current) if current == &box_name => {}
                Some(_) => return None,
            }
        }
        inferred
    }

    fn infer_same_module_helper_box_name_from_known_value(&self, value: ValueId) -> Option<String> {
        if let Some(MirType::Box(box_name)) = self.type_ctx.value_types.get(&value) {
            return Some(box_name.clone());
        }

        if let Some(box_name) = self.type_ctx.value_origin_newbox.get(&value).cloned() {
            return Some(box_name);
        }

        if let Some(function) = self.scope_ctx.current_function.as_ref() {
            if let Some(MirType::Box(box_name)) = function.metadata.value_types.get(&value) {
                return Some(box_name.clone());
            }
        }

        None
    }

    fn resolve_same_module_helper_receiver_origin_value(&self, object_value: ValueId) -> ValueId {
        let Some(function) = self.scope_ctx.current_function.as_ref() else {
            return object_value;
        };
        let def_map = build_value_def_map(function);
        resolve_value_origin(function, &def_map, object_value)
    }

    fn inline_record_helper_body(
        &mut self,
        func_name: &str,
        receiver: Option<ValueId>,
        bindings: Vec<HelperArgBinding>,
        body: &[ASTNode],
    ) -> Result<ValueId, String> {
        let saved_var_map = self.variable_ctx.variable_map.clone();

        if let Some(receiver) = receiver {
            self.variable_ctx
                .variable_map
                .insert("me".to_string(), receiver);
        }
        for binding in bindings {
            self.variable_ctx
                .variable_map
                .insert(binding.param_name, binding.value);
        }

        let result = self.lower_record_helper_body_until_return(func_name, body);
        self.variable_ctx.variable_map = saved_var_map;
        result
    }

    fn lower_record_helper_body_until_return(
        &mut self,
        func_name: &str,
        body: &[ASTNode],
    ) -> Result<ValueId, String> {
        for stmt in body {
            if let ASTNode::Return { value, .. } = stmt {
                return match value {
                    Some(expr) => self.build_expression(*expr.clone()),
                    None => crate::mir::builder::emission::constant::emit_void(self),
                };
            }
            self.build_statement(stmt.clone())?;
        }

        Err(format!(
            "[record-helper-arg/missing-return] func={}",
            func_name
        ))
    }
}

fn is_inlineable_same_module_helper_body(func_name: &str, body: &[ASTNode]) -> bool {
    if !INLINEABLE_SAME_MODULE_HELPER_SETTERS.contains(&func_name) {
        return false;
    }
    let Some((last_stmt, prefix)) = body.split_last() else {
        return false;
    };
    if !matches!(last_stmt, ASTNode::Return { value: Some(_), .. }) {
        return false;
    }

    prefix.iter().all(|stmt| {
        matches!(stmt, ASTNode::Assignment { target, value, .. }
            if is_inlineable_same_module_helper_assignment_target(target.as_ref())
                && is_inlineable_same_module_helper_expr(value.as_ref()))
    }) && matches!(last_stmt, ASTNode::Return { value: Some(expr), .. }
            if is_inlineable_same_module_helper_expr(expr.as_ref()))
}

fn is_inlineable_same_module_helper_assignment_target(node: &ASTNode) -> bool {
    matches!(
        node,
        ASTNode::FieldAccess { object, .. }
            if is_inlineable_same_module_helper_receiver_base(object.as_ref())
    )
}

fn is_inlineable_same_module_helper_receiver_base(node: &ASTNode) -> bool {
    match node {
        ASTNode::Me { .. } | ASTNode::This { .. } => true,
        ASTNode::Variable { name, .. } => name == "me" || name == "this",
        _ => false,
    }
}

fn is_inlineable_same_module_helper_expr(node: &ASTNode) -> bool {
    match node {
        ASTNode::Literal { .. }
        | ASTNode::Variable { .. }
        | ASTNode::Me { .. }
        | ASTNode::This { .. } => true,
        ASTNode::FieldAccess { object, .. } => {
            is_inlineable_same_module_helper_receiver_base(object.as_ref())
        }
        ASTNode::UnaryOp { operand, .. } => is_inlineable_same_module_helper_expr(operand.as_ref()),
        ASTNode::BinaryOp { left, right, .. } => {
            is_inlineable_same_module_helper_expr(left.as_ref())
                && is_inlineable_same_module_helper_expr(right.as_ref())
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::LiteralValue;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction};

    fn span() -> crate::ast::Span {
        crate::ast::Span::unknown()
    }

    fn field_assign(field: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(ASTNode::FieldAccess {
                object: Box::new(ASTNode::Me { span: span() }),
                field: field.to_string(),
                span: span(),
            }),
            value: Box::new(value),
            span: span(),
        }
    }

    fn int_lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    #[test]
    fn inlineable_setter_accepts_simple_assignment_and_return() {
        let body = vec![
            field_assign(
                "attempt_count",
                ASTNode::BinaryOp {
                    operator: crate::ast::BinaryOperator::Add,
                    left: Box::new(ASTNode::FieldAccess {
                        object: Box::new(ASTNode::Me { span: span() }),
                        field: "attempt_count".to_string(),
                        span: span(),
                    }),
                    right: Box::new(int_lit(1)),
                    span: span(),
                },
            ),
            ASTNode::Return {
                value: Some(Box::new(int_lit(1))),
                span: span(),
            },
        ];

        assert!(is_inlineable_same_module_helper_body(
            "HakoAllocObjectLifecycleAllocResult.recordAttempt/0",
            &body
        ));
    }

    #[test]
    fn inlineable_setter_rejects_wrapper_call_body() {
        let body = vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::FunctionCall {
                name: "other".to_string(),
                arguments: Vec::new(),
                span: span(),
            })),
            span: span(),
        }];

        assert!(!is_inlineable_same_module_helper_body(
            "HakoAllocObjectLifecycleFacade.recordSmallAllocFailure/1",
            &body
        ));
    }

    #[test]
    fn infer_same_module_helper_receiver_box_name_follows_phi_inputs_without_hint() {
        let signature = FunctionSignature {
            name: "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let block = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "FooBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(MirInstruction::Phi {
            dst: ValueId::new(3),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(1)),
                (BasicBlockId::new(0), ValueId::new(2)),
            ],
            type_hint: None,
        });

        let mut builder = MirBuilder::new();
        builder.scope_ctx.current_function = Some(function);

        assert_eq!(
            builder
                .infer_same_module_helper_receiver_box_name(ValueId::new(3))
                .as_deref(),
            Some("FooBox")
        );
    }
}
