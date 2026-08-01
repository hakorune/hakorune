use crate::ast::{ASTNode, EnumMatchArm, LiteralValue};
use crate::mir::builder::calls::drive_call_arguments_v1;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1,
};
use crate::mir::resolved_semantics::{
    admit_direct_enum_match_v1, EnumMatchAdmissionV1, EnumVariantAdmissionV1,
};
use crate::mir::{CompareOp, MirInstruction, MirType, ValueId};

pub(in crate::mir::builder) use super::enum_match_scopebox::{
    PreparedRawScopeBoxRouteV1, PreparedRawScopeBoxV1,
};

pub(in crate::mir::builder) struct PreparedRawEnumVariantHeaderV1 {
    tag: u32,
    declared_payload_type: Option<MirType>,
    _seal: PreparedRawEnumVariantHeaderSealV1,
}

impl PreparedRawEnumVariantHeaderV1 {
    pub(in crate::mir::builder) fn from_verified_admission_v1(
        admission: &EnumVariantAdmissionV1,
    ) -> Self {
        Self {
            tag: admission.tag(),
            declared_payload_type: payload_mir_type(admission.declared_payload_type_name()),
            _seal: PreparedRawEnumVariantHeaderSealV1,
        }
    }
}

struct PreparedRawEnumVariantHeaderSealV1;

pub(in crate::mir::builder) struct PreparedRawEnumMatchV1 {
    enum_name: String,
    scrutinee: ASTNode,
    route: PreparedRawEnumMatchRouteV1,
}

enum PreparedRawEnumMatchRouteV1 {
    PayloadProjection {
        variant_name: String,
        tag: u32,
        declared_payload_type_name: Option<String>,
    },
    BoolSelect {
        specs: Box<[(u32, bool)]>,
    },
}

struct ObservedRawEnumMatchArmV1 {
    variant_name: String,
    binding_present: bool,
    bool_value: Option<bool>,
}

impl PreparedRawEnumMatchV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &super::MirBuilder,
        enum_name: String,
        scrutinee: ASTNode,
        arms: Vec<EnumMatchArm>,
        else_expr: Option<Box<ASTNode>>,
    ) -> Result<Self, String> {
        if let Some(admission) = builder
            .comp_ctx
            .enum_decls
            .get(&enum_name)
            .and_then(|decl| {
                admit_direct_enum_match_v1(
                    &decl.type_parameters,
                    &decl.variants,
                    &arms,
                    else_expr.as_deref(),
                )
            })
        {
            return Ok(Self {
                enum_name,
                scrutinee,
                route: PreparedRawEnumMatchRouteV1::from_admission(admission),
            });
        }
        if else_expr.is_some() {
            return Err(format!(
                "[freeze:contract][mir_builder/enum_match] `{}` else-arm lowering is outside direct-MIR guard-let MVP",
                enum_name
            ));
        }

        let mut projection_shape = true;
        let mut projection_index = None;
        let mut observed = Vec::with_capacity(arms.len());
        for arm in arms {
            let target_projection = matches!(
                (&arm.binding_name, &arm.body),
                (Some(binding), ASTNode::Variable { name, .. }) if binding == name
            );
            let ignored_null = arm.binding_name.is_none() && null_literal_body(&arm.body);
            if target_projection {
                if projection_index.replace(observed.len()).is_some() {
                    projection_shape = false;
                }
            } else if !ignored_null {
                projection_shape = false;
            }
            observed.push(ObservedRawEnumMatchArmV1 {
                variant_name: arm.variant_name,
                binding_present: arm.binding_name.is_some(),
                bool_value: bool_literal_body(&arm.body),
            });
        }

        let route = if projection_shape {
            if let Some(target_index) = projection_index {
                builder.require_exhaustive_known_arm_names(&enum_name, &observed)?;
                let target = &observed[target_index];
                let resolved = builder.resolve_known_variant(&enum_name, &target.variant_name)?;
                if !resolved.decl.has_payload() || resolved.decl.requires_compat_payload_box() {
                    return Err(format!(
                        "[freeze:contract][mir_builder/enum_match] {}::{} payload projection requires a single scalar payload",
                        enum_name, target.variant_name
                    ));
                }
                PreparedRawEnumMatchRouteV1::PayloadProjection {
                    variant_name: target.variant_name.clone(),
                    tag: resolved.tag,
                    declared_payload_type_name: resolved.decl.payload_type_name.clone(),
                }
            } else {
                builder.prepare_raw_enum_match_bool_select_v1(&enum_name, &observed)?
            }
        } else {
            builder.prepare_raw_enum_match_bool_select_v1(&enum_name, &observed)?
        };
        Ok(Self {
            enum_name,
            scrutinee,
            route,
        })
    }
}

