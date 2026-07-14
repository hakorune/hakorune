use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::BasicBlockId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum BindingSsaErrorV1 {
    ForeignBinding {
        expected: FunctionOwnerIdV1,
        actual: FunctionOwnerIdV1,
    },
    MissingDefinition {
        block: BasicBlockId,
        binding: BindingRefV1,
    },
    BlockSealedTwice {
        block: BasicBlockId,
    },
    WitnessBlockMismatch {
        expected: BasicBlockId,
        actual: BasicBlockId,
    },
    PhiOperation {
        operation: &'static str,
        detail: String,
    },
    DuringPhiCleanup {
        primary: Box<BindingSsaErrorV1>,
        cleanup_failures: Box<[String]>,
    },
    Poisoned,
    UnsealedAtFinish {
        blocks: Box<[BasicBlockId]>,
    },
    IncompleteAtFinish {
        count: usize,
    },
}

impl std::fmt::Display for BindingSsaErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ForeignBinding { expected, actual } => write!(
                formatter,
                "Binding SSA foreign binding owner: expected={expected:?}, actual={actual:?}"
            ),
            Self::MissingDefinition { block, binding } => write!(
                formatter,
                "Binding SSA missing definition at {block} for {binding:?}"
            ),
            Self::BlockSealedTwice { block } => {
                write!(formatter, "Binding SSA block {block} was sealed twice")
            }
            Self::WitnessBlockMismatch { expected, actual } => write!(
                formatter,
                "Binding SSA predecessor witness mismatch: expected={expected}, actual={actual}"
            ),
            Self::PhiOperation { operation, detail } => {
                write!(formatter, "Binding SSA PHI {operation} failed: {detail}")
            }
            Self::DuringPhiCleanup {
                primary,
                cleanup_failures,
            } => write!(
                formatter,
                "Binding SSA failed during PHI cleanup: primary=<{primary}> cleanup=[{}]",
                cleanup_failures.join(" | ")
            ),
            Self::Poisoned => formatter.write_str("Binding SSA instance is poisoned"),
            Self::UnsealedAtFinish { blocks } => {
                write!(
                    formatter,
                    "Binding SSA finish found open blocks: {blocks:?}"
                )
            }
            Self::IncompleteAtFinish { count } => {
                write!(
                    formatter,
                    "Binding SSA finish found {count} incomplete PHIs"
                )
            }
        }
    }
}

impl std::error::Error for BindingSsaErrorV1 {}
