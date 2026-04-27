use super::common::lower_me_this_method_effect;
use super::cond_lowering_prelude::lower_blockexpr_value_prelude_stmts;
use super::helpers_pure_value::is_pure_value_expr;
use super::CoreEffectPlan;
use crate::mir::builder::calls::extern_calls;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::MirBuilder;
use crate::mir::{BinaryOp, ConstValue, Effect, EffectMask, MirType, ValueId};
use std::collections::BTreeMap;

impl super::PlanNormalizer {
    pub(in crate::mir::builder) fn declared_field_type_for_base(
        builder: &MirBuilder,
        base: ValueId,
        field: &str,
    ) -> Option<MirType> {
        builder
            .type_ctx
            .value_origin_newbox
            .get(&base)
            .and_then(|box_name| builder.comp_ctx.declared_field_type_name(box_name, field))
            .map(MirBuilder::parse_type_name_to_mir)
    }

    fn allocate_field_result(builder: &mut MirBuilder, declared_type: &Option<MirType>) -> ValueId {
        match declared_type {
            Some(ty) => {
                let value_id = builder.alloc_typed(ty.clone());
                if let MirType::Box(class_name) = ty {
                    builder
                        .type_ctx
                        .value_origin_newbox
                        .insert(value_id, class_name.clone());
                }
                value_id
            }
            None => {
                let value_id = builder.next_value_id();
                builder.type_ctx.set_type(value_id, MirType::Unknown);
                value_id
            }
        }
    }

    fn arithmetic_result_type(builder: &MirBuilder, lhs: ValueId, rhs: ValueId) -> MirType {
        let lhs_ty = builder.type_ctx.get_type(lhs);
        let rhs_ty = builder.type_ctx.get_type(rhs);
        if matches!(lhs_ty, Some(MirType::Float)) || matches!(rhs_ty, Some(MirType::Float)) {
            MirType::Float
        } else {
            MirType::Integer
        }
    }

