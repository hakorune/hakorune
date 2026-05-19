//! RecordHelperArgumentScalarizationBox.
//!
//! C205b record values are builder-local carriers, not runtime objects. This
//! owner handles the narrow helper form where a same-module helper receives a
//! local record argument and field-reads it immediately.

use crate::ast::{ASTNode, ParamDecl};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::BTreeMap;

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
                let value = self.variable_ctx.variable_map.get(name).copied().ok_or_else(|| {
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

        Err(format!("[record-helper-arg/missing-return] func={}", func_name))
    }
}
