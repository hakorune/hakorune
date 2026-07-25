//! S3 exact Raw VM-reference execution.
//!
//! This lane consumes a published Raw owner, executes only the sealed Main
//! target, and converts the VM result into a source result before the shared
//! process projection.  It never discovers an entry from the module or
//! reconstructs a status from a VM value.

use super::raw_root_publication::RawPublishedInvocationV1;
use super::source_entry_result::{
    CanonicalProcessExitV1, ProcessExitProfileV1, ProcessExitProjectionV1, SealedSourceFaultV1,
    SourceEntryResultV1,
};
use super::source_entry_vm_reference::{
    RawVmReferenceRunReportV1, VmReferenceProcessOutcomeV1, VmSourceEntryDecodePlanV1,
};
use crate::backend::vm_types::{VMError, VMValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawVmReferenceActivationStageV1 {
    Target,
    DecodePlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawVmReferenceActivationErrorV1 {
    NonRawTarget,
    EntryTargetMismatch,
    DecodePlanUnavailable,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawVmReferenceActivationV1 {
    owner: RawPublishedInvocationV1,
    stage: RawVmReferenceActivationStageV1,
    error: RawVmReferenceActivationErrorV1,
}

impl RejectedRawVmReferenceActivationV1 {
    pub(in crate::mir) const fn stage(&self) -> RawVmReferenceActivationStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawVmReferenceActivationErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}

    pub(in crate::mir) fn into_public_string(self) -> String {
        let stage = match self.stage {
            RawVmReferenceActivationStageV1::Target => "target",
            RawVmReferenceActivationStageV1::DecodePlan => "decode-plan",
        };
        let detail = format!("{:?}", self.error);
        self.discard();
        format!("[raw-vm-reference/{stage}/rejected] {detail}")
    }
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedRawVmReferenceActivationV1 {
    published: RawPublishedInvocationV1,
    plan: VmSourceEntryDecodePlanV1,
}

impl RawPublishedInvocationV1 {
    pub(in crate::mir) fn prepare_vm_reference_activation(
        self,
    ) -> Result<PreparedRawVmReferenceActivationV1, RejectedRawVmReferenceActivationV1> {
        if !self.main_entry_target_matches()
            || !self.selected_entry().is_main_target()
            || self.selected_entry().arity() != 0
        {
            return Err(RejectedRawVmReferenceActivationV1 {
                owner: self,
                stage: RawVmReferenceActivationStageV1::Target,
                error: RawVmReferenceActivationErrorV1::EntryTargetMismatch,
            });
        }
        let plan = match self.vm_decode_plan() {
            Ok(plan) => plan,
            Err(()) => {
                return Err(RejectedRawVmReferenceActivationV1 {
                    owner: self,
                    stage: RawVmReferenceActivationStageV1::DecodePlan,
                    error: RawVmReferenceActivationErrorV1::DecodePlanUnavailable,
                })
            }
        };
        Ok(PreparedRawVmReferenceActivationV1 {
            published: self,
            plan,
        })
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RawVmReferenceExecutionReceiptV1;

#[derive(Debug)]
pub(in crate::mir) struct CompletedRawVmReferenceExecutionV1 {
    published: RawPublishedInvocationV1,
    source_result: SourceEntryResultV1,
    _receipt: RawVmReferenceExecutionReceiptV1,
}

impl PreparedRawVmReferenceActivationV1 {
    pub(in crate::mir) fn execute(self) -> CompletedRawVmReferenceExecutionV1 {
        let Self { published, plan } = self;
        let symbol = published.selected_entry().symbol().to_owned();
        let (published, execution) = published.execute_exact_vm_entry(&symbol);
        let source_result = match execution {
            Ok(value) => decode_vm_value(plan, value),
            Err(error) => vm_error_to_source_fault(error),
        };
        CompletedRawVmReferenceExecutionV1 {
            published,
            source_result,
            _receipt: RawVmReferenceExecutionReceiptV1,
        }
    }
}

impl CompletedRawVmReferenceExecutionV1 {
    pub(in crate::mir) fn complete_source_entry(
        self,
    ) -> Result<VmReferenceProcessOutcomeV1, RawVmReferenceActivationErrorV1> {
        let Self {
            published,
            source_result,
            ..
        } = self;
        let termination = ProcessExitProjectionV1::project_borrowed(
            &source_result,
            ProcessExitProfileV1::Canonical(CanonicalProcessExitV1::V1),
        )
        .map_err(|_| RawVmReferenceActivationErrorV1::NonRawTarget)?;
        // The published owner is consumed only here and remains inside the
        // process outcome until the terminal consumes that outcome by value.
        Ok(VmReferenceProcessOutcomeV1::from_raw_vm_reference(
            published,
            source_result,
            termination,
        ))
    }
}

impl super::MirCompiler {
    /// Explicit Raw VM-reference production entry.  It is available only in
    /// the VM-reference feature and never widens the general VM runner.
    pub fn run_raw_vm_reference(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
    ) -> Result<RawVmReferenceRunReportV1, String> {
        if self.builder.repl_mode {
            return Err("[raw-vm-reference/source-binding/repl-unsupported] NarrowV1".to_owned());
        }
        let published = self
            .compile_raw_published_v1(ast, source_file)
            .map_err(|rejected| rejected.into_public_string())?;
        let prepared = published
            .prepare_vm_reference_activation()
            .map_err(|rejected| rejected.into_public_string())?;
        let outcome = prepared.execute().complete_source_entry().map_err(|error| {
            format!("[raw-vm-reference/source-entry/rejected] {error:?}")
        })?;
        Ok(outcome.into_run_report())
    }
}

fn decode_vm_value(plan: VmSourceEntryDecodePlanV1, value: VMValue) -> SourceEntryResultV1 {
    match plan {
        VmSourceEntryDecodePlanV1::Unit {
            origin,
            requires_void,
        } => {
            if requires_void && !matches!(value, VMValue::Void) {
                return abi_mismatch("Void", vm_value_kind(&value));
            }
            if !requires_void && !is_supported_unit_payload(&value) {
                return abi_mismatch("unit-compatible VM value", vm_value_kind(&value));
            }
            SourceEntryResultV1::Unit(origin)
        }
        VmSourceEntryDecodePlanV1::Integer => match value {
            VMValue::Integer(value) => SourceEntryResultV1::Integer(value),
            other => abi_mismatch("Integer", vm_value_kind(&other)),
        },
        VmSourceEntryDecodePlanV1::Bool => match value {
            VMValue::Bool(value) => SourceEntryResultV1::Bool(value),
            other => abi_mismatch("Bool", vm_value_kind(&other)),
        },
        VmSourceEntryDecodePlanV1::Float => match value {
            VMValue::Float(value) => SourceEntryResultV1::Float(value),
            other => abi_mismatch("Float", vm_value_kind(&other)),
        },
        VmSourceEntryDecodePlanV1::String => match value {
            VMValue::String(value) => SourceEntryResultV1::String(value.into_boxed_str()),
            other => abi_mismatch("String", vm_value_kind(&other)),
        },
    }
}

fn is_supported_unit_payload(value: &VMValue) -> bool {
    matches!(
        value,
        VMValue::Integer(_)
            | VMValue::Float(_)
            | VMValue::Bool(_)
            | VMValue::String(_)
            | VMValue::Void
    )
}

fn vm_value_kind(value: &VMValue) -> &'static str {
    match value {
        VMValue::Integer(_) => "Integer",
        VMValue::ExactNumeric(_) => "ExactNumeric",
        VMValue::Float(_) => "Float",
        VMValue::Bool(_) => "Bool",
        VMValue::String(_) => "String",
        VMValue::Future(_) => "Future",
        VMValue::Void => "Void",
        VMValue::BoxRef(_) => "BoxRef",
        VMValue::WeakBox(_) => "WeakBox",
    }
}

fn abi_mismatch(expected: &'static str, actual: &'static str) -> SourceEntryResultV1 {
    SourceEntryResultV1::Fault(SealedSourceFaultV1::new(
        "vm-entry-result-abi-mismatch",
        format!("expected={}, actual={}", expected, actual).into_boxed_str(),
    ))
}

fn vm_error_to_source_fault(error: VMError) -> SourceEntryResultV1 {
    let (code, detail) = match &error {
        VMError::DivisionByZero => ("vm-division-by-zero", error.to_string()),
        VMError::StepBudgetExceeded { .. } => ("vm-step-budget-exceeded", error.to_string()),
        VMError::DuringFrameRestore { .. } => ("vm-frame-restore-failed", error.to_string()),
        VMError::FrameRestoreFailed { .. } => ("vm-frame-restore-failed", error.to_string()),
        VMError::InvalidValue(_) => ("vm-invalid-value", error.to_string()),
        VMError::InvalidInstruction(_) => ("vm-invalid-instruction", error.to_string()),
        VMError::InvalidBasicBlock(_) => ("vm-invalid-basic-block", error.to_string()),
        VMError::StackUnderflow => ("vm-stack-underflow", error.to_string()),
        VMError::TypeError(_) => ("vm-type-error", error.to_string()),
        VMError::TaskFailed(_) => ("vm-task-failed", error.to_string()),
        VMError::TaskCancelled(_) => ("vm-task-cancelled", error.to_string()),
    };
    SourceEntryResultV1::Fault(SealedSourceFaultV1::new(code, detail.into_boxed_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, Span};
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};

    static ENTRY_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    use crate::mir::compiler::source_entry_result::UnitOriginV1;

    fn empty_script() -> ASTNode {
        ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        }
    }

    fn literal_script(value: crate::ast::LiteralValue) -> ASTNode {
        ASTNode::Program {
            statements: vec![ASTNode::Literal {
                value,
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }
    }

    fn binary_script(
        left: crate::ast::LiteralValue,
        operator: BinaryOperator,
        right: crate::ast::LiteralValue,
    ) -> ASTNode {
        ASTNode::Program {
            statements: vec![ASTNode::BinaryOp {
                operator,
                left: Box::new(ASTNode::Literal {
                    value: left,
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: right,
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }
    }

    fn print_script() -> ASTNode {
        ASTNode::Program {
            statements: vec![ASTNode::Print {
                expression: Box::new(ASTNode::Literal {
                    value: crate::ast::LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }
    }

    fn assignment_script() -> ASTNode {
        ASTNode::Program {
            statements: vec![
                ASTNode::Local {
                    variables: vec!["x".into()],
                    initial_values: vec![Some(Box::new(ASTNode::Literal {
                        value: crate::ast::LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "x".into(),
                        span: Span::unknown(),
                    }),
                    value: Box::new(ASTNode::Literal {
                        value: crate::ast::LiteralValue::Integer(3),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        }
    }

    fn compound_assignment_script() -> ASTNode {
        ASTNode::Program {
            statements: vec![
                ASTNode::Local {
                    variables: vec!["x".into()],
                    initial_values: vec![Some(Box::new(ASTNode::Literal {
                        value: crate::ast::LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::CompoundAssignment {
                    target: Box::new(ASTNode::Variable {
                        name: "x".into(),
                        span: Span::unknown(),
                    }),
                    operator: BinaryOperator::Add,
                    value: Box::new(ASTNode::Literal {
                        value: crate::ast::LiteralValue::Integer(2),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        }
    }

    fn empty_app() -> ASTNode {
        let main = ASTNode::FunctionDeclaration {
            name: "main".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        };
        let mut methods = HashMap::new();
        methods.insert("main".into(), main);
        ASTNode::Program {
            statements: vec![ASTNode::BoxDeclaration {
                name: "Main".into(),
                fields: Vec::new(),
                field_decls: Vec::new(),
                public_fields: Vec::new(),
                private_fields: Vec::new(),
                methods,
                constructors: HashMap::new(),
                init_fields: Vec::new(),
                weak_fields: Vec::new(),
                delegates: Vec::new(),
                invariants: Vec::new(),
                transitions: Vec::new(),
                is_interface: false,
                is_record: false,
                extends: Vec::new(),
                implements: Vec::new(),
                type_parameters: Vec::new(),
                is_sync: false,
                is_static: true,
                static_init: None,
                attrs: DeclarationAttrs::default(),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }
    }

    #[test]
    fn unit_plan_does_not_promote_print_payload_to_a_value() {
        let result = decode_vm_value(
            VmSourceEntryDecodePlanV1::Unit {
                origin: UnitOriginV1::PrintStatement,
                requires_void: false,
            },
            VMValue::Integer(1),
        );
        assert!(matches!(
            result,
            SourceEntryResultV1::Unit(UnitOriginV1::PrintStatement)
        ));
    }

    #[test]
    fn integer_plan_preserves_exact_integer_value() {
        assert!(matches!(
            decode_vm_value(VmSourceEntryDecodePlanV1::Integer, VMValue::Integer(255)),
            SourceEntryResultV1::Integer(255)
        ));
    }

    #[test]
    fn synthetic_unit_requires_void_payload() {
        let result = decode_vm_value(
            VmSourceEntryDecodePlanV1::Unit {
                origin: UnitOriginV1::EmptyBody,
                requires_void: true,
            },
            VMValue::Integer(0),
        );
        assert!(matches!(result, SourceEntryResultV1::Fault(_)));
    }

    #[test]
    fn raw_vm_entry_executes_published_empty_script() {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        let report = compiler
            .run_raw_vm_reference(empty_script(), Some("raw-vm-empty.hako"))
            .expect("explicit Raw VM-reference entry should execute empty Script");
        assert_eq!(report.status_code(), 0);
        assert_eq!(report.diagnostic_tag(), None);
        assert!(compiler.builder.current_module.is_none());
    }

    #[test]
    fn raw_vm_entry_preserves_integer_process_status_boundaries() {
        for value in [0, 255] {
            let mut compiler = crate::mir::compiler::MirCompiler::new();
            let report = compiler
                .run_raw_vm_reference(
                    literal_script(crate::ast::LiteralValue::Integer(value)),
                    Some("raw-vm-integer.hako"),
                )
                .expect("integer Raw VM-reference entry should execute");
            assert_eq!(report.status_code(), value as u8);
            assert_eq!(report.diagnostic_tag(), None);
        }
    }

    #[test]
    fn raw_vm_entry_reports_out_of_range_integer_without_zero_fallback() {
        for value in [-1, 256] {
            let mut compiler = crate::mir::compiler::MirCompiler::new();
            let report = compiler
                .run_raw_vm_reference(
                    literal_script(crate::ast::LiteralValue::Integer(value)),
                    Some("raw-vm-range.hako"),
                )
                .expect("out-of-range result should be a typed process fault");
            assert_eq!(report.status_code(), 70);
            assert_eq!(
                report.diagnostic_tag(),
                Some("[process/exit-code-out-of-range]")
            );
        }
    }

    #[test]
    fn raw_vm_entry_reports_unsupported_process_result_kinds() {
        let cases = [
            (crate::ast::LiteralValue::Bool(true), "Bool"),
            (crate::ast::LiteralValue::Float(1.5), "Float"),
            (crate::ast::LiteralValue::String("raw".into()), "String"),
        ];
        for (value, _kind) in cases {
            let mut compiler = crate::mir::compiler::MirCompiler::new();
            let report = compiler
                .run_raw_vm_reference(literal_script(value), Some("raw-vm-kind.hako"))
                .expect("unsupported process result should remain a typed fault");
            assert_eq!(report.status_code(), 70);
            assert_eq!(
                report.diagnostic_tag(),
                Some("[process/unsupported-result]")
            );
        }
    }

    #[test]
    fn raw_vm_entry_keeps_print_statement_as_unit() {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        let report = compiler
            .run_raw_vm_reference(print_script(), Some("raw-vm-print.hako"))
            .expect("Print statement should execute as a Unit source result");
        assert_eq!(report.status_code(), 0);
        assert_eq!(report.diagnostic_tag(), None);
    }

    #[test]
    fn raw_vm_entry_keeps_assignment_statement_as_unit() {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        let report = compiler
            .run_raw_vm_reference(assignment_script(), Some("raw-vm-assignment.hako"))
            .expect("assignment statement should execute as Unit");
        assert_eq!(report.status_code(), 0);
        assert_eq!(report.diagnostic_tag(), None);
    }

    #[test]
    fn raw_vm_entry_keeps_compound_assignment_statement_as_unit() {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        let report = compiler
            .run_raw_vm_reference(
                compound_assignment_script(),
                Some("raw-vm-compound-assignment.hako"),
            )
            .expect("compound assignment statement should execute as Unit");
        assert_eq!(report.status_code(), 0);
        assert_eq!(report.diagnostic_tag(), None);
    }

    #[test]
    fn raw_vm_entry_executes_empty_app_main_without_symbol_discovery() {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        let report = compiler
            .run_raw_vm_reference(empty_app(), Some("raw-vm-app.hako"))
            .expect("empty App Main should execute through the sealed target");
        assert_eq!(report.status_code(), 0);
        assert_eq!(report.diagnostic_tag(), None);
    }

    #[test]
    fn raw_vm_entry_ignores_decoy_nyash_entry_environment() {
        let _lock = ENTRY_ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("entry environment lock");
        let previous = std::env::var_os("NYASH_ENTRY");
        std::env::set_var("NYASH_ENTRY", "Decoy.main/0");
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        let result = compiler.run_raw_vm_reference(empty_app(), Some("raw-vm-decoy.hako"));
        match previous {
            Some(value) => std::env::set_var("NYASH_ENTRY", value),
            None => std::env::remove_var("NYASH_ENTRY"),
        }
        let report = result.expect("sealed Main target must ignore NYASH_ENTRY");
        assert_eq!(report.status_code(), 0);
        assert_eq!(report.diagnostic_tag(), None);
    }

    #[test]
    fn raw_vm_entry_maps_division_fault_to_source_diagnostic() {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        let report = compiler
            .run_raw_vm_reference(
                binary_script(
                    crate::ast::LiteralValue::Integer(1),
                    BinaryOperator::Divide,
                    crate::ast::LiteralValue::Integer(0),
                ),
                Some("raw-vm-div-zero.hako"),
            )
            .expect("division fault should be a typed process fault");
        assert_eq!(report.status_code(), 70);
        assert_eq!(report.diagnostic_tag(), Some("[process/source-fault]"));
    }

    #[test]
    fn raw_vm_reference_reuses_compiler_after_success() {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        for source_file in ["raw-vm-reuse-1.hako", "raw-vm-reuse-2.hako"] {
            let report = compiler
                .run_raw_vm_reference(empty_script(), Some(source_file))
                .expect("fresh Raw VM-reference execution should remain reusable");
            assert_eq!(report.status_code(), 0);
            assert_eq!(report.diagnostic_tag(), None);
        }
    }

    #[test]
    fn raw_vm_reference_reuses_compiler_after_entry_rejection() {
        let mut compiler = crate::mir::compiler::MirCompiler::new();
        compiler.builder.repl_mode = true;
        assert!(compiler
            .run_raw_vm_reference(empty_script(), Some("raw-vm-rejected.hako"))
            .is_err());
        compiler.builder.repl_mode = false;
        let report = compiler
            .run_raw_vm_reference(empty_script(), Some("raw-vm-recovered.hako"))
            .expect("entry rejection must not poison the compiler");
        assert_eq!(report.status_code(), 0);
        assert_eq!(report.diagnostic_tag(), None);
    }
}