    /// Helper: Lower value AST to (ValueId, const_effects)
    /// Returns the ValueId and any Const instructions needed to define literals
    ///
    /// phi_bindings: PHI dst for loop variables (takes precedence over variable_map)
    #[track_caller]
    pub(in crate::mir::builder) fn lower_value_ast(
        ast: &crate::ast::ASTNode,
        builder: &mut MirBuilder,
        phi_bindings: &BTreeMap<String, ValueId>,
    ) -> Result<(ValueId, Vec<CoreEffectPlan>), String> {
        // Keep instruction spans meaningful in the plan-based lowering path too.
        // This mirrors `MirBuilder::build_expression_impl`.
        builder.metadata_ctx.set_current_span(ast.span());

        use crate::ast::{ASTNode, LiteralValue, UnaryOperator};

        match ast {
            ASTNode::Variable { name, .. } => {
                if let Some(&phi_dst) = phi_bindings.get(name) {
                    return Ok((phi_dst, vec![]));
                }
                if let Some(&value_id) = builder.variable_ctx.variable_map.get(name) {
                    Ok((value_id, vec![]))
                } else {
                    Err(format!("[normalizer] Variable {} not found", name))
                }
            }
            ASTNode::FieldAccess { object, field, .. } => {
                let (object_id, mut effects) =
                    Self::lower_value_ast(object, builder, phi_bindings)?;
                let declared_type = Self::declared_field_type_for_base(builder, object_id, field);
                let result_id = Self::allocate_field_result(builder, &declared_type);
                effects.push(CoreEffectPlan::FieldGet {
                    dst: result_id,
                    base: object_id,
                    field: field.clone(),
                    declared_type,
                });
                Ok((result_id, effects))
            }
            ASTNode::Literal { value, span, .. } => {
                let value_id = builder.next_value_id();
                // Diagnostics-only: record literal origin spans only when strict/dev or debug is enabled.
                // This avoids keeping extra state in normal/release compiles.
                if crate::config::env::joinir_dev::debug_enabled()
                    || (crate::config::env::joinir_dev::strict_enabled()
                        && crate::config::env::joinir_dev::planner_required_enabled())
                {
                    builder.metadata_ctx.record_value_span(value_id, *span);
                }
                let (const_value, value_type) = match value {
                    LiteralValue::Integer(n) => (ConstValue::Integer(*n), MirType::Integer),
                    LiteralValue::Float(n) => (ConstValue::Float(*n), MirType::Float),
                    LiteralValue::String(s) => (ConstValue::String(s.clone()), MirType::String),
                    LiteralValue::Bool(b) => (ConstValue::Bool(*b), MirType::Bool),
                    LiteralValue::Null => (ConstValue::Null, MirType::Unknown),
                    LiteralValue::Void => (ConstValue::Void, MirType::Void),
                };

                builder.type_ctx.set_type(value_id, value_type);

                if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                    let caller = std::panic::Location::caller();
                    let ring0 = crate::runtime::get_global_ring0();
                    let fn_name = builder
                        .scope_ctx
                        .current_function
                        .as_ref()
                        .map(|f| f.signature.name.as_str())
                        .unwrap_or("<none>");
                    let next_value_id = builder
                        .scope_ctx
                        .current_function
                        .as_ref()
                        .map(|f| f.next_value_id)
                        .unwrap_or(0);
                    let file = builder
                        .metadata_ctx
                        .current_source_file()
                        .unwrap_or_else(|| "unknown".to_string());
                    ring0.log.debug(&format!(
                        "[lit/lower:alloc] fn={} bb={:?} v=%{} lit={:?} span={} file={} next={} emit=skipped:plan_effect caller={}",
                        fn_name,
                        builder.current_block,
                        value_id.0,
                        value,
                        span.location_string(),
                        file,
                        next_value_id,
                        caller
                    ));
                }

                let const_effect = CoreEffectPlan::Const {
                    dst: value_id,
                    value: const_value,
                };

                Ok((value_id, vec![const_effect]))
            }
            ASTNode::UnaryOp {
                operator, operand, ..
            } => match operator {
                UnaryOperator::Minus => {
                    let (rhs, mut effects) = Self::lower_value_ast(operand, builder, phi_bindings)?;
                    let rhs_ty = builder
                        .type_ctx
                        .get_type(rhs)
                        .cloned()
                        .unwrap_or(MirType::Integer);
                    let (zero_val, zero_ty) = match rhs_ty {
                        MirType::Float => (ConstValue::Float(0.0), MirType::Float),
                        _ => (ConstValue::Integer(0), MirType::Integer),
                    };
                    let zero_id = builder.alloc_typed(zero_ty);
                    effects.push(CoreEffectPlan::Const {
                        dst: zero_id,
                        value: zero_val,
                    });
                    let dst = builder.alloc_typed(rhs_ty);
                    effects.push(CoreEffectPlan::BinOp {
                        dst,
                        lhs: zero_id,
                        op: BinaryOp::Sub,
                        rhs,
                    });
                    Ok((dst, effects))
                }
                UnaryOperator::BitNot => {
                    let (rhs, mut effects) = Self::lower_value_ast(operand, builder, phi_bindings)?;
                    let mask_id = builder.alloc_typed(MirType::Integer);
                    effects.push(CoreEffectPlan::Const {
                        dst: mask_id,
                        value: ConstValue::Integer(-1),
                    });
                    let dst = builder.alloc_typed(MirType::Integer);
                    effects.push(CoreEffectPlan::BinOp {
                        dst,
                        lhs: rhs,
                        op: BinaryOp::BitXor,
                        rhs: mask_id,
                    });
                    Ok((dst, effects))
                }
                UnaryOperator::Not => super::loop_body_lowering::lower_bool_expr(
                    builder,
                    phi_bindings,
                    ast,
                    "[normalizer] value bool unary not",
                ),
                UnaryOperator::Weak => {
                    Err("[normalizer] Unary 'weak' is not supported yet".to_string())
                }
            },
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } => {
                let mut arg_ids = Vec::new();
                let mut arg_effects = Vec::new();
                for arg in arguments {
                    let (arg_id, mut effects) = Self::lower_value_ast(arg, builder, phi_bindings)?;
                    arg_ids.push(arg_id);
                    arg_effects.append(&mut effects);
                }

                let result_id = builder.next_value_id();
                let result_type = match object.as_ref() {
                    ASTNode::Variable { name, .. } if name == "env" => {
                        extern_calls::get_env_method_return_type("env", method)
                            .unwrap_or(MirType::Unknown)
                    }
                    _ => MirType::Unknown,
                };
                builder.type_ctx.set_type(result_id, result_type);

                match object.as_ref() {
                    ASTNode::Variable { name, .. } if name == "env" => {
                        let Some((iface_name, method_name, effects, returns_value)) =
                            extern_calls::get_env_method_spec("env", method)
                        else {
                            return Err(format!(
                                "[normalizer] env method not supported: {}",
                                method
                            ));
                        };
                        if !returns_value {
                            return Err(format!(
                                "[normalizer] env method used as value: {}",
                                method
                            ));
                        }
                        arg_effects.push(CoreEffectPlan::ExternCall {
                            dst: Some(result_id),
                            iface_name,
                            method_name,
                            args: arg_ids,
                            effects,
                        });
                    }
                    ASTNode::Variable { name, .. } => {
                        if let Some(&phi_dst) = phi_bindings.get(name) {
                            arg_effects.push(CoreEffectPlan::MethodCall {
                                dst: Some(result_id),
                                object: phi_dst,
                                method: method.clone(),
                                args: arg_ids,
                                effects: EffectMask::PURE.add(Effect::Io),
                            });
                        } else if let Some(&value_id) = builder.variable_ctx.variable_map.get(name)
                        {
                            arg_effects.push(CoreEffectPlan::MethodCall {
                                dst: Some(result_id),
                                object: value_id,
                                method: method.clone(),
                                args: arg_ids,
                                effects: EffectMask::PURE.add(Effect::Io),
                            });
                        } else if builder.comp_ctx.user_defined_boxes.contains_key(name) {
                            let func = format!("{}.{}/{}", name, method, arguments.len());
                            arg_effects.push(CoreEffectPlan::GlobalCall {
                                dst: Some(result_id),
                                func,
                                args: arg_ids,
                            });
                        } else {
                            return Err(format!(
                                "[normalizer] Method call object {} not found",
                                name
                            ));
                        }
                    }
                    ASTNode::Literal {
                        value: LiteralValue::String(_),
                        ..
                    } => {
                        let (object_id, mut object_effects) =
                            Self::lower_value_ast(object, builder, phi_bindings)?;
                        arg_effects.append(&mut object_effects);
                        arg_effects.push(CoreEffectPlan::MethodCall {
                            dst: Some(result_id),
                            object: object_id,
                            method: method.clone(),
                            args: arg_ids,
                            effects: EffectMask::PURE.add(Effect::Io),
                        });
                    }
                    ASTNode::Me { .. } | ASTNode::This { .. } => {
                        let effect = lower_me_this_method_effect(
                            builder,
                            phi_bindings,
                            object.as_ref(),
                            method,
                            arg_ids,
                            arguments.len(),
                            Some(result_id),
                            "[normalizer] me.method() without bound receiver".to_string(),
                            "[normalizer] this.method() without current_static_box".to_string(),
                        )?;
                        arg_effects.push(effect);
                    }
                    ASTNode::FieldAccess { .. }
                    | ASTNode::ThisField { .. }
                    | ASTNode::MeField { .. } => {
                        let (object_id, mut object_effects) =
                            Self::lower_value_ast(object, builder, phi_bindings)?;
                        arg_effects.append(&mut object_effects);
                        arg_effects.push(CoreEffectPlan::MethodCall {
                            dst: Some(result_id),
                            object: object_id,
                            method: method.clone(),
                            args: arg_ids,
                            effects: EffectMask::PURE.add(Effect::Io),
                        });
                    }
                    ASTNode::MethodCall {
                        object: _callee,
                        method: _,
                        arguments: _,
                        ..
                    } => {
                        // Nested receiver calls must materialize the full inner call result.
                        // Lowering only the inner callee base (for example `arr` in
                        // `arr.get(idx).length()`) loses the receiver chain and
                        // misbinds the outer method to the wrong object.
                        let (object_id, mut object_effects) =
                            Self::lower_value_ast(object, builder, phi_bindings)?;
                        arg_effects.append(&mut object_effects);
                        arg_effects.push(CoreEffectPlan::MethodCall {
                            dst: Some(result_id),
                            object: object_id,
                            method: method.clone(),
                            args: arg_ids,
                            effects: EffectMask::PURE.add(Effect::Io),
                        });
                    }
                    _ => {
                        let (object_id, mut object_effects) =
                            Self::lower_value_ast(object, builder, phi_bindings)?;
                        arg_effects.append(&mut object_effects);
                        arg_effects.push(CoreEffectPlan::MethodCall {
                            dst: Some(result_id),
                            object: object_id,
                            method: method.clone(),
                            args: arg_ids,
                            effects: EffectMask::PURE.add(Effect::Io),
                        });
                    }
                }

                Ok((result_id, arg_effects))
            }
            ASTNode::FunctionCall {
                name, arguments, ..
            } => {
                let mut arg_ids = Vec::new();
                let mut arg_effects = Vec::new();
                for arg in arguments {
                    let (arg_id, mut effects) = Self::lower_value_ast(arg, builder, phi_bindings)?;
                    arg_ids.push(arg_id);
                    arg_effects.append(&mut effects);
                }
                let result_id = builder.next_value_id();
                builder.type_ctx.set_type(result_id, MirType::Unknown);
                arg_effects.push(CoreEffectPlan::GlobalCall {
                    dst: Some(result_id),
                    func: name.clone(),
                    args: arg_ids,
                });
                Ok((result_id, arg_effects))
            }
            ASTNode::Call {
                callee, arguments, ..
            } => {
                let (callee_id, mut callee_effects) =
                    Self::lower_value_ast(callee, builder, phi_bindings)?;
                let mut arg_ids = Vec::new();
                let mut arg_effects = Vec::new();
                for arg in arguments {
                    let (arg_id, mut effects) = Self::lower_value_ast(arg, builder, phi_bindings)?;
                    arg_ids.push(arg_id);
                    arg_effects.append(&mut effects);
                }
                let result_id = builder.next_value_id();
                builder.type_ctx.set_type(result_id, MirType::Unknown);
                arg_effects.append(&mut callee_effects);
                arg_effects.push(CoreEffectPlan::ValueCall {
                    dst: Some(result_id),
                    callee: callee_id,
                    args: arg_ids,
                });
                Ok((result_id, arg_effects))
            }
            ASTNode::New {
                class, arguments, ..
            } => {
                let mut effects = Vec::new();
                let result_id = builder.next_value_id();
                builder
                    .type_ctx
                    .set_type(result_id, MirType::Box(class.clone()));
                let mut arg_ids = Vec::new();
                for arg in arguments {
                    let (arg_id, mut more) = Self::lower_value_ast(arg, builder, phi_bindings)?;
                    effects.append(&mut more);
                    arg_ids.push(arg_id);
                }
                Self::record_newbox_metadata(builder, result_id, class);
                effects.push(CoreEffectPlan::NewBox {
                    dst: result_id,
                    box_type: class.clone(),
                    args: arg_ids,
                });
                Ok((result_id, effects))
            }
            ASTNode::ArrayLiteral { elements, .. } => {
                let mut effects = Vec::new();
                let array_id = builder.next_value_id();
                builder
                    .type_ctx
                    .set_type(array_id, MirType::Box("ArrayBox".to_string()));
                Self::record_newbox_metadata(builder, array_id, "ArrayBox");
                effects.push(CoreEffectPlan::NewBox {
                    dst: array_id,
                    box_type: "ArrayBox".to_string(),
                    args: vec![],
                });
                for element in elements {
                    let (element_id, mut more) =
                        Self::lower_value_ast(element, builder, phi_bindings)?;
                    effects.append(&mut more);
                    effects.push(CoreEffectPlan::MethodCall {
                        dst: None,
                        object: array_id,
                        method: "push".to_string(),
                        args: vec![element_id],
                        effects: EffectMask::PURE.add(Effect::Io),
                    });
                }
                Ok((array_id, effects))
            }
            ASTNode::MapLiteral { entries, .. } => {
                let mut effects = Vec::new();
                let map_id = builder.next_value_id();
                builder
                    .type_ctx
                    .set_type(map_id, MirType::Box("MapBox".to_string()));
                Self::record_newbox_metadata(builder, map_id, "MapBox");
                effects.push(CoreEffectPlan::NewBox {
                    dst: map_id,
                    box_type: "MapBox".to_string(),
                    args: vec![],
                });
                for (key_expr, value) in entries {
                    let key_literal = ASTNode::Literal {
                        value: LiteralValue::String(key_expr.clone()),
                        span: crate::ast::Span::unknown(),
                    };
                    let (key_id, mut key_effects) =
                        Self::lower_value_ast(&key_literal, builder, phi_bindings)?;
                    effects.append(&mut key_effects);
                    let (value_id, mut value_effects) =
                        Self::lower_value_ast(value, builder, phi_bindings)?;
                    effects.append(&mut value_effects);
                    effects.push(CoreEffectPlan::MethodCall {
                        dst: None,
                        object: map_id,
                        method: "set".to_string(),
                        args: vec![key_id, value_id],
                        effects: EffectMask::PURE.add(Effect::Io),
                    });
                }
                Ok((map_id, effects))
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                if prelude_stmts.is_empty() {
                    return Self::lower_value_ast(tail_expr.as_ref(), builder, phi_bindings);
                }
                let (bindings, mut effects) = lower_blockexpr_value_prelude_stmts(
                    builder,
                    phi_bindings,
                    prelude_stmts,
                    "[normalizer] blockexpr value",
                )?;
                let (tail_id, mut tail_effects) =
                    Self::lower_value_ast(tail_expr.as_ref(), builder, &bindings)?;
                effects.append(&mut tail_effects);
                Ok((tail_id, effects))
            }
            ASTNode::BinaryOp { operator, .. } => match operator {
                crate::ast::BinaryOperator::Add
                | crate::ast::BinaryOperator::Subtract
                | crate::ast::BinaryOperator::Multiply
                | crate::ast::BinaryOperator::Divide
                | crate::ast::BinaryOperator::Modulo => {
                    let (lhs, op, rhs, mut consts) =
                        Self::lower_binop_ast(ast, builder, phi_bindings)?;
                    let dst = builder.alloc_typed(Self::arithmetic_result_type(builder, lhs, rhs));
                    consts.push(CoreEffectPlan::BinOp { dst, lhs, op, rhs });
                    Ok((dst, consts))
                }
                crate::ast::BinaryOperator::And
                | crate::ast::BinaryOperator::Or
                | crate::ast::BinaryOperator::Less
                | crate::ast::BinaryOperator::LessEqual
                | crate::ast::BinaryOperator::Greater
                | crate::ast::BinaryOperator::GreaterEqual
                | crate::ast::BinaryOperator::Equal
                | crate::ast::BinaryOperator::NotEqual => {
                    super::loop_body_lowering::lower_bool_expr(
                        builder,
                        phi_bindings,
                        ast,
                        "[normalizer] value bool binary op",
                    )
                }
                _ => Err(format!("[normalizer] Unsupported value AST: {:?}", ast)),
            },
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let Some(else_body) = else_body else {
                    return Err("[normalizer] value-if without else is unsupported".to_string());
                };
                if then_body.len() != 1 || else_body.len() != 1 {
                    return Err(
                        "[normalizer] value-if requires single-expression branches".to_string()
                    );
                }
                let then_expr = &then_body[0];
                let else_expr = &else_body[0];
                if !is_pure_value_expr(then_expr) || !is_pure_value_expr(else_expr) {
                    return Err("[normalizer] value-if requires pure expressions".to_string());
                }
                let cond_view = CondBlockView::from_expr(condition);
                let (cond_id, mut effects) = super::cond_lowering_entry::lower_bool_expr_value_id(
                    builder,
                    phi_bindings,
                    &cond_view,
                    "[normalizer] value-if",
                )?;
                let (then_id, mut then_effects) =
                    Self::lower_value_ast(then_expr, builder, phi_bindings)?;
                let (else_id, mut else_effects) =
                    Self::lower_value_ast(else_expr, builder, phi_bindings)?;
                effects.append(&mut then_effects);
                effects.append(&mut else_effects);
                let ty = builder
                    .type_ctx
                    .get_type(then_id)
                    .cloned()
                    .unwrap_or(MirType::Unknown);
                let result_id = builder.alloc_typed(ty);
                effects.push(CoreEffectPlan::Select {
                    dst: result_id,
                    cond: cond_id,
                    then_val: then_id,
                    else_val: else_id,
                });
                Ok((result_id, effects))
            }
            _ => Err(format!("[normalizer] Unsupported value AST: {:?}", ast)),
        }
    }

    fn record_newbox_metadata(builder: &mut MirBuilder, value_id: ValueId, class: &str) {
        let class_name = class.to_string();
        builder
            .type_ctx
            .value_types
            .insert(value_id, MirType::Box(class_name.clone()));
        builder
            .type_ctx
            .value_origin_newbox
            .insert(value_id, class_name.clone());
        builder
            .comp_ctx
            .type_registry
            .record_newbox(value_id, class_name.clone());
        builder
            .comp_ctx
            .type_registry
            .record_type(value_id, MirType::Box(class_name));
    }
}

