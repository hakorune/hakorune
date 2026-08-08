//! Parser-issued build-gate source ledger.
//!
//! This is transport evidence only.  It does not select a branch or issue a
//! resolver-grade source seal.  B2 opens only `TopLevelItem`; member/body
//! gates remain outside this ledger until a separate source contract exists.

use super::source_path::SourceBuildGatePathV1;
use super::{NyashParser, ParseError};
use crate::ast::{BuildPredicate, Span};

pub(super) use super::source_path::PreparedBuildGateSourceRecordV1;
pub(super) use super::source_path::SourceBuildGateScopeV1;

impl NyashParser {
    pub(super) fn source_build_gate_scope(&self) -> SourceBuildGateScopeV1 {
        self.source_build_gate_scope
    }

    pub(super) fn set_source_build_gate_scope(
        &mut self,
        scope: SourceBuildGateScopeV1,
    ) -> SourceBuildGateScopeV1 {
        std::mem::replace(&mut self.source_build_gate_scope, scope)
    }

    pub(super) fn register_source_build_gate(
        &mut self,
        gate_id: super::source_authority::SourceBuildGateIdV1,
        gate_path: SourceBuildGatePathV1,
        predicate: BuildPredicate,
        span: Span,
    ) -> Result<(), ParseError> {
        if self.source_build_gate_scope != SourceBuildGateScopeV1::TopLevelItem {
            return Ok(());
        }
        if gate_path.brand() != &self.source_invocation_brand() {
            return Err(ParseError::BuildCfg {
                message: "foreign parser brand in build-gate source ledger".to_owned(),
                line: span.line,
            });
        }
        self.prepared_source_build_gate_records
            .push(PreparedBuildGateSourceRecordV1 {
                brand: self.source_invocation_brand(),
                gate_id,
                gate_path,
                scope: SourceBuildGateScopeV1::TopLevelItem,
                predicate,
                span,
            });
        Ok(())
    }

    pub(super) fn take_source_build_gate_records(
        &mut self,
    ) -> Vec<PreparedBuildGateSourceRecordV1> {
        std::mem::take(&mut self.prepared_source_build_gate_records)
    }
}
