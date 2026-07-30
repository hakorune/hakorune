//! Evaluated-Place lowering for compound assignment.

use super::ValueId;
use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1,
};

pub(in crate::mir::builder) struct PreparedRawCompoundAssignmentV1 {
    route: PreparedRawCompoundAssignmentRouteV1,
    operator: BinaryOperator,
    rhs: ASTNode,
}

enum PreparedRawCompoundAssignmentRouteV1 {
    Local {
        name: String,
    },
    Field {
        object: ASTNode,
        field: String,
    },
    Index {
        target: ASTNode,
        index: ASTNode,
        target_label: Option<String>,
    },
    Unsupported,
}

enum EvaluatedPlace {
    Local(String),
    Field {
        base: ValueId,
        field: String,
    },
    Index {
        base: ValueId,
        index: ValueId,
        target_label: Option<String>,
    },
}

impl PreparedRawCompoundAssignmentV1 {
    pub(in crate::mir::builder) fn prepare(
        target: ASTNode,
        operator: BinaryOperator,
        rhs: ASTNode,
    ) -> Self {
        let route = match target {
            ASTNode::Variable { name, .. } => PreparedRawCompoundAssignmentRouteV1::Local { name },
            ASTNode::FieldAccess { object, field, .. } => {
                PreparedRawCompoundAssignmentRouteV1::Field {
                    object: *object,
                    field,
                }
            }
            ASTNode::Index { target, index, .. } => {
                let target_label = match target.as_ref() {
                    ASTNode::Variable { name, .. } => Some(name.clone()),
                    _ => None,
                };
                PreparedRawCompoundAssignmentRouteV1::Index {
                    target: *target,
                    index: *index,
                    target_label,
                }
            }
            _ => PreparedRawCompoundAssignmentRouteV1::Unsupported,
        };
        Self {
            route,
            operator,
            rhs,
        }
    }
}

