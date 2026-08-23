use crate::ast::ASTNode;

use super::{source_seal, NyashParser, ParseError};

impl NyashParser {
    pub(super) fn open_postpass_product(
        &mut self,
        ast: ASTNode,
    ) -> Result<source_seal::OpenParserPostpassProductV1, ParseError> {
        let decisions = self.issue_build_gate_decision_set(&ast)?;
        let (callable_rows, prepared_static_box_sources) = self
            .callable_source_session
            .take()
            .ok_or_else(|| ParseError::GrammarContract {
                stable_reject_tag: "parser/direct-callable-source",
                detail: "direct callable source session was already moved into postpass".to_owned(),
                line: 0,
            })?
            .into_postpass_parts();
        Ok(source_seal::OpenParserPostpassProductV1::new(
            ast,
            std::mem::take(&mut self.prepared_source_seals),
            prepared_static_box_sources,
            self.take_source_build_gate_records(),
            callable_rows,
            self.take_metadata(),
            decisions,
        ))
    }
}
