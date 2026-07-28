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
use crate::mir::builder::callable_declaration_catalog::SameModuleCallableNamespaceV1;
use crate::mir::builder::calls::{CatalogHelperChildV1, MethodCallArgumentDescentV1};
use crate::mir::builder::MirBuilder;
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin};
use crate::mir::MirInstruction;
use crate::mir::{MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy)]
struct InlineableSameModuleHelperSetterKeyV1 {
    owner: &'static str,
    method: &'static str,
    arity: usize,
}

const INLINEABLE_SAME_MODULE_HELPER_SETTERS: &[InlineableSameModuleHelperSetterKeyV1] = &[
    InlineableSameModuleHelperSetterKeyV1 {
        owner: "HakoAllocObjectLifecycleAllocResult",
        method: "recordAttempt",
        arity: 0,
    },
    InlineableSameModuleHelperSetterKeyV1 {
        owner: "HakoAllocObjectLifecycleAllocResult",
        method: "recordSelectedPage",
        arity: 1,
    },
    InlineableSameModuleHelperSetterKeyV1 {
        owner: "HakoAllocObjectLifecycleAllocResult",
        method: "recordBlock",
        arity: 1,
    },
    InlineableSameModuleHelperSetterKeyV1 {
        owner: "HakoAllocObjectLifecycleFacade",
        method: "recordLastAllocPage",
        arity: 3,
    },
];

#[derive(Clone)]
struct HelperArgBinding {
    param_name: String,
    value: ValueId,
}

/// Consumer-local snapshot used only to end the immutable catalog borrow
/// before expression lowering mutates the Builder. It is not a declaration
/// store and is never retained in Builder state.
#[derive(Debug)]
struct PreparedHelperDeclarationV1 {
    function_name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    body: Vec<ASTNode>,
}

/// Read-only eligibility evidence for the record-helper inline path. It owns
/// no lowered arguments or Builder effect; execution remains a separate step.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedRecordHelperInlineV1 {
    helper: PreparedHelperDeclarationV1,
    record_arg_indices: Vec<usize>,
}

/// Read-only eligibility evidence for the allowlisted setter inline path.
/// It cannot lower an argument or emit the helper body by itself.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedSameModuleHelperSetterInlineV1 {
    helper: PreparedHelperDeclarationV1,
}

impl MirBuilder {
    pub(in crate::mir::builder) fn try_inline_record_helper_call_with_descent(
        &mut self,
        namespace: SameModuleCallableNamespaceV1,
        owner: &str,
        method: &str,
        args: &[ASTNode],
        receiver: Option<ValueId>,
        descent: &mut dyn MethodCallArgumentDescentV1,
    ) -> Result<Option<ValueId>, String> {
        let Some(prepared) = self.prepare_record_helper_inline(namespace, owner, method, args)?
        else {
            return Ok(None);
        };
        self.execute_prepared_record_helper_inline(prepared, args, receiver, descent)
            .map(Some)
    }

    pub(in crate::mir::builder) fn prepare_record_helper_inline(
        &self,
        namespace: SameModuleCallableNamespaceV1,
        owner: &str,
        method: &str,
        args: &[ASTNode],
    ) -> Result<Option<PreparedRecordHelperInlineV1>, String> {
        let Some(helper) =
            self.prepare_same_module_helper_declaration(namespace, owner, method, args.len())?
        else {
            return Ok(None);
        };
        if helper.params.len() != args.len() {
            return Ok(None);
        }
        let record_arg_indices = self.collect_record_helper_arg_indices(args);
        if record_arg_indices.is_empty() {
            return Ok(None);
        }
        Ok(Some(PreparedRecordHelperInlineV1 {
            helper,
            record_arg_indices,
        }))
    }

    pub(in crate::mir::builder) fn execute_prepared_record_helper_inline(
        &mut self,
        prepared: PreparedRecordHelperInlineV1,
        args: &[ASTNode],
        receiver: Option<ValueId>,
        descent: &mut dyn MethodCallArgumentDescentV1,
    ) -> Result<ValueId, String> {
        let bindings = self.build_record_helper_arg_bindings(
            &prepared.helper.function_name,
            args,
            &prepared.helper.params,
            &prepared.helper.param_decls,
            &prepared.record_arg_indices,
            descent,
        )?;
        self.inline_record_helper_body(
            &prepared.helper.function_name,
            receiver,
            bindings,
            &prepared.helper.body,
            descent,
        )
    }

