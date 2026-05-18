use crate::ast::{ASTNode, EnumMatchArm, LiteralValue};
use crate::mir::{CompareOp, MirInstruction, MirType, ValueId};

impl super::MirBuilder {
    pub(super) fn try_build_guard_let_scopebox(
        &mut self,
        body: Vec<ASTNode>,
    ) -> Result<Option<ValueId>, String> {
        let Some(temp_name) = guard_let_scopebox_subject(&body) else {
            return Ok(None);
        };

        let mut last_value = None;
        for stmt in body {
            last_value = Some(self.build_statement(stmt)?);
        }
        self.variable_ctx.remove(&temp_name);
        self.binding_ctx.remove(&temp_name);
        Ok(Some(last_value.unwrap_or_else(|| self.next_value_id())))
    }

    pub(super) fn build_enum_match_expression(
        &mut self,
        enum_name: String,
        scrutinee: ASTNode,
        arms: Vec<EnumMatchArm>,
        else_expr: Option<Box<ASTNode>>,
    ) -> Result<ValueId, String> {
        if else_expr.is_some() {
            return Err(format!(
                "[freeze:contract][mir_builder/enum_match] `{}` else-arm lowering is outside direct-MIR guard-let MVP",
                enum_name
            ));
        }

        if let Some(projected) =
            self.try_build_guard_let_payload_projection(&enum_name, &scrutinee, &arms)?
        {
            return Ok(projected);
        }

        self.build_guard_let_variant_bool_select(enum_name, scrutinee, arms)
    }

    pub(super) fn try_build_enum_variant_constructor(
        &mut self,
        enum_name: &str,
        variant_name: &str,
        arguments: Vec<ASTNode>,
    ) -> Result<Option<ValueId>, String> {
        let Some(resolved) = self.comp_ctx.resolve_enum_variant(enum_name, variant_name) else {
            return Ok(None);
        };
        if resolved.decl.requires_compat_payload_box() {
            return Err(format!(
                "[freeze:contract][mir_builder/enum_ctor] {}::{} record/tuple payload lowering is outside direct-MIR MVP",
                enum_name, variant_name
            ));
        }
        let tag = resolved.tag;
        let payload_type = payload_mir_type(resolved.decl.payload_type_name.as_deref());
        let expected_arity = resolved.decl.payload_arity();
        if arguments.len() != expected_arity {
            return Err(format!(
                "[freeze:contract][mir_builder/enum_ctor] {}::{} expects {} arg(s), got {}",
                enum_name,
                variant_name,
                expected_arity,
                arguments.len()
            ));
        }
        if crate::semantics::option_contract::requires_non_nullish_payload(enum_name, variant_name)
            && arguments.iter().any(ast_is_statically_nullish)
        {
            return Err(crate::semantics::option_contract::nullish_payload_error(
                "mir_builder/enum_ctor",
            ));
        }

        let arg_values = self.build_call_args(&arguments)?;
        let payload = match arg_values.as_slice() {
            [] => None,
            [payload] => Some(*payload),
            _ => {
                return Err(format!(
                    "[freeze:contract][mir_builder/enum_ctor] multi-payload variants are outside MVP: {}::{}",
                    enum_name, variant_name
                ))
            }
        };
        let dst = self.next_value_id();
        self.emit_instruction(MirInstruction::VariantMake {
            dst,
            enum_name: enum_name.to_string(),
            variant: variant_name.to_string(),
            tag,
            payload,
            payload_type,
        })?;
        self.type_ctx
            .value_types
            .insert(dst, MirType::Box(runtime_variant_box_name(enum_name)));
        Ok(Some(dst))
    }