impl PreparedRawEnumMatchRouteV1 {
    fn from_admission(admission: EnumMatchAdmissionV1) -> Self {
        match admission {
            EnumMatchAdmissionV1::PayloadProjection {
                variant_name,
                tag,
                declared_payload_type_name,
            } => Self::PayloadProjection {
                variant_name: variant_name.into(),
                tag,
                declared_payload_type_name: declared_payload_type_name.map(Into::into),
            },
            EnumMatchAdmissionV1::BoolSelect { specs } => Self::BoolSelect { specs },
        }
    }
}

pub(in crate::mir::builder) fn prepare_raw_enum_variant_header_v1(
    builder: &super::MirBuilder,
    enum_name: &str,
    variant_name: &str,
    arguments: &[ASTNode],
) -> Result<Option<PreparedRawEnumVariantHeaderV1>, String> {
    let Some(resolved) = builder
        .comp_ctx
        .resolve_enum_variant(enum_name, variant_name)
    else {
        return Ok(None);
    };
    if resolved.decl.requires_compat_payload_box() {
        return Err(format!(
            "[freeze:contract][mir_builder/enum_ctor] {}::{} record/tuple payload lowering is outside direct-MIR MVP",
            enum_name, variant_name
        ));
    }
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
        && enum_variant_arguments_are_statically_nullish_v1(arguments)
    {
        return Err(crate::semantics::option_contract::nullish_payload_error(
            "mir_builder/enum_ctor",
        ));
    }
    Ok(Some(PreparedRawEnumVariantHeaderV1 {
        tag: resolved.tag,
        declared_payload_type: payload_mir_type(resolved.decl.payload_type_name.as_deref()),
        _seal: PreparedRawEnumVariantHeaderSealV1,
    }))
}

