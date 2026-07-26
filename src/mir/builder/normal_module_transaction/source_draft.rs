//! Opaque verified source-Main draft retained by the normal transaction.

use crate::mir::compiler::normal_source_plan::VerifiedNormalMainThunkResultV1;
use crate::mir::{MirFunction, MirType};

use super::result_type::normal_main_result_mir_type;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum NormalMainSourceDraftErrorV1 {
    SymbolMismatch {
        expected: Box<str>,
        actual: Box<str>,
    },
    ArityMismatch {
        expected: usize,
        actual: usize,
    },
    ResultMismatch {
        expected: MirType,
        actual: MirType,
    },
}

#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedNormalMainSourceDraftV1 {
    draft: MirFunction,
    _seal: VerifiedNormalMainSourceDraftSealV1,
}

#[derive(Debug)]
struct VerifiedNormalMainSourceDraftSealV1;

impl VerifiedNormalMainSourceDraftV1 {
    pub(in crate::mir::builder) fn seal(
        draft: MirFunction,
        expected_symbol: &str,
        expected_arity: usize,
        result: VerifiedNormalMainThunkResultV1,
    ) -> Result<Self, (MirFunction, NormalMainSourceDraftErrorV1)> {
        if draft.signature.name != expected_symbol {
            let actual = draft.signature.name.clone().into_boxed_str();
            return Err((
                draft,
                NormalMainSourceDraftErrorV1::SymbolMismatch {
                    expected: expected_symbol.into(),
                    actual,
                },
            ));
        }
        let actual_arity = draft.signature.params.len();
        if actual_arity != expected_arity {
            return Err((
                draft,
                NormalMainSourceDraftErrorV1::ArityMismatch {
                    expected: expected_arity,
                    actual: actual_arity,
                },
            ));
        }
        let expected_type = normal_main_result_mir_type(result);
        if draft.signature.return_type != expected_type {
            let actual = draft.signature.return_type.clone();
            return Err((
                draft,
                NormalMainSourceDraftErrorV1::ResultMismatch {
                    expected: expected_type,
                    actual,
                },
            ));
        }
        Ok(Self {
            draft,
            _seal: VerifiedNormalMainSourceDraftSealV1,
        })
    }

    pub(in crate::mir::builder) fn draft(&self) -> &MirFunction {
        &self.draft
    }

    pub(in crate::mir::builder) fn into_draft(self) -> MirFunction {
        self.draft
    }
}