    fn build_guard_let_variant_bool_select(
        &mut self,
        enum_name: String,
        scrutinee: ASTNode,
        arms: Vec<EnumMatchArm>,
    ) -> Result<ValueId, String> {
        if arms.is_empty() {
            return Err(format!(
                "[freeze:contract][mir_builder/enum_match] `{}` has no arms",
                enum_name
            ));
        }
        self.require_exhaustive_known_arms(&enum_name, &arms)?;
        let mut specs = Vec::with_capacity(arms.len());
        for arm in arms {
            if arm.binding_name.is_some() {
                return Err(format!(
                    "[freeze:contract][mir_builder/enum_match] `{}` bool-select guard shape must not bind payloads",
                    enum_name
                ));
            }
            let Some(value) = bool_literal_body(&arm.body) else {
                return Err(format!(
                    "[freeze:contract][mir_builder/enum_match] `{}` only guard-let boolean variant tests are accepted",
                    enum_name
                ));
            };
            let resolved = self.resolve_known_variant(&enum_name, &arm.variant_name)?;
            specs.push((resolved.tag, value));
        }

        let scrutinee_value = self.build_expression_impl(scrutinee)?;
        let tag_value = self.next_value_id();
        self.emit_instruction(MirInstruction::VariantTag {
            dst: tag_value,
            value: scrutinee_value,
            enum_name,
        })?;
        self.type_ctx
            .value_types
            .insert(tag_value, MirType::Integer);

        let mut specs = specs.into_iter().rev();
        let (_, default_value) = specs
            .next()
            .expect("non-empty checked before reverse lowering");
        let mut result = crate::mir::builder::emission::constant::emit_bool(self, default_value)?;

        for (tag, arm_value) in specs {
            let tag_const =
                crate::mir::builder::emission::constant::emit_integer(self, i64::from(tag))?;
            let cond = self.next_value_id();
            crate::mir::builder::emission::compare::emit_to(
                self,
                cond,
                CompareOp::Eq,
                tag_value,
                tag_const,
            )?;
            let then_val = crate::mir::builder::emission::constant::emit_bool(self, arm_value)?;
            let dst = self.next_value_id();
            self.emit_instruction(MirInstruction::Select {
                dst,
                cond,
                then_val,
                else_val: result,
            })?;
            self.type_ctx.value_types.insert(dst, MirType::Bool);
            result = dst;
        }

        Ok(result)
    }

    fn try_build_guard_let_payload_projection(
        &mut self,
        enum_name: &str,
        scrutinee: &ASTNode,
        arms: &[EnumMatchArm],
    ) -> Result<Option<ValueId>, String> {
        let Some(target_arm) = single_projection_arm(arms) else {
            return Ok(None);
        };
        self.require_exhaustive_known_arms(enum_name, arms)?;
        let resolved = self.resolve_known_variant(enum_name, &target_arm.variant_name)?;
        if !resolved.decl.has_payload() || resolved.decl.requires_compat_payload_box() {
            return Err(format!(
                "[freeze:contract][mir_builder/enum_match] {}::{} payload projection requires a single scalar payload",
                enum_name, target_arm.variant_name
            ));
        }
        let tag = resolved.tag;
        let payload_type = payload_mir_type(resolved.decl.payload_type_name.as_deref());

        let scrutinee_value = self.build_expression_impl(scrutinee.clone())?;
        let dst = self.next_value_id();
        self.emit_instruction(MirInstruction::VariantProject {
            dst,
            value: scrutinee_value,
            enum_name: enum_name.to_string(),
            variant: target_arm.variant_name.clone(),
            tag,
            payload_type: payload_type.clone(),
        })?;
        self.type_ctx
            .value_types
            .insert(dst, payload_type.unwrap_or(MirType::Unknown));
        Ok(Some(dst))
    }