    pub(in crate::mir::builder) fn prepare_same_module_helper_setter_inline(
        &self,
        owner: &str,
        method: &str,
        args: &[ASTNode],
    ) -> Result<Option<PreparedSameModuleHelperSetterInlineV1>, String> {
        if !is_inlineable_same_module_helper_key(owner, method, args.len()) {
            if crate::config::env::builder_static_call_trace() {
                crate::runtime::get_global_ring0().log.info(&format!(
                    "[same-module-helper-inline] reject allowlist owner={} method={} arity={}",
                    owner,
                    method,
                    args.len()
                ));
            }
            return Ok(None);
        }

        let Some(helper) = self.prepare_same_module_helper_declaration(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            owner,
            method,
            args.len(),
        )?
        else {
            if crate::config::env::builder_static_call_trace() {
                crate::runtime::get_global_ring0().log.info(&format!(
                    "[same-module-helper-inline] reject missing-declaration owner={} method={} arity={}",
                    owner,
                    method,
                    args.len()
                ));
            }
            return Ok(None);
        };
        if helper.params.len() != args.len() {
            if crate::config::env::builder_static_call_trace() {
                crate::runtime::get_global_ring0().log.info(&format!(
                    "[same-module-helper-inline] reject arity-mismatch func={} helper_params={} args={}",
                    helper.function_name,
                    helper.params.len(),
                    args.len()
                ));
            }
            return Ok(None);
        }
        if !is_inlineable_same_module_helper_body(&helper.body) {
            if crate::config::env::builder_static_call_trace() {
                crate::runtime::get_global_ring0().log.info(&format!(
                    "[same-module-helper-inline] reject body-shape func={}",
                    helper.function_name
                ));
            }
            return Ok(None);
        }

        Ok(Some(PreparedSameModuleHelperSetterInlineV1 { helper }))
    }

    pub(in crate::mir::builder) fn execute_prepared_same_module_helper_setter_inline(
        &mut self,
        prepared: PreparedSameModuleHelperSetterInlineV1,
        args: &[ASTNode],
        receiver: Option<ValueId>,
        descent: &mut dyn MethodCallArgumentDescentV1,
    ) -> Result<ValueId, String> {
        let helper = prepared.helper;
        let mut bindings = Vec::with_capacity(args.len());
        for (index, param_name) in helper.params.iter().enumerate() {
            let value = descent.lower_index(self, index)?;
            bindings.push(HelperArgBinding {
                param_name: param_name.clone(),
                value,
            });
        }

        if crate::config::env::builder_static_call_trace() {
            crate::runtime::get_global_ring0().log.info(&format!(
                "[same-module-helper-inline] accept func={} receiver={:?} args={}",
                helper.function_name,
                receiver.map(|v| v.0),
                args.len()
            ));
        }
        self.inline_record_helper_body(
            &helper.function_name,
            receiver,
            bindings,
            &helper.body,
            descent,
        )
    }

    pub(in crate::mir::builder) fn prepare_same_module_helper_setter_inline_from_receiver(
        &self,
        object_value: ValueId,
        method: &str,
        args: &[ASTNode],
    ) -> Result<Option<PreparedSameModuleHelperSetterInlineV1>, String> {
        let Some(box_name) = self.infer_same_module_helper_receiver_box_name(object_value) else {
            if crate::config::env::builder_static_call_trace() {
                crate::runtime::get_global_ring0().log.info(&format!(
                    "[same-module-helper-inline] reject receiver-box-miss receiver=%{} method={}",
                    object_value.0, method
                ));
            }
            return Ok(None);
        };
        if crate::config::env::builder_static_call_trace() {
            crate::runtime::get_global_ring0().log.info(&format!(
                "[same-module-helper-inline] candidate receiver=%{} box={} method={} arity={}",
                object_value.0,
                box_name,
                method,
                args.len()
            ));
        }
        self.prepare_same_module_helper_setter_inline(&box_name, method, args)
    }