#[cfg(test)]
mod tests {
    use super::super::PlanNormalizer;
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::MirBuilder;
    use crate::mir::{Effect, EffectMask, MirType, ValueId};
    use std::collections::BTreeMap;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn method_call(object: ASTNode, method: &str, arguments: Vec<ASTNode>) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(object),
            method: method.to_string(),
            arguments,
            span: Span::unknown(),
        }
    }

    #[test]
    fn lower_value_ast_keeps_nested_method_call_receiver_chain() {
        let mut builder = MirBuilder::new();
        let array_id = ValueId(1);
        let index_id = ValueId(2);
        builder
            .variable_ctx
            .variable_map
            .insert("arr".to_string(), array_id);
        builder
            .variable_ctx
            .variable_map
            .insert("idx".to_string(), index_id);
        builder
            .type_ctx
            .set_type(array_id, MirType::Box("RuntimeDataBox".to_string()));
        builder.type_ctx.set_type(index_id, MirType::Integer);

        let expr = method_call(
            method_call(var("arr"), "get", vec![var("idx")]),
            "length",
            vec![],
        );

        let (outer_result, effects) =
            PlanNormalizer::lower_value_ast(&expr, &mut builder, &BTreeMap::new())
                .expect("nested method call should lower");

        assert_eq!(
            effects.len(),
            2,
            "expected get + length effects, got {effects:?}"
        );

        let inner_result = match &effects[0] {
            super::CoreEffectPlan::MethodCall {
                dst: Some(dst),
                object,
                method,
                args,
                effects,
            } => {
                assert_eq!(*object, array_id, "get should stay on the array receiver");
                assert_eq!(method, "get");
                assert_eq!(args.as_slice(), &[index_id]);
                assert_eq!(*effects, EffectMask::PURE.add(Effect::Io));
                *dst
            }
            other => panic!("first effect must be inner get, got {:?}", other),
        };

        match &effects[1] {
            super::CoreEffectPlan::MethodCall {
                dst: Some(dst),
                object,
                method,
                args,
                effects,
            } => {
                assert_eq!(*dst, outer_result);
                assert_eq!(*object, inner_result, "length must receive the get result");
                assert_eq!(method, "length");
                assert!(args.is_empty());
                assert_eq!(*effects, EffectMask::PURE.add(Effect::Io));
            }
            other => panic!("second effect must be outer length, got {:?}", other),
        }
    }
}