    fn resolve_known_variant(
        &self,
        enum_name: &str,
        variant_name: &str,
    ) -> Result<super::compilation_context::ResolvedEnumVariant<'_>, String> {
        self.comp_ctx
            .resolve_enum_variant(enum_name, variant_name)
            .ok_or_else(|| {
                format!(
                    "[freeze:contract][mir_builder/enum] unknown variant `{}::{}`",
                    enum_name, variant_name
                )
            })
    }

    fn require_exhaustive_known_arms(
        &self,
        enum_name: &str,
        arms: &[EnumMatchArm],
    ) -> Result<(), String> {
        let Some(decl) = self.comp_ctx.enum_decls.get(enum_name) else {
            return Err(format!(
                "[freeze:contract][mir_builder/enum] missing enum inventory for `{}`",
                enum_name
            ));
        };
        if arms.len() != decl.variants.len() {
            return Err(format!(
                "[freeze:contract][mir_builder/enum_match] `{}` non-exhaustive direct-MIR guard-let lowering",
                enum_name
            ));
        }
        for arm in arms {
            if !decl
                .variants
                .iter()
                .any(|variant| variant.name == arm.variant_name)
            {
                return Err(format!(
                    "[freeze:contract][mir_builder/enum_match] `{}` unknown arm variant `{}`",
                    enum_name, arm.variant_name
                ));
            }
        }
        Ok(())
    }
}

fn bool_literal_body(node: &ASTNode) -> Option<bool> {
    match node {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            ..
        } => Some(*value),
        _ => None,
    }
}

fn guard_let_scopebox_subject(body: &[ASTNode]) -> Option<String> {
    let [subject_local, failure_if, binding_local] = body else {
        return None;
    };
    let temp_name = guard_let_subject_temp_name(subject_local)?;
    if !guard_let_failure_if_uses_temp(failure_if, &temp_name) {
        return None;
    }
    if !guard_let_binding_local_uses_temp(binding_local, &temp_name) {
        return None;
    }
    Some(temp_name)
}

fn guard_let_subject_temp_name(node: &ASTNode) -> Option<String> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = node
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 || initial_values[0].is_none() {
        return None;
    }
    let name = variables[0].as_str();
    name.starts_with("__ny_guard_let_subject_")
        .then(|| name.to_string())
}

fn guard_let_failure_if_uses_temp(node: &ASTNode, temp_name: &str) -> bool {
    let ASTNode::If { condition, .. } = node else {
        return false;
    };
    enum_match_scrutinee_is_temp(condition, temp_name)
}

fn guard_let_binding_local_uses_temp(node: &ASTNode, temp_name: &str) -> bool {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = node
    else {
        return false;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return false;
    }
    let Some(initial_value) = initial_values[0].as_deref() else {
        return false;
    };
    enum_match_scrutinee_is_temp(initial_value, temp_name)
}

fn enum_match_scrutinee_is_temp(node: &ASTNode, temp_name: &str) -> bool {
    match node {
        ASTNode::EnumMatchExpr { scrutinee, .. } => matches!(
            scrutinee.as_ref(),
            ASTNode::Variable { name, .. } if name == temp_name
        ),
        _ => false,
    }
}

fn null_literal_body(node: &ASTNode) -> bool {
    matches!(
        node,
        ASTNode::Literal {
            value: LiteralValue::Null,
            ..
        }
    )
}

fn single_projection_arm(arms: &[EnumMatchArm]) -> Option<&EnumMatchArm> {
    let mut target = None;
    for arm in arms {
        match (&arm.binding_name, &arm.body) {
            (Some(binding), ASTNode::Variable { name, .. }) if binding == name => {
                if target.replace(arm).is_some() {
                    return None;
                }
            }
            (None, body) if null_literal_body(body) => {}
            _ => return None,
        }
    }
    target
}

fn payload_mir_type(raw: Option<&str>) -> Option<MirType> {
    let raw = raw?;
    if looks_like_generic_type_param(raw) {
        return None;
    }
    Some(super::MirBuilder::parse_type_name_to_mir(raw))
}

fn looks_like_generic_type_param(raw: &str) -> bool {
    !raw.is_empty()
        && raw
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
}

fn ast_is_statically_nullish(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Literal {
            value: LiteralValue::Null | LiteralValue::Void,
            ..
        } => true,
        ASTNode::BlockExpr { tail_expr, .. } => ast_is_statically_nullish(tail_expr),
        _ => false,
    }
}

fn runtime_variant_box_name(enum_name: &str) -> String {
    format!("__hako_sum_{}", enum_name)
}