    fn prepare_same_module_helper_declaration(
        &self,
        namespace: SameModuleCallableNamespaceV1,
        owner: &str,
        method: &str,
        arity: usize,
    ) -> Result<Option<PreparedHelperDeclarationV1>, String> {
        let declaration = self
            .comp_ctx
            .callable_declaration(namespace, owner, method, arity)
            .map_err(|error| error.to_string())?;
        Ok(declaration.map(|declaration| PreparedHelperDeclarationV1 {
            function_name: declaration.key().mir_symbol_projection(),
            params: declaration.params().to_vec(),
            param_decls: declaration.param_decls().to_vec(),
            body: declaration.body().to_vec(),
        }))
    }

    fn collect_record_helper_arg_indices(&self, args: &[ASTNode]) -> Vec<usize> {
        args.iter()
            .enumerate()
            .filter_map(|(idx, arg)| {
                let ASTNode::Variable { name, .. } = arg else {
                    return None;
                };
                let value = self
                    .function_state
                    .variable_ctx
                    .variable_map
                    .get(name)
                    .copied()?;
                self.function_state.compilation.record_local_value(value)?;
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
        descent: &mut dyn MethodCallArgumentDescentV1,
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
                    .function_state
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
                let record = self
                    .function_state
                    .compilation
                    .record_local_value(value)
                    .ok_or_else(|| {
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
                let value = descent.lower_index(self, idx)?;
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
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned()
        {
            return Some(box_name);
        }

        match self.function_state.type_ctx.value_types.get(&object_value) {
            Some(MirType::Box(box_name)) => Some(box_name.clone()),
            _ => {
                if let Some(function) = self.function_state.current_function.as_ref() {
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

        let function = self.function_state.current_function.as_ref()?;
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
        if let Some(MirType::Box(box_name)) = self.function_state.type_ctx.value_types.get(&value) {
            return Some(box_name.clone());
        }

        if let Some(box_name) = self
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&value)
            .cloned()
        {
            return Some(box_name);
        }

        if let Some(function) = self.function_state.current_function.as_ref() {
            if let Some(MirType::Box(box_name)) = function.metadata.value_types.get(&value) {
                return Some(box_name.clone());
            }
        }

        None
    }

    fn resolve_same_module_helper_receiver_origin_value(&self, object_value: ValueId) -> ValueId {
        let Some(function) = self.function_state.current_function.as_ref() else {
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
        descent: &mut dyn MethodCallArgumentDescentV1,
    ) -> Result<ValueId, String> {
        let saved_var_map = self.function_state.variable_ctx.variable_map.clone();

        if let Some(receiver) = receiver {
            self.function_state
                .variable_ctx
                .variable_map
                .insert("me".to_string(), receiver);
        }
        for binding in bindings {
            self.function_state
                .variable_ctx
                .variable_map
                .insert(binding.param_name, binding.value);
        }

        let result = self.lower_record_helper_body_until_return(func_name, body, descent);
        self.function_state.variable_ctx.variable_map = saved_var_map;
        result
    }

    fn lower_record_helper_body_until_return(
        &mut self,
        func_name: &str,
        body: &[ASTNode],
        descent: &mut dyn MethodCallArgumentDescentV1,
    ) -> Result<ValueId, String> {
        for stmt in body {
            if let ASTNode::Return { value, .. } = stmt {
                return match value {
                    Some(expr) => descent.lower_catalog_helper_child(
                        self,
                        CatalogHelperChildV1::Expression(*expr.clone()),
                    ),
                    None => crate::mir::builder::emission::constant::emit_void(self),
                };
            }
            descent
                .lower_catalog_helper_child(self, CatalogHelperChildV1::Statement(stmt.clone()))?;
        }

        Err(format!(
            "[record-helper-arg/missing-return] func={}",
            func_name
        ))
    }
}

fn is_inlineable_same_module_helper_key(owner: &str, method: &str, arity: usize) -> bool {
    INLINEABLE_SAME_MODULE_HELPER_SETTERS
        .iter()
        .any(|key| key.owner == owner && key.method == method && key.arity == arity)
}

fn is_inlineable_same_module_helper_body(body: &[ASTNode]) -> bool {
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
#[path = "record_helper_args_tests.rs"]
mod tests;
