//! 🎯 箱理論: Special Method Handlers
//!
//! 責務: TypeOp functions, math functions, string normalization
//! - selected math function lowering
//! - build_str_normalization: str(x) → x.str() normalization

use super::super::{MirBuilder, MirInstruction, MirType, ValueId};
use super::special_handlers;
use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1,
};
use crate::mir::TypeOpKind;

pub(super) enum PreparedRawMathArgumentV1 {
    Direct(ASTNode),
    IntegerBoxToFloat(ASTNode),
}

impl PreparedRawMathArgumentV1 {
    fn prepare(argument: ASTNode) -> Self {
        let wrapper = match &argument {
            ASTNode::New {
                class, arguments, ..
            } if arguments.len() == 1 && class == "FloatBox" => Some(false),
            ASTNode::New {
                class, arguments, ..
            } if arguments.len() == 1 && class == "IntegerBox" => Some(true),
            _ => None,
        };
        let Some(cast_to_float) = wrapper else {
            return Self::Direct(argument);
        };
        let ASTNode::New { arguments, .. } = argument else {
            unreachable!("exact math wrapper projection must remain New");
        };
        let inner = arguments
            .into_iter()
            .next()
            .expect("exact math wrapper retains one argument");
        if cast_to_float {
            Self::IntegerBoxToFloat(inner)
        } else {
            Self::Direct(inner)
        }
    }
}

pub(super) fn prepare_raw_math_arguments_v1(
    arguments: Vec<ASTNode>,
) -> Vec<PreparedRawMathArgumentV1> {
    arguments
        .into_iter()
        .map(PreparedRawMathArgumentV1::prepare)
        .collect()
}

impl MirBuilder {
    /// Lower a math route selected before child effects.
    pub(super) fn lower_math_function_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        name: String,
        arguments: Vec<PreparedRawMathArgumentV1>,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        debug_assert!(special_handlers::is_math_function(&name));
        // Build numeric args directly for math.* to preserve f64 typing
        let mut math_args: Vec<ValueId> = Vec::new();
        for argument in arguments {
            match argument {
                PreparedRawMathArgumentV1::Direct(argument) => {
                    math_args.push(drive_legacy_expression_v1(self, port, argument)?);
                }
                PreparedRawMathArgumentV1::IntegerBoxToFloat(argument) => {
                    let iv = drive_legacy_expression_v1(self, port, argument)?;
                    let fv = self.next_value_id();
                    self.emit_instruction(MirInstruction::TypeOp {
                        dst: fv,
                        op: TypeOpKind::Cast,
                        value: iv,
                        ty: MirType::Float,
                    })?;
                    math_args.push(fv);
                }
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

#[cfg(test)]
mod tests {
    use super::{prepare_raw_math_arguments_v1, PreparedRawMathArgumentV1};
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::RecursiveChildLoweringPortV1;
    use crate::mir::{MirBuilder, ValueId};

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn new_box(class: &str, arguments: Vec<ASTNode>) -> ASTNode {
        ASTNode::New {
            class: class.to_owned(),
            type_arguments: Vec::new(),
            arguments,
            field_initializers: Vec::new(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn math_argument_recipes_preserve_exact_wrapper_boundary() {
        let prepared = prepare_raw_math_arguments_v1(vec![
            new_box("FloatBox", vec![integer(1)]),
            new_box("IntegerBox", vec![integer(2)]),
            new_box("FloatBox", vec![integer(3), integer(4)]),
            integer(5),
        ]);
        assert!(matches!(
            &prepared[0],
            PreparedRawMathArgumentV1::Direct(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                ..
            })
        ));
        assert!(matches!(
            &prepared[1],
            PreparedRawMathArgumentV1::IntegerBoxToFloat(ASTNode::Literal {
                value: LiteralValue::Integer(2),
                ..
            })
        ));
        assert!(matches!(
            &prepared[2],
            PreparedRawMathArgumentV1::Direct(ASTNode::New {
                class,
                arguments,
                ..
            }) if class == "FloatBox" && arguments.len() == 2
        ));
        assert!(matches!(
            &prepared[3],
            PreparedRawMathArgumentV1::Direct(ASTNode::Literal {
                value: LiteralValue::Integer(5),
                ..
            })
        ));
    }

    struct FailSecondPortV1 {
        expression_count: usize,
    }

    impl RecursiveChildLoweringPortV1 for FailSecondPortV1 {
        type BodyInput = Vec<ASTNode>;
        type StatementInput = ASTNode;
        type ExpressionInput = ASTNode;

        fn lower_body(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::BodyInput,
        ) -> Result<ValueId, String> {
            unreachable!("math argument descent requests expressions only")
        }

        fn lower_statement(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::StatementInput,
        ) -> Result<ValueId, String> {
            unreachable!("math argument descent requests expressions only")
        }

        fn lower_expression(
            &mut self,
            builder: &mut MirBuilder,
            _input: Self::ExpressionInput,
        ) -> Result<ValueId, String> {
            self.expression_count += 1;
            if self.expression_count == 2 {
                Err("second math argument failed".to_owned())
            } else {
                Ok(builder.next_value_id())
            }
        }
    }

    #[test]
    fn math_argument_failure_stops_suffix_and_mathbox_effects() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("math_argument_failure/0".to_owned());
        let mut port = FailSecondPortV1 {
            expression_count: 0,
        };
        let prepared = prepare_raw_math_arguments_v1(vec![integer(1), integer(2), integer(3)]);
        let error = builder
            .lower_math_function_with_port_v1(&mut port, "max".to_owned(), prepared)
            .unwrap_err();
        assert_eq!(error, "second math argument failed");
        assert_eq!(port.expression_count, 2);
        assert!(builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .is_empty());
    }
}