pub(in crate::mir::builder) fn lower_prepared_raw_compound_assignment_with_port_v1<Port>(
    builder: &mut super::MirBuilder,
    port: &mut Port,
    prepared: PreparedRawCompoundAssignmentV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1,
{
    let place = match prepared.route {
        PreparedRawCompoundAssignmentRouteV1::Local { name } => EvaluatedPlace::Local(name),
        PreparedRawCompoundAssignmentRouteV1::Field { object, field } => {
            builder.fail_if_record_field_assignment_target(&object, &field)?;
            let object_value = drive_legacy_expression_v1(builder, port, object)?;
            let base = builder.local_field_base(object_value);
            EvaluatedPlace::Field { base, field }
        }
        PreparedRawCompoundAssignmentRouteV1::Index {
            target,
            index,
            target_label,
        } => {
            let base = drive_legacy_expression_v1(builder, port, target)?;
            builder.preflight_compound_index_place(base)?;
            let index = drive_legacy_expression_v1(builder, port, index)?;
            EvaluatedPlace::Index {
                base,
                index,
                target_label,
            }
        }
        PreparedRawCompoundAssignmentRouteV1::Unsupported => {
            return Err("Complex compound assignment targets not yet supported".to_string());
        }
    };
    let old = builder.read_compound_place(&place)?;
    let rhs_value = drive_legacy_expression_v1(builder, port, prepared.rhs)?;
    let new_value = builder.build_binary_op_from_values(prepared.operator, old, rhs_value)?;
    builder.write_compound_place(place, new_value)
}

impl super::MirBuilder {
    fn preflight_compound_index_place(&self, base: ValueId) -> Result<(), String> {
        if self.current_fastmem_region().is_some() {
            return Ok(());
        }
        match self.infer_index_target_class(base).as_deref() {
            Some("ArrayBox") | Some("MapBox") => Ok(()),
            class_hint => Err(format!(
                "[semantic-kernel/compound-assignment/unsupported-index-place] index operator is only supported for Array/Map (found {})",
                class_hint.unwrap_or("unknown")
            )),
        }
    }

    fn read_compound_place(&mut self, place: &EvaluatedPlace) -> Result<ValueId, String> {
        match place {
            EvaluatedPlace::Local(name) => self.build_variable_access(name.clone()),
            EvaluatedPlace::Field { base, field } => {
                self.build_field_access_from_value(*base, field.clone())
            }
            EvaluatedPlace::Index {
                base,
                index,
                target_label,
            } => self.build_index_access_from_values(
                None,
                *base,
                *index,
                target_label.clone(),
                "compound_load",
                None,
            ),
        }
    }

    fn write_compound_place(
        &mut self,
        place: EvaluatedPlace,
        value: ValueId,
    ) -> Result<ValueId, String> {
        match place {
            EvaluatedPlace::Local(name) => self.build_assignment_from_value(name, value),
            EvaluatedPlace::Field { base, field } => self.build_field_assignment_from_value_id(
                self.current_fastmem_region(),
                base,
                field,
                value,
            ),
            EvaluatedPlace::Index {
                base,
                index,
                target_label,
            } => self.build_index_access_from_values(
                None,
                base,
                index,
                target_label,
                "compound_store",
                Some(value),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
    use crate::mir::{BindingId, MirBuilder, MirInstruction};

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_owned(),
            span: Span::unknown(),
        }
    }

    fn builder(name: &str) -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test(name.to_owned());
        builder
    }

    fn declare(builder: &mut MirBuilder, name: &str, value: ValueId) {
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(name.to_owned(), value);
        builder
            .function_state
            .binding_ctx
            .insert(name.to_owned(), BindingId::new(0));
    }

    fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
        builder
            .function_state
            .current_function
            .as_ref()
            .expect("compound-assignment raw function")
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter().cloned())
            .collect()
    }

    fn lower(
        builder: &mut MirBuilder,
        target: ASTNode,
        operator: BinaryOperator,
        rhs: ASTNode,
    ) -> Result<ValueId, String> {
        let prepared = PreparedRawCompoundAssignmentV1::prepare(target, operator, rhs);
        let mut port = RawLegacyChildLoweringPortV1;
        lower_prepared_raw_compound_assignment_with_port_v1(builder, &mut port, prepared)
    }

    fn route(prepared: PreparedRawCompoundAssignmentV1) -> &'static str {
        match prepared.route {
            PreparedRawCompoundAssignmentRouteV1::Local { .. } => "local",
            PreparedRawCompoundAssignmentRouteV1::Field { .. } => "field",
            PreparedRawCompoundAssignmentRouteV1::Index { .. } => "index",
            PreparedRawCompoundAssignmentRouteV1::Unsupported => "unsupported",
        }
    }

    #[test]
    fn source_target_partition_is_total_and_disjoint() {
        let field = ASTNode::FieldAccess {
            object: Box::new(variable("object")),
            field: "value".to_owned(),
            span: Span::unknown(),
        };
        let index = ASTNode::Index {
            target: Box::new(variable("items")),
            index: Box::new(integer(0)),
            span: Span::unknown(),
        };
        let routes = [
            route(PreparedRawCompoundAssignmentV1::prepare(
                variable("x"),
                BinaryOperator::Add,
                integer(1),
            )),
            route(PreparedRawCompoundAssignmentV1::prepare(
                field,
                BinaryOperator::Add,
                integer(1),
            )),
            route(PreparedRawCompoundAssignmentV1::prepare(
                index,
                BinaryOperator::Add,
                integer(1),
            )),
            route(PreparedRawCompoundAssignmentV1::prepare(
                integer(0),
                BinaryOperator::Add,
                integer(1),
            )),
        ];
        assert_eq!(routes, ["local", "field", "index", "unsupported"]);
    }

    #[test]
    fn unsupported_target_rejects_before_rhs_effects() {
        let mut builder = builder("compound_assignment_unsupported_target/0");

        let error = lower(
            &mut builder,
            integer(0),
            BinaryOperator::Add,
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(integer(40)),
                right: Box::new(integer(2)),
                span: Span::unknown(),
            },
        )
        .unwrap_err();

        assert_eq!(
            error,
            "Complex compound assignment targets not yet supported"
        );
        assert!(instructions(&builder).is_empty());
    }

    #[test]
    fn local_compound_assignment_preflights_and_reuses_after_rhs_failure() {
        let mut missing_target = builder("compound_assignment_missing_target/0");
        let error = lower(
            &mut missing_target,
            variable("missing"),
            BinaryOperator::Add,
            integer(99),
        )
        .unwrap_err();
        assert!(error.contains("Undefined variable: missing"));
        assert!(instructions(&missing_target).is_empty());

        let mut builder = builder("compound_assignment_failure_reuse/0");
        let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 9).unwrap();
        declare(&mut builder, "x", old);
        let before_rhs = instructions(&builder);

        let error = lower(
            &mut builder,
            variable("x"),
            BinaryOperator::Add,
            variable("missing_rhs"),
        )
        .unwrap_err();
        assert!(error.contains("Undefined variable: missing_rhs"));
        assert_eq!(instructions(&builder), before_rhs);
        assert_eq!(
            builder.function_state.variable_ctx.variable_map.get("x"),
            Some(&old)
        );

        let value = lower(&mut builder, variable("x"), BinaryOperator::Add, integer(4)).unwrap();
        assert_eq!(
            builder.function_state.variable_ctx.variable_map.get("x"),
            Some(&value)
        );
        assert_ne!(value, old);
        assert_eq!(
            instructions(&builder)
                .iter()
                .filter(|row| matches!(row, MirInstruction::BinOp { .. }))
                .count(),
            1
        );
        assert_eq!(
            instructions(&builder)
                .iter()
                .filter(|row| matches!(row, MirInstruction::ReleaseStrong { .. }))
                .count(),
            1
        );
    }
}
