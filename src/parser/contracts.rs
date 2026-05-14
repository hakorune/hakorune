use crate::ast::{ASTNode, ContractClause, ContractKind};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    pub(crate) fn parse_contract_clauses_until_body(
        &mut self,
    ) -> Result<Vec<ContractClause>, ParseError> {
        let mut clauses = Vec::new();
        loop {
            while self.match_token(&TokenType::NEWLINE) {
                self.advance();
            }
            let Some(kind) = self.current_contract_kind() else {
                break;
            };
            self.advance();
            let condition = self.parse_expression()?;
            clauses.push(ContractClause { kind, condition });
            if self.match_token(&TokenType::SEMICOLON) {
                self.advance();
            }
        }
        Ok(clauses)
    }

    pub(crate) fn try_parse_invariant_clause(&mut self) -> Result<Option<ASTNode>, ParseError> {
        while self.match_token(&TokenType::NEWLINE) {
            self.advance();
        }
        if !matches!(&self.current_token().token_type, TokenType::IDENTIFIER(name) if name == "invariant") {
            return Ok(None);
        }
        self.advance();
        let condition = self.parse_expression()?;
        if self.match_token(&TokenType::SEMICOLON) {
            self.advance();
        }
        Ok(Some(condition))
    }

    fn current_contract_kind(&self) -> Option<ContractKind> {
        match &self.current_token().token_type {
            TokenType::IDENTIFIER(name) if name == "requires" => Some(ContractKind::Requires),
            TokenType::IDENTIFIER(name) if name == "ensures" => Some(ContractKind::Ensures),
            _ => None,
        }
    }
}
