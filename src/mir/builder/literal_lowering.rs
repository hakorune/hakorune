//! Literal lowering and exact-numeric literal metadata.

use super::{MirBuilder, ValueId};
use crate::ast::LiteralValue;
use crate::mir::exact_numeric_value_facts::ExactNumericConstFact;
use crate::mir::numeric_substrate::{
    exact_numeric_const_from_i128, exact_numeric_mir_type_from_declared_name,
    ExactNumericConversionError, NumericTarget,
};

impl MirBuilder {
    /// Build a literal value.
    pub(in crate::mir::builder) fn build_literal(
        &mut self,
        literal: LiteralValue,
    ) -> Result<ValueId, String> {
        // Canonical Const emission publishes the transient type only after the
        // instruction succeeds. Literal dispatch must not duplicate that fact.
        Ok(match literal {
            LiteralValue::Integer(n) => {
                crate::mir::builder::emission::constant::emit_integer(self, n)?
            }
            LiteralValue::TypedInteger {
                value,
                declared_type_name,
            } => self.emit_typed_integer_literal(value, declared_type_name)?,
            LiteralValue::Float(f) => crate::mir::builder::emission::constant::emit_float(self, f)?,
            LiteralValue::String(s) => {
                crate::mir::builder::emission::constant::emit_string(self, s)?
            }
            LiteralValue::Bool(b) => crate::mir::builder::emission::constant::emit_bool(self, b)?,
            LiteralValue::Null => crate::mir::builder::emission::constant::emit_null(self)?,
            LiteralValue::Void => crate::mir::builder::emission::constant::emit_void(self)?,
        })
    }

    pub(in crate::mir::builder) fn emit_typed_integer_literal(
        &mut self,
        value: i64,
        declared_type_name: String,
    ) -> Result<ValueId, String> {
        let Some(ty) = exact_numeric_mir_type_from_declared_name(
            Some(declared_type_name.as_str()),
            NumericTarget::host(),
        ) else {
            return Err(format!(
                "[exact-numeric-literal/unknown-type] declared_type={}",
                declared_type_name
            ));
        };
        let checked = exact_numeric_const_from_i128(i128::from(value), &ty)
            .map_err(exact_numeric_literal_error)?;
        let dst = crate::mir::builder::emission::constant::emit_integer(self, value)?;
        if let Some(function) = self.function_state.current_function.as_mut() {
            function.metadata.exact_numeric_const_facts.insert(
                dst,
                ExactNumericConstFact {
                    declared_type_name: checked.ty.source_name,
                    value: checked.value,
                },
            );
        }
        Ok(dst)
    }
}

fn exact_numeric_literal_error(error: ExactNumericConversionError) -> String {
    match error {
        ExactNumericConversionError::NegativeToUnsigned { source_name, value } => format!(
            "[exact-numeric-literal/negative-unsigned] declared_type={} value={}",
            source_name, value
        ),
        ExactNumericConversionError::OutOfRange {
            source_name,
            value,
            min,
            max,
        } => format!(
            "[exact-numeric-literal/out-of-range] declared_type={} value={} range={}..{}",
            source_name, value, min, max
        ),
    }
}
