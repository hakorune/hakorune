//! 🎯 箱理論: Special Method Handlers
//!
//! 責務: TypeOp functions, math functions, string normalization
//! - try_build_typeop_function: isType/asType function call handling
//! - try_handle_math_function: math.* function calls (sin/cos/abs/min/max)
//! - build_str_normalization: str(x) → x.str() normalization

use super::super::{MirBuilder, MirInstruction, MirType, ValueId};
use super::special_handlers;
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::TypeOpKind;

impl MirBuilder {
    /// Try build TypeOp function calls (isType, asType)
    pub(super) fn try_build_typeop_function(
        &mut self,
        name: &str,
        args: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        if (name == "isType" || name == "asType") && args.len() == 2 {
            if let Some(type_name) = special_handlers::extract_string_literal(&args[1]) {
                let val = self.build_expression(args[0].clone())?;
                let ty = special_handlers::parse_type_name_to_mir(&type_name);
                let dst = self.next_value_id();
                let op = if name == "isType" {
                    TypeOpKind::Check
                } else {
                    TypeOpKind::Cast
                };
                self.emit_instruction(MirInstruction::TypeOp {
                    dst,
                    op,
                    value: val,
                    ty,
                })?;
                return Ok(Some(dst));
            }
        }
        Ok(None)
    }

    /// Try handle math.* function in function-style (sin/cos/abs/min/max)
    pub(super) fn try_handle_math_function(
        &mut self,
        name: &str,
        raw_args: Vec<ASTNode>,
    ) -> Option<Result<ValueId, String>> {
        if !special_handlers::is_math_function(name) {
            return None;
        }
        // Build numeric args directly for math.* to preserve f64 typing
        let mut math_args: Vec<ValueId> = Vec::new();
        for a in raw_args.into_iter() {
            match a {
                ASTNode::New {
                    class, arguments, ..
                } if class == "FloatBox" && arguments.len() == 1 => {
                    match self.build_expression(arguments[0].clone()) {
                        v @ Ok(_) => math_args.push(v.unwrap()),
                        err @ Err(_) => return Some(err),
                    }
                }
                ASTNode::New {
                    class, arguments, ..
                } if class == "IntegerBox" && arguments.len() == 1 => {
                    let iv = match self.build_expression(arguments[0].clone()) {
                        Ok(v) => v,
                        Err(e) => return Some(Err(e)),
                    };
                    let fv = self.next_value_id();
                    if let Err(e) = self.emit_instruction(MirInstruction::TypeOp {
                        dst: fv,
                        op: TypeOpKind::Cast,
                        value: iv,
                        ty: MirType::Float,
                    }) {
                        return Some(Err(e));
                    }
                    math_args.push(fv);
                }
                ASTNode::Literal {
                    value: LiteralValue::Float(_),
                    ..
                } => match self.build_expression(a) {
                    v @ Ok(_) => math_args.push(v.unwrap()),
                    err @ Err(_) => return Some(err),
                },
                other => match self.build_expression(other) {
                    v @ Ok(_) => math_args.push(v.unwrap()),
                    err @ Err(_) => return Some(err),
                },
            }
        }
        // new MathBox()
        let math_recv = self.next_value_id();
        if let Err(e) = self.emit_constructor_call(math_recv, "MathBox".to_string(), vec![]) {
            return Some(Err(e));
        }
        self.type_ctx
            .value_origin_newbox
            .insert(math_recv, "MathBox".to_string());
        // birth()
        if let Err(e) = self.emit_method_call(None, math_recv, "birth".to_string(), vec![]) {
            return Some(Err(e));
        }
        // call method
        let dst = self.next_value_id();
        if let Err(e) = self.emit_method_call(Some(dst), math_recv, name.to_string(), math_args) {
            return Some(Err(e));
        }
        Some(Ok(dst))
    }

    /// Build str(x) normalization to x.str()
    pub(super) fn build_str_normalization(&mut self, arg: ValueId) -> Result<ValueId, String> {
        let dst = self.next_value_id();
        // Use unified method emission; downstream rewrite will functionize as needed
        self.emit_method_call(Some(dst), arg, "str".to_string(), vec![])?;
        Ok(dst)
    }
}