impl super::MirBuilder {
    pub(in crate::mir::builder) fn lower_prepared_raw_enum_match_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        prepared: PreparedRawEnumMatchV1,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        let PreparedRawEnumMatchV1 {
            enum_name,
            scrutinee,
            route,
        } = prepared;
        let scrutinee_value = drive_legacy_expression_v1(self, port, scrutinee)?;
        match route {
            PreparedRawEnumMatchRouteV1::PayloadProjection {
                variant_name,
                tag,
                declared_payload_type_name,
            } => {
                let payload_type =
                    payload_mir_type(declared_payload_type_name.as_deref()).or_else(|| {
                        self.function_state
                            .type_ctx
                            .value_types
                            .get(&scrutinee_value)
                            .and_then(|ty| concrete_enum_payload_type(&enum_name, ty))
                    });
                let dst = self.next_value_id();
                self.emit_instruction(MirInstruction::VariantProject {
                    dst,
                    value: scrutinee_value,
                    enum_name,
                    variant: variant_name,
                    tag,
                    payload_type: payload_type.clone(),
                })?;
                self.function_state
                    .type_ctx
                    .value_types
                    .insert(dst, payload_type.unwrap_or(MirType::Unknown));
                Ok(dst)
            }
            PreparedRawEnumMatchRouteV1::BoolSelect { specs } => {
                let tag_value = self.next_value_id();
                self.emit_instruction(MirInstruction::VariantTag {
                    dst: tag_value,
                    value: scrutinee_value,
                    enum_name,
                })?;
                self.function_state
                    .type_ctx
                    .value_types
                    .insert(tag_value, MirType::Integer);

                let mut specs = specs.into_vec().into_iter().rev();
                let (_, default_value) = specs
                    .next()
                    .expect("non-empty checked before reverse lowering");
                let mut result =
                    crate::mir::builder::emission::constant::emit_bool(self, default_value)?;
                for (tag, arm_value) in specs {
                    let tag_const = crate::mir::builder::emission::constant::emit_integer(
                        self,
                        i64::from(tag),
                    )?;
                    let cond = self.next_value_id();
                    crate::mir::builder::emission::compare::emit_to(
                        self,
                        cond,
                        CompareOp::Eq,
                        tag_value,
                        tag_const,
                    )?;
                    let then_val =
                        crate::mir::builder::emission::constant::emit_bool(self, arm_value)?;
                    let dst = self.next_value_id();
                    self.emit_instruction(MirInstruction::Select {
                        dst,
                        cond,
                        then_val,
                        else_val: result,
                    })?;
                    self.function_state
                        .type_ctx
                        .value_types
                        .insert(dst, MirType::Bool);
                    result = dst;
                }
                Ok(result)
            }
        }
    }

    /// Lower one prepared enum constructor while retaining the raw child port.
    pub(in crate::mir::builder) fn lower_prepared_raw_enum_variant_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        enum_name: String,
        variant_name: String,
        arguments: Vec<ASTNode>,
        prepared: PreparedRawEnumVariantHeaderV1,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        let PreparedRawEnumVariantHeaderV1 {
            tag,
            declared_payload_type,
            _seal: _,
        } = prepared;
        let arg_values = drive_call_arguments_v1(self, port, arguments.as_slice())?;
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
        let payload_type = declared_payload_type
            .or_else(|| {
                payload.and_then(|value| {
                    self.function_state
                        .type_ctx
                        .value_types
                        .get(&value)
                        .cloned()
                })
            })
            .or_else(|| {
                self.function_state
                    .current_function
                    .as_ref()
                    .and_then(|function| {
                        function
                            .metadata
                            .declared_return_type_name
                            .as_deref()
                            .and_then(|raw| concrete_enum_payload_type_name(&enum_name, raw))
                    })
            })
            .or_else(|| {
                self.function_state
                    .current_function
                    .as_ref()
                    .and_then(|function| {
                        concrete_enum_payload_type(&enum_name, &function.signature.return_type)
                    })
            });
        let dst = self.next_value_id();
        self.emit_instruction(MirInstruction::VariantMake {
            dst,
            enum_name: enum_name.clone(),
            variant: variant_name,
            tag,
            payload,
            payload_type,
        })?;
        self.function_state
            .type_ctx
            .value_types
            .insert(dst, MirType::Box(runtime_variant_box_name(&enum_name)));
        Ok(dst)
    }

    fn prepare_raw_enum_match_bool_select_v1(
        &self,
        enum_name: &str,
        arms: &[ObservedRawEnumMatchArmV1],
    ) -> Result<PreparedRawEnumMatchRouteV1, String> {
        if arms.is_empty() {
            return Err(format!(
                "[freeze:contract][mir_builder/enum_match] `{}` has no arms",
                enum_name
            ));
        }
        self.require_exhaustive_known_arm_names(enum_name, arms)?;
        let mut specs = Vec::with_capacity(arms.len());
        for arm in arms {
            if arm.binding_present {
                return Err(format!(
                    "[freeze:contract][mir_builder/enum_match] `{}` bool-select guard shape must not bind payloads",
                    enum_name
                ));
            }
            let Some(value) = arm.bool_value else {
                return Err(format!(
                    "[freeze:contract][mir_builder/enum_match] `{}` only guard-let boolean variant tests are accepted",
                    enum_name
                ));
            };
            let resolved = self.resolve_known_variant(enum_name, &arm.variant_name)?;
            specs.push((resolved.tag, value));
        }
        Ok(PreparedRawEnumMatchRouteV1::BoolSelect {
            specs: specs.into_boxed_slice(),
        })
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

    fn require_exhaustive_known_arm_names(
        &self,
        enum_name: &str,
        arms: &[ObservedRawEnumMatchArmV1],
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

fn null_literal_body(node: &ASTNode) -> bool {
    matches!(
        node,
        ASTNode::Literal {
            value: LiteralValue::Null,
            ..
        }
    )
}

fn payload_mir_type(raw: Option<&str>) -> Option<MirType> {
    let raw = raw?;
    if looks_like_generic_type_param(raw) {
        return None;
    }
    Some(super::MirBuilder::parse_type_name_to_mir(raw))
}

fn concrete_enum_payload_type(enum_name: &str, ty: &MirType) -> Option<MirType> {
    let MirType::Box(box_name) = ty else {
        return None;
    };
    concrete_enum_payload_type_name(enum_name, box_name)
}

fn concrete_enum_payload_type_name(enum_name: &str, raw: &str) -> Option<MirType> {
    let prefix = format!("{}<", enum_name);
    let payload = raw.strip_prefix(&prefix)?.strip_suffix('>')?.trim();
    if payload.is_empty() {
        return None;
    }
    payload_mir_type(Some(payload))
}

fn looks_like_generic_type_param(raw: &str) -> bool {
    !raw.is_empty()
        && raw
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
}

pub(in crate::mir) fn enum_variant_arguments_are_statically_nullish_v1(
    arguments: &[ASTNode],
) -> bool {
    arguments.iter().any(ast_is_statically_nullish)
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

#[cfg(test)]
mod raw_scopebox_route_tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::builder::recursive_child_lowering::RecursiveChildLoweringPortV1;
    use crate::mir::{BindingId, MirBuilder, ValueId};

    struct RecordingPort {
        body_calls: usize,
        statement_calls: usize,
        fail_statement: Option<usize>,
    }

    impl RecursiveChildLoweringPortV1 for RecordingPort {
        type BodyInput = Vec<ASTNode>;
        type StatementInput = ASTNode;
        type ExpressionInput = ASTNode;

        fn lower_body(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::BodyInput,
        ) -> Result<ValueId, String> {
            self.body_calls += 1;
            Ok(ValueId::new(90))
        }

        fn lower_statement(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::StatementInput,
        ) -> Result<ValueId, String> {
            self.statement_calls += 1;
            if self.fail_statement == Some(self.statement_calls) {
                return Err("scopebox child failure".to_string());
            }
            Ok(ValueId::new(self.statement_calls as u32))
        }

        fn lower_expression(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::ExpressionInput,
        ) -> Result<ValueId, String> {
            unreachable!("scopebox route test lowers statements or a body")
        }
    }

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn enum_match(temp_name: &str) -> ASTNode {
        ASTNode::EnumMatchExpr {
            enum_name: "Result".to_string(),
            scrutinee: Box::new(variable(temp_name)),
            arms: vec![],
            else_expr: None,
            span: Span::unknown(),
        }
    }

    fn local(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(value))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        }
    }

    fn guard_let_body(temp_name: &str) -> Vec<ASTNode> {
        vec![
            local(
                temp_name,
                ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                },
            ),
            ASTNode::If {
                condition: Box::new(enum_match(temp_name)),
                then_body: vec![],
                else_body: None,
                span: Span::unknown(),
            },
            local("value", enum_match(temp_name)),
        ]
    }

    #[test]
    fn raw_scopebox_route_is_disjoint_before_lowering() {
        let temp_name = "__ny_guard_let_subject_0";
        let guard = PreparedRawScopeBoxV1::prepare(guard_let_body(temp_name));
        assert!(matches!(
            guard.route,
            PreparedRawScopeBoxRouteV1::GuardLet { .. }
        ));

        let ordinary = PreparedRawScopeBoxV1::prepare(vec![local(
            "value",
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            },
        )]);
        assert!(matches!(
            ordinary.route,
            PreparedRawScopeBoxRouteV1::Ordinary { .. }
        ));

        let mut builder = MirBuilder::new();
        let mut port = RecordingPort {
            body_calls: 0,
            statement_calls: 0,
            fail_statement: None,
        };
        let ordinary = PreparedRawScopeBoxV1::prepare(vec![local(
            "value",
            ASTNode::Literal {
                value: LiteralValue::Integer(2),
                span: Span::unknown(),
            },
        )]);
        let result = builder
            .lower_prepared_raw_scopebox_with_port_v1(&mut port, ordinary)
            .expect("ordinary ScopeBox must use the body terminal");
        assert_eq!(result, ValueId::new(90));
        assert_eq!(port.body_calls, 1);
        assert_eq!(port.statement_calls, 0);
    }

    #[test]
    fn guard_let_failure_never_retries_ordinary_or_runs_success_cleanup() {
        let temp_name = "__ny_guard_let_subject_0";
        let mut builder = MirBuilder::new();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(temp_name.to_string(), ValueId::new(7));
        builder
            .function_state
            .binding_ctx
            .insert(temp_name.to_string(), BindingId::new(7));
        let mut port = RecordingPort {
            body_calls: 0,
            statement_calls: 0,
            fail_statement: Some(2),
        };

        let error = builder
            .lower_prepared_raw_scopebox_with_port_v1(
                &mut port,
                PreparedRawScopeBoxV1::prepare(guard_let_body(temp_name)),
            )
            .expect_err("selected guard-let child must fail");

        assert_eq!(error, "scopebox child failure");
        assert_eq!(port.statement_calls, 2);
        assert_eq!(port.body_calls, 0, "ordinary body route must not retry");
        assert!(builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key(temp_name));
        assert!(builder.function_state.binding_ctx.contains(temp_name));
    }

    #[test]
    fn guard_let_success_lowers_once_then_removes_only_the_temp_binding() {
        let temp_name = "__ny_guard_let_subject_0";
        let mut builder = MirBuilder::new();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(temp_name.to_string(), ValueId::new(7));
        builder
            .function_state
            .binding_ctx
            .insert(temp_name.to_string(), BindingId::new(7));
        let mut port = RecordingPort {
            body_calls: 0,
            statement_calls: 0,
            fail_statement: None,
        };

        let result = builder
            .lower_prepared_raw_scopebox_with_port_v1(
                &mut port,
                PreparedRawScopeBoxV1::prepare(guard_let_body(temp_name)),
            )
            .expect("selected guard-let route must lower");

        assert_eq!(result, ValueId::new(3));
        assert_eq!(port.statement_calls, 3);
        assert_eq!(port.body_calls, 0);
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key(temp_name));
        assert!(!builder.function_state.binding_ctx.contains(temp_name));
    }
}
