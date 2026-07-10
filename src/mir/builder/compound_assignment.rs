//! Evaluated-Place lowering for compound assignment.

use super::ValueId;
use crate::ast::{ASTNode, BinaryOperator};

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

impl super::MirBuilder {
    pub(super) fn build_compound_assignment_statement(
        &mut self,
        target: ASTNode,
        operator: BinaryOperator,
        rhs: ASTNode,
    ) -> Result<ValueId, String> {
        let place = self.evaluate_compound_place(target)?;
        let old = self.read_compound_place(&place)?;
        let rhs_value = self.build_expression(rhs)?;
        let new_value = self.build_binary_op_from_values(operator, old, rhs_value)?;
        self.write_compound_place(place, new_value)
    }

    fn evaluate_compound_place(&mut self, target: ASTNode) -> Result<EvaluatedPlace, String> {
        match target {
            ASTNode::Variable { name, .. } => Ok(EvaluatedPlace::Local(name)),
            ASTNode::FieldAccess { object, field, .. } => {
                self.fail_if_record_field_assignment_target(&object, &field)?;
                let object_value = self.build_expression(*object)?;
                let base = self.local_field_base(object_value);
                Ok(EvaluatedPlace::Field { base, field })
            }
            ASTNode::Index { target, index, .. } => {
                let target_label = match target.as_ref() {
                    ASTNode::Variable { name, .. } => Some(name.clone()),
                    _ => None,
                };
                let base = self.build_expression(*target)?;
                self.preflight_compound_index_place(base)?;
                let index = self.build_expression(*index)?;
                Ok(EvaluatedPlace::Index {
                    base,
                    index,
                    target_label,
                })
            }
            _ => Err("Complex compound assignment targets not yet supported".to_string()),
        }
    }

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
