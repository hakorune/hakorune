//! 🎯 箱理論: Special Method Handlers
//!
//! 責務: TypeOp functions, math functions, string normalization
//! - selected math function lowering
//! - build_str_normalization: str(x) → x.str() normalization

use super::super::{MirBuilder, MirInstruction, MirType, ValueId};
use super::special_handlers;
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1,
};
use crate::mir::TypeOpKind;

impl MirBuilder {
    /// Lower a math route selected before child effects.
    pub(in crate::mir::builder) fn lower_math_function_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        name: String,
        raw_args: Vec<ASTNode>,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        debug_assert!(special_handlers::is_math_function(&name));
        // Build numeric args directly for math.* to preserve f64 typing
        let mut math_args: Vec<ValueId> = Vec::new();
        for a in raw_args.into_iter() {
            match a {
                ASTNode::New {
                    class, arguments, ..
                } if class == "FloatBox" && arguments.len() == 1 => {
                    let inner = arguments
                        .into_iter()
                        .next()
                        .expect("FloatBox arity checked");
                    math_args.push(drive_legacy_expression_v1(self, port, inner)?);
                }
                ASTNode::New {
                    class, arguments, ..
                } if class == "IntegerBox" && arguments.len() == 1 => {
                    let inner = arguments
                        .into_iter()
                        .next()
                        .expect("IntegerBox arity checked");
                    let iv = drive_legacy_expression_v1(self, port, inner)?;
                    let fv = self.next_value_id();
                    self.emit_instruction(MirInstruction::TypeOp {
                        dst: fv,
                        op: TypeOpKind::Cast,
                        value: iv,
                        ty: MirType::Float,
                    })?;
                    math_args.push(fv);
                }
                ASTNode::Literal {
                    value: LiteralValue::Float(_),
                    ..
                } => math_args.push(drive_legacy_expression_v1(self, port, a)?),
                other => math_args.push(drive_legacy_expression_v1(self, port, other)?),
            }
        }
        // new MathBox()
        let math_recv = self.next_value_id();
        self.emit_constructor_call(math_recv, "MathBox".to_string(), vec![])?;
        self.function_state
            .type_ctx
            .value_origin_newbox
            .insert(math_recv, "MathBox".to_string());
        // birth()
        self.emit_method_call(None, math_recv, "birth".to_string(), vec![])?;
        // call method
        let dst = self.next_value_id();
        self.emit_method_call(Some(dst), math_recv, name, math_args)?;
        Ok(dst)
    }

    /// Build str(x) normalization to x.str()
    pub(super) fn build_str_normalization(&mut self, arg: ValueId) -> Result<ValueId, String> {
        let dst = self.next_value_id();
        // Use unified method emission; downstream rewrite will functionize as needed
        self.emit_method_call(Some(dst), arg, "str".to_string(), vec![])?;
        Ok(dst)
    }
}
